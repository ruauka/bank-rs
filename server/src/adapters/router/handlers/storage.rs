use crate::adapters::storage::StorageState;
use crate::domain::entities::account::Account;
use crate::domain::errors::AppError::{AccountExistsErr, BackupLoadFileErr, EmptyDbErr};
use crate::domain::errors::Result;
use crate::domain::usecases;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::collections::HashMap;

#[utoipa::path(
get,
path = "/history/{account}",
params(
("account" = String, Path, description = "account name")
),
responses(
(status = 200, description = "Got account successfully", body = Account),
(status = 404, description = "Account not found", body = AppError, example = json!(
{"error": AccountExistsErr(String::from("account_№n")).to_string()})),
))]
/// Получение всех транзакций счета
pub async fn get_account_transactions(
    State(state): State<StorageState>,
    Path(account_name): Path<String>,
) -> Result<Json<Account>> {
    // получение счета
    let acc: Account = match usecases::storage::get_account_transactions(state, account_name) {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };
    // 200
    Ok(Json(acc))
}

#[utoipa::path(
get,
path = "/history",
responses(
(status = 200, description = "Got all transactions successfully", body = HashMap<String, Account>),
(status = 400, description = "Empty db error", body = AppError, example = json!({"error": EmptyDbErr.to_string()}))
)
)]
/// Получение бд
pub async fn get_all_transactions(
    State(state): State<StorageState>,
) -> Result<Json<HashMap<String, Account>>> {
    // получение всех транзакций бд в разбивке по счетам
    let db: HashMap<String, Account> = match usecases::storage::get_all_transactions(state) {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };
    // 200
    Ok(Json(db))
}

#[utoipa::path(
post,
path = "/backup",
responses(
(status = 200, description = "Backup successfully", body = HashMap<String, String>, example = json!({"info": "successfully backup"})),
(status = 400, description = "Backup load file error", body = AppError, example = json!({"error": BackupLoadFileErr.to_string()}))
)
)]
/// Восстановление бд
pub async fn backup_execute(
    State(state): State<StorageState>,
) -> Result<Json<HashMap<String, String>>> {
    // backup бд
    if let Err(err) = usecases::storage::backup_execute(state) {
        return Err(err);
    };
    // 200
    Ok(Json(HashMap::from([(
        "info".to_string(),
        "successfully backup".to_string(),
    )])))
}
