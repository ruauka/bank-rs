pub mod handlers;
mod middleware;

use crate::adapter::router::http::handlers::account::{
    account, balance, new_account, replenish, transfer, withdraw,
};
use crate::adapter::router::http::handlers::storage::{backup, history};
use crate::adapter::router::http::handlers::transaction::transaction;
use crate::adapter::router::http::handlers::{account, storage, transaction};
use crate::adapter::router::http::middleware::middleware;
use crate::adapter::storage::Storage;
use crate::domain::entities::account::{Account, BalanceResponse, Status};
use crate::domain::entities::transaction::{
    Operation, Transaction, TransactionRequest, TransactionResponse,
};
use crate::domain::entities::transaction::{TransferRequest, TransferResponse};
use crate::domain::errors::AppError;
use axum::Router;
use axum::{
    middleware::{self as middle},
    routing::{get, post},
};
use std::sync::{Arc, RwLock};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
paths(
account::new_account,
account::replenish,
account::withdraw,
account::transfer,
account::balance,
account::account,
transaction::transaction,
storage::history,
storage::backup
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
        .nest("/account", account_registration(&shared_state))
        // хендлеры транзакций
        .nest("/transaction", transaction_registration(&shared_state))
        // хендлеры бд
        .nest("/storage", storage_registration(&shared_state))
        // swagger
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // .layer((
        //     TraceLayer::new_for_http()
        //         .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        //         .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        //     // Graceful shutdown
        //     TimeoutLayer::new(Duration::from_secs(5)),
        // ))
        // кастомный middleware
        .layer(middle::from_fn(middleware))
}

/// Регистрация хендлеров работы со счетом.
pub fn account_registration(shared_state: &Arc<RwLock<Storage>>) -> Router {
    Router::new()
        .route("/new", post(new_account))
        .route("/replenish", post(replenish))
        .route("/withdraw", post(withdraw))
        .route("/transfer", post(transfer))
        .route("/balance/:account", get(balance))
        .route("/:account", get(account))
        .with_state(Arc::clone(shared_state))
}

/// Регистрация хендлеров работы со транзакциями.
pub fn transaction_registration(shared_state: &Arc<RwLock<Storage>>) -> Router {
    Router::new()
        .route("/:account/:id", get(transaction))
        .with_state(Arc::clone(shared_state))
}

/// Регистрация хендлеров работы с БД.
pub fn storage_registration(shared_state: &Arc<RwLock<Storage>>) -> Router {
    Router::new()
        .route("/history", get(history))
        .route("/backup", post(backup))
        .with_state(Arc::clone(shared_state))
}
