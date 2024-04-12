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
path = "/account/new",
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
    let tx: TransactionResponse = usecases::account::new_account(state);
    // 201
    Ok(Json(tx))
}

#[utoipa::path(
post,
path = "/account/replenish",
request_body = TransactionRequest,
responses(
(status = 200, description = "Account replenished successfully", body = TransactionResponse),
(status = 400, description = "Errors", body = AppError, example = json!(
[
{"error1": ZeroValueTransactionErr.to_string()},
{"error2": OverdraftErr.to_string()}
]
)),
(status = 404, description = "Account not found", body = AppError, example = json!(
{"error": AccountExistsErr(String::from("account_№n")).to_string()}
)),
))]
/// Пополнение счета
pub async fn replenish(
    State(state): State<StorageState>,
    Json(payload): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>> {
    // пополнение счета
    let tx: TransactionResponse = match usecases::account::change_acc_balance(
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
    Ok(Json(tx))
}

#[utoipa::path(
post,
path = "/account/withdraw",
request_body = TransactionRequest,
responses(
(status = 200, description = "Account withdrawed successfully", body = TransactionResponse),
(status = 400, description = "Errors", body = AppError, example = json ! (
[
{"error1": ZeroValueTransactionErr.to_string()},
{"error2": OverdraftErr.to_string()}
]
)),
(status = 404, description = "Account not found", body = AppError, example = json!(
{"error": AccountExistsErr(String::from("account_№n")).to_string()}
)),
))]
/// Списание со счета
pub async fn withdraw(
    State(state): State<StorageState>,
    Json(payload): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>> {
    // списание со счета
    let tx: TransactionResponse = match usecases::account::change_acc_balance(
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
    Ok(Json(tx))
}

#[utoipa::path(
post,
path = "/account/transfer",
request_body = TransferRequest,
responses(
(status = 200, description = "Transfered successfully", body = TransferResponse),
(status = 400, description = "Errors", body = AppError, example = json ! (
[
{"error1": ZeroValueTransactionErr.to_string()},
{"error2": SelfTransactionErr.to_string()},
{"error3": OverdraftErr.to_string()}
]
)),
(status = 404, description = "Account not found", body = AppError, example = json!(
{"error": AccountExistsErr(String::from("account_№n")).to_string()}
)),
))]
/// Перевод со счета на счет
pub async fn transfer(
    State(state): State<StorageState>,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<TransferResponse>> {
    // Перевод со счета на счет
    let tx: TransferResponse = match usecases::account::transfer(state, payload) {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };
    // 200
    Ok(Json(tx))
}

#[utoipa::path(
get,
path = "/account/balance/{account}",
params(
("account" = String, Path, description = "account name")
),
responses(
(status = 200, description = "Got balance successfully", body = BalanceResponse),
(status = 404, description = "Account not found", body = AppError, example = json!(
{"error": AccountExistsErr(String::from("account_№n")).to_string()})),
))]
/// Баланса счета
pub async fn balance(
    State(state): State<StorageState>,
    Path(account_name): Path<String>,
) -> Result<Json<BalanceResponse>> {
    // Баланса счета
    let balance: BalanceResponse = match usecases::account::balance(state, account_name) {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };
    // 200
    Ok(Json(balance))
}

#[utoipa::path(
get,
path = "/account/{account}",
params(
("account" = String, Path, description = "account name")
),
responses(
(status = 200, description = "Got account successfully", body = Account),
(status = 404, description = "Account not found", body = AppError, example = json!(
{"error": AccountExistsErr(String::from("account_№n")).to_string()})),
))]
/// Получение всех транзакций счета
pub async fn account(
    State(state): State<StorageState>,
    Path(account_name): Path<String>,
) -> Result<Json<Account>> {
    // получение счета
    let acc: Account = match usecases::account::account(state, account_name) {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };
    // 200
    Ok(Json(acc))
}
