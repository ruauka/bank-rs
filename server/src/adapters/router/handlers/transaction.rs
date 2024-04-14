use crate::adapters::storage::cache::Cache;
use crate::adapters::storage::StorageState;
use crate::domain::entities::account::Account;
use crate::domain::entities::transaction::Transaction;
use crate::domain::errors::AppError::{AccountExistsErr, TransactionExistsErr};
use crate::domain::errors::{AppError, Result};
use crate::domain::usecases;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};

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
{"error1": AccountExistsErr(String::from("account_№n")).to_string()},
{"error2": TransactionExistsErr(String::from("account №n"), String::from("transaction №n")).to_string()},
]
))))]
/// Получение транзакции по id
pub async fn transaction(
    State(state): State<StorageState>,
    Path((account_name, transaction_id)): Path<(String, u32)>,
) -> Result<Json<Transaction>, AppError> {
    // получение транзакции по id
    usecases::transaction::transaction(state, account_name, transaction_id).map(Json)
}
