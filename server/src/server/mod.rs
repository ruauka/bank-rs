use crate::adapters::router::router;
use crate::adapters::storage::storage::{AccountStorageImpl, PATH};
use crate::adapters::storage::{Storage, StorageState};
use axum::Router;
use std::fs;
use std::sync::{Arc, RwLock};
use tokio::signal;
use tracing::info;

/// Хост и порт.
const ADDRESS: &str = "127.0.0.1:8080";

/// Основная функция. Инициализация и запуск сервиса.
pub async fn execute() {
    // создание 'state' объекта
    let shared_state: Arc<RwLock<Storage>> = StorageState::default();
    // создание роутера и регистрация хендлеров и swagger
    let router: Router = router(shared_state).await;
    // создание папки для backup.json
    fs::create_dir_all(PATH).expect("error occurred while creating backup folder");
    // включение трейсинга
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    // tcp-движок
    let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();
    info!(
        "🚀 Server started successfully. Listening on {}...",
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
