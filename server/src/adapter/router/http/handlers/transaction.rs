use crate::adapter::storage::StorageState;
use crate::domain::entities::transaction::Transaction;
use crate::domain::errors::AppError::{AccountNotExists, TransactionNotExists};
use crate::domain::errors::{AppError, Result};
use crate::domain::usecases;
use axum::extract::{Path, State};
use axum::Json;

#[utoipa::path(
get,
path = "/transaction/{account}/{id}",
params(
("account" = String, Path, description = "account name"),
("id" = u32, Path, description = "transaction id")
),
responses(
(status = 200, description = "Got transaction successfully", body = [Transaction]),
(status = 404, description = "Errors", body = AppError, example = json ! (
[
{"error1": AccountNotExists(String::from("account_№n")).to_string()},
{"error2": TransactionNotExists(String::from("account №n"), String::from("transaction №n")).to_string()},
]
))))]
/// Получение транзакции по id
pub async fn transaction(
    State(state): State<StorageState>,
    Path((account_id, transaction_id)): Path<(u32, u32)>,
) -> Result<Json<Transaction>, AppError> {
    usecases::transaction::transaction(&state, account_id, transaction_id).map(Json)
}
