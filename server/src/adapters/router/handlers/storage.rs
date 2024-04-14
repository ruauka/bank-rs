use crate::adapters::storage::StorageState;
use crate::domain::entities::account::Account;
use crate::domain::errors::AppError::{
    BackupLoadFileErr, EmptyDbErr, EmptyReplicaFile, InvalidReplicaFile,
};
use crate::domain::errors::{AppError, Result};
use crate::domain::usecases;
use axum::extract::State;
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
pub async fn history(
    State(state): State<StorageState>,
) -> Result<Json<HashMap<String, Account>>, AppError> {
    // получение всех транзакций бд в разбивке по счетам
    usecases::storage::history(state).map(Json)
}

#[utoipa::path(
post,
path = "/storage/backup",
responses(
(status = 200, description = "Backup successfully", body = HashMap<String, String>, example = json!({"info": "successfully backup"})),
(status = 400, description = "Backup load file errors", body = AppError, example = json ! (
[
{"error1": BackupLoadFileErr.to_string()},
{"error2": EmptyReplicaFile.to_string()},
{"error3": InvalidReplicaFile.to_string()}
]
)),
)
)]
/// Восстановление бд
pub async fn backup(
    State(state): State<StorageState>,
) -> Result<Json<HashMap<String, String>>, AppError> {
    // backup бд
    usecases::storage::backup(state)?;
    // 200
    Ok(Json(HashMap::from([(
        "info".to_string(),
        "successfully backup".to_string(),
    )])))
}
