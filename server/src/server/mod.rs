mod cli;

use crate::adapter::router::grpc::proto::bank_server::BankServer;
use crate::adapter::router::grpc::BankService;
use crate::adapter::router::http::router;
use crate::adapter::storage::cache::PATH;
use crate::adapter::storage::{Storage, StorageState};
use crate::server::cli::Cli;
use axum::Router;
use clap::Parser;
use std::fs;
use std::sync::{Arc, RwLock};
use tokio::signal;
use tonic::transport::Server;
use tracing::info;

/// Основная функция. Инициализация и запуск сервиса.
pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    // cli-конфиг
    let cfg: Cli = Cli::parse();
    // включение трейсинга
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    // создание 'state' объекта
    let shared_state: Arc<RwLock<Storage>> = StorageState::default();
    // создание папки для backup.json
    fs::create_dir_all(PATH).expect("error occurred while creating backup folder");
    // хост и порт
    let address: String = format!("{}:{}", cfg.host, cfg.port);
    // старт сервиса http/gRPC
    match cfg.protocol.as_str() {
        "grpc" => grpc_start(shared_state, address).await,
        "http" => {
            http_start(shared_state, address).await;
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Страт gRPC сервера.
async fn grpc_start(
    state: StorageState,
    address: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // создание приложения
    let app = BankService { state };
    info!(
        "🚀 GRPC server started successfully. Listening on {}...",
        address
    );
    // запуск сервиса
    Server::builder()
        .add_service(BankServer::new(app))
        .serve(address.parse()?)
        .await?;

    Ok(())
}

/// Страт Http сервера.
async fn http_start(state: StorageState, address: String) {
    // создание роутера и регистрация хендлеров и swagger
    let router: Router = router(state).await;
    // tcp-движок
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    info!(
        "🚀 Http server started successfully. Listening on {}...",
        listener.local_addr().unwrap()
    );
    // запуск сервиса с graceful shutdown
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Graceful shutdown.
async fn shutdown_signal() {
    // сигнал "ctrl_c"
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    // сигнал terminate
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    // отслеживание всех сигналов завершения
    tokio::select! {
        _ = ctrl_c => { info!("Shutting down server...") },
        _ = terminate => { info!("Shutting down server...") },
    }
}
