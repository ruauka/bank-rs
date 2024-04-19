use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
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
    AccountNotExists(String),
    // транзакция не существует
    #[error("account: '{0}' has no transaction with id: '{1}'")]
    TransactionNotExists(String, String),
    // транзакция со значением 0
    #[error("forbid transaction with 0 or less")]
    ZeroValueTransaction,
    // на счете не хватает средств для проведения транзакция
    #[error("account balance less than operation value")]
    Overdraft,
    // транзакция самому себе
    #[error("forbid transaction to yourself")]
    SelfTransfer,
    // пустая бд
    #[error("empty database")]
    EmptyDb,
    // пустой файл backup.json
    #[error("empty backup.json")]
    EmptyBackupFile,
    // ошибка при сериализации backup.json в HashMap<String, Account>
    #[error("invalid backup.json")]
    InvalidBackupFile,
    // ошибка загрузки файла репликации backup.json
    #[error("backup load file error")]
    BackupLoadFile,
    // // остальные
    // #[error(transparent)]
    // Other(#[from] anyhow::Error),
}

/// Имплементация для Axum Response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            AppError::AccountNotExists(_) | AppError::TransactionNotExists(_, _) => {
                (StatusCode::NOT_FOUND, self.to_string())
            }
            AppError::ZeroValueTransaction
            | AppError::Overdraft
            | AppError::SelfTransfer
            | AppError::EmptyDb
            | AppError::EmptyBackupFile
            | AppError::InvalidBackupFile
            | AppError::BackupLoadFile => (StatusCode::BAD_REQUEST, self.to_string()),
            // AppError::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        let body = Json(json!({
            "error": err_msg,
        }));
        (status, body).into_response()
    }
}
