use crate::adapters::storage::StorageState;
use crate::domain::entities::account::{Account, BalanceResponse};
use crate::domain::entities::transaction::{
    Operation::{Replenish, Withdraw},
    TransactionRequest, TransactionResponse, TransferRequest, TransferResponse,
};
use crate::domain::errors::AppError::{
    AccountNotExists, Overdraft, SelfTransfer, ZeroValueTransaction,
};
use crate::domain::errors::{AppError, Result};
use crate::domain::usecases;
use axum::extract::{Path, State};
use axum::Json;

#[utoipa::path(
post,
path = "/account/new",
responses(
(status = 200, description = "Account created successfully", body = TransactionResponse),
)
)]
/// Создание нового счета
///
/// Хендлер создания нового счета. Счета имееют имя: account_ и число. Пример: account_1.
/// Имя счетов формируются автоматически. Имя первого счета: account_1.
pub async fn new_account(State(state): State<StorageState>) -> Json<TransactionResponse> {
    Json(usecases::account::new_account(state))
}

#[utoipa::path(
post,
path = "/account/replenish",
request_body = TransactionRequest,
responses(
(status = 200, description = "Account replenished successfully", body = TransactionResponse),
(status = 400, description = "Errors", body = AppError, example = json!(
[
{"error1": ZeroValueTransaction.to_string()},
{"error2": Overdraft.to_string()}
]
)),
(status = 404, description = "Account not found", body = AppError, example = json!(
{"error": AccountNotExists(String::from("account_№n")).to_string()}
)),
))]
/// Пополнение счета
pub async fn replenish(
    State(state): State<StorageState>,
    Json(payload): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>, AppError> {
    let res: Result<Json<TransactionResponse>, AppError> = usecases::account::change_acc_balance(
        &state,
        payload.transaction_value.unwrap(),
        &payload.account_name,
        Replenish,
    )
    .map(Json);
    // для себя, запомнить
    // map применяет функцию (экстактор Json в данном случае) к OK значению из change_acc_balance

    Ok(res.unwrap())
}

#[utoipa::path(
post,
path = "/account/withdraw",
request_body = TransactionRequest,
responses(
(status = 200, description = "Account withdrawed successfully", body = TransactionResponse),
(status = 400, description = "Errors", body = AppError, example = json ! (
[
{"error1": ZeroValueTransaction.to_string()},
{"error2": Overdraft.to_string()}
]
)),
(status = 404, description = "Account not found", body = AppError, example = json!(
{"error": AccountNotExists(String::from("account_№n")).to_string()}
)),
))]
/// Списание со счета
pub async fn withdraw(
    State(state): State<StorageState>,
    Json(payload): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>, AppError> {
    usecases::account::change_acc_balance(
        &state,
        payload.transaction_value.unwrap(),
        &payload.account_name,
        Withdraw,
    )
    .map(Json)
}

#[utoipa::path(
post,
path = "/account/transfer",
request_body = TransferRequest,
responses(
(status = 200, description = "Transfered successfully", body = TransferResponse),
(status = 400, description = "Errors", body = AppError, example = json ! (
[
{"error1": ZeroValueTransaction.to_string()},
{"error2": SelfTransfer.to_string()},
{"error3": Overdraft.to_string()}
]
)),
(status = 404, description = "Account not found", body = AppError, example = json!(
{"error": AccountNotExists(String::from("account_№n")).to_string()}
)),
))]
/// Перевод со счета на счет
pub async fn transfer(
    State(state): State<StorageState>,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<TransferResponse>, AppError> {
    usecases::account::transfer(state, payload).map(Json)
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
{"error": AccountNotExists(String::from("account_№n")).to_string()})),
))]
/// Баланс счета
pub async fn balance(
    State(state): State<StorageState>,
    Path(account_name): Path<String>,
) -> Result<Json<BalanceResponse>, AppError> {
    usecases::account::balance(state, account_name).map(Json)
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
{"error": AccountNotExists(String::from("account_№n")).to_string()})),
))]
/// Получение счета
pub async fn account(
    State(state): State<StorageState>,
    Path(account_name): Path<String>,
) -> Result<Json<Account>, AppError> {
    usecases::account::account(state, account_name).map(Json)
}
