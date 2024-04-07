use crate::adapters::storage::{Storage, StorageState};
use crate::domain::entities::account::{Account, BalanceResponse};
use crate::domain::entities::transaction::{
    Operation,
    Operation::{Replenish, TransferDecrease, TransferIncrease, Withdraw},
    Transaction, TransactionRequest, TransactionResponse, TransferRequest, TransferResponse,
};
use crate::domain::errors::AppError::{
    AccountExistsErr, OverdraftErr, SelfTransactionErr, ZeroValueTransactionErr,
};
use crate::domain::errors::{AppError, Result};
use crate::domain::usecases;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

#[utoipa::path(
post,
path = "/new",
responses(
(status = 201, description = "Account created successfully", body = TransactionResponse),
)
)]
/// Создание нового счета
///
/// Хендлер создания нового счета. Счета имееют имя: account_ и число. Пример: account_1.
/// Имя счетов формируются автоматически. Имя первого счета: account_1.
pub async fn new_account(State(state): State<StorageState>) -> Result<Json<TransactionResponse>> {
    // новый счета
    let tr: TransactionResponse = usecases::account::new_account(state);
    // 201
    Ok(Json(tr))
}

#[utoipa::path(
post,
path = "/replenish",
request_body = TransactionRequest,
responses(
(status = 200, description = "Account replenished successfully", body = TransactionResponse),
(status = 400, description = "Errors", body = AppError, example = json ! (
[
{"error1": AccountExistsErr(String::from("account_№n")).to_string()},
{"error2": ZeroValueTransactionErr.to_string()},
{"error3": OverdraftErr.to_string()}
]
))))]
/// Пополнение счета
pub async fn replenish(
    State(state): State<StorageState>,
    Json(payload): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>> {
    // пополнение счета
    let tr: TransactionResponse = match usecases::account::change_acc_balance(
        &state,
        payload.transaction_value.unwrap(),
        &payload.account_name,
        Replenish,
    ) {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };
    // 200
    Ok(Json(tr))
}

#[utoipa::path(
post,
path = "/withdraw",
request_body = TransactionRequest,
responses(
(status = 200, description = "Account withdrawed successfully", body = TransactionResponse),
(status = 400, description = "Errors", body = AppError, example = json ! (
[
{"error1": AccountExistsErr(String::from("account_№n")).to_string()},
{"error2": ZeroValueTransactionErr.to_string()},
{"error3": OverdraftErr.to_string()}
]
))))]
/// Списание со счета
pub async fn withdraw(
    State(state): State<StorageState>,
    Json(payload): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>> {
    // списание со счета
    let tr: TransactionResponse = match usecases::account::change_acc_balance(
        &state,
        payload.transaction_value.unwrap(),
        &payload.account_name,
        Withdraw,
    ) {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };
    // 200
    Ok(Json(tr))
}

#[utoipa::path(
post,
path = "/transfer",
request_body = TransferRequest,
responses(
(status = 200, description = "Transfered successfully", body = TransferResponse),
(status = 400, description = "Errors", body = AppError, example = json ! (
[
{"error1": ZeroValueTransactionErr.to_string()},
{"error2": SelfTransactionErr.to_string()},
{"error3": AccountExistsErr(String::from("account_№n")).to_string()},
{"error4": OverdraftErr.to_string()}
]
))))]
/// Перевод со счета на счет
pub async fn transfer(
    State(state): State<StorageState>,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<TransferResponse>> {
    // Перевод со счета на счет
    let tr: TransferResponse = match usecases::account::transfer(state, payload) {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };
    // 200
    Ok(Json(tr))
}

#[utoipa::path(
get,
path = "/balance/{account}",
params(
("account" = String, Path, description = "account name")
),
responses(
(status = 200, description = "Got balance successfully", body = BalanceResponse),
(status = 400, description = "Empty db error", body = AppError, example = json ! (
{"error": AccountExistsErr(String::from("account_№n")).to_string()}))
)
)]
/// Баланса счета
pub async fn get_balance(
    State(state): State<StorageState>,
    Path(account_name): Path<String>,
) -> Result<Json<BalanceResponse>> {
    // Баланса счета
    let balance: BalanceResponse = match usecases::account::get_balance(state, account_name) {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };
    // 200
    Ok(Json(balance))
}
