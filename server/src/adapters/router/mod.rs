pub mod handlers;

use crate::adapters::router::handlers::{account, storage, transaction};
use axum::{http::header::CONTENT_TYPE, Router};
use axum::{
    // routing::get,
    Extension,
    // Router,
};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::adapters::router::handlers::account::{
    get_balance, new_account, replenish, transfer, withdraw,
};
use crate::adapters::router::handlers::storage::{backup_execute, get_all_transactions};
use crate::adapters::router::handlers::transaction::transaction;
use crate::adapters::storage::storage::AccountStorageImpl;
use crate::adapters::storage::Storage;
use crate::domain::entities::account::{Account, BalanceResponse, Status};
use crate::domain::entities::transaction::{
    Operation, Transaction, TransactionRequest, TransactionResponse,
};
use crate::domain::entities::transaction::{TransferRequest, TransferResponse};
use crate::domain::errors::AppError;
use axum::routing::{get, post};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
paths(
account::new_account,
account::replenish,
account::withdraw,
account::transfer,
account::get_balance,
transaction::transaction,
storage::get_all_transactions,
storage::backup_execute
),
components(
schemas(Account, Status, TransactionResponse, Transaction, Operation, BalanceResponse,
TransactionRequest, AppError, TransferRequest, TransferResponse, Transaction)
),
tags(
(name = "Bank service", description = "The service emulates banking transactions workflow")
)
)]
pub struct ApiDoc;

/// Создание роутера и регистрация хендлеров.
pub async fn router(shared_state: Arc<RwLock<Storage>>) -> Router {
    Router::new()
        // хендлеры счета
        .merge(account_registration(&shared_state))
        // хендлеры транзакций
        .merge(transaction_registration(&shared_state))
        // хендлеры бд
        .merge(storage_registration(&shared_state))
        // swagger
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer((
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            // Graceful shutdown
            TimeoutLayer::new(Duration::from_secs(5)),
        ))
}

/// Регистрация хендлеров работы со счетом.
pub fn account_registration(shared_state: &Arc<RwLock<Storage>>) -> Router {
    Router::new()
        .route("/new", post(new_account))
        .route("/replenish", post(replenish))
        .route("/withdraw", post(withdraw))
        .route("/transfer", post(transfer))
        .route("/balance/:account", get(get_balance))
        .with_state(Arc::clone(shared_state))
}

/// Регистрация хендлеров работы со транзакциями.
pub fn transaction_registration(shared_state: &Arc<RwLock<Storage>>) -> Router {
    Router::new()
        .route("/transaction/:account/:id", get(transaction))
        .with_state(Arc::clone(shared_state))
}

/// Регистрация хендлеров работы с БД.
pub fn storage_registration(shared_state: &Arc<RwLock<Storage>>) -> Router {
    Router::new()
        .route("/history", get(get_all_transactions))
        .route("/backup", post(backup_execute))
        .with_state(Arc::clone(shared_state))
}
