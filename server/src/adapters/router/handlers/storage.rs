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
path = "/storage/history",
responses(
(status = 200, description = "Got all transactions successfully", body = HashMap<String, Account>),
(status = 400, description = "Empty db error", body = AppError, example = json!({"error": EmptyDbErr.to_string()}))
)
)]
/// Получение бд
pub async fn history(State(state): State<StorageState>) -> Result<Json<HashMap<String, Account>>> {
    // получение всех транзакций бд в разбивке по счетам
    let db: HashMap<String, Account> = match usecases::storage::history(state) {
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
path = "/storage/backup",
responses(
(status = 200, description = "Backup successfully", body = HashMap<String, String>, example = json!({"info": "successfully backup"})),
(status = 400, description = "Backup load file error", body = AppError, example = json!({"error": BackupLoadFileErr.to_string()}))
)
)]
/// Восстановление бд
pub async fn backup(State(state): State<StorageState>) -> Result<Json<HashMap<String, String>>> {
    // backup бд
    if let Err(err) = usecases::storage::backup(state) {
        return Err(err);
    };
    // 200
    Ok(Json(HashMap::from([(
        "info".to_string(),
        "successfully backup".to_string(),
    )])))
}
