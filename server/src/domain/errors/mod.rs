use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use utoipa::ToSchema;

/// Переназначение Result для ответов хендлеров.
pub type Result<T, E = AppError> = core::result::Result<T, E>;

/// Ошибки сервиса.
#[derive(Debug, Error, ToSchema)]
pub enum AppError {
    // #[schema(example = "Some error from enum 'Errors'")]
    // счет не существует
    #[error("account with name: '{0}' not found")]
    AccountExistsErr(String),
    // транзакция не существует
    #[error("account: '{0}' has no transaction with id: '{1}'")]
    TransactionExistsErr(String, String),
    // транзакция со значением 0
    #[error("forbid transaction with 0 or less")]
    ZeroValueTransactionErr,
    // на счете не хватает средств для проведения транзакция
    #[error("account balance less than operation value")]
    OverdraftErr,
    // транзакция самому себе
    #[error("forbid transaction to yourself")]
    SelfTransactionErr,
    // пустая бд
    #[error("empty database")]
    EmptyDbErr,
    // пустой файл backup.json
    #[error("empty backup.json")]
    EmptyReplicaFile,
    // ошибка при сериализации backup.json в HashMap<String, Account>
    #[error("invalid backup.json")]
    InvalidReplicaFile,
    // ошибка загрузки файла репликации backup.json
    #[error("backup load file error")]
    BackupLoadFileErr,
    // // остальные
    // #[error(transparent)]
    // Other(#[from] anyhow::Error),
}

/// Имплементация для Axum Response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            AppError::AccountExistsErr(_) | AppError::TransactionExistsErr(_, _) => {
                (StatusCode::NOT_FOUND, self.to_string())
            }
            AppError::ZeroValueTransactionErr
            | AppError::OverdraftErr
            | AppError::SelfTransactionErr
            | AppError::EmptyDbErr
            | AppError::EmptyReplicaFile
            | AppError::InvalidReplicaFile
            | AppError::BackupLoadFileErr => (StatusCode::BAD_REQUEST, self.to_string()),
            // AppError::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        let body = Json(json!({
            "error": err_msg,
        }));
        (status, body).into_response()
    }
}
