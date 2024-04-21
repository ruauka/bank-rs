use crate::schemas::request::TransferRequest;
use serde::Deserialize;

/// Структура ответа совершенной транзакции.
#[derive(Debug, Deserialize)]
pub struct TransactionResponse {
    pub account_id: u32,
    pub transaction_id: u32,
    pub balance: f64,
}

/// Структура ответа успешного перевода.
#[derive(Debug, Deserialize)]
pub struct TransferResponse {
    pub info: String,
    pub details: TransferRequest,
}

/// Структура ответа баланса счета.
#[derive(Debug, Deserialize)]
pub struct BalanceResponse {
    pub balance: f64,
}
