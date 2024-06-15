use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Статусы транзакции.
#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize, ToSchema)]
pub enum Operation {
    #[default]
    Registration,
    Replenish,
    Withdraw,
    TransferIncrease,
    TransferDecrease,
}

/// Структура транзакции.
#[derive(Debug, Default, Clone, Deserialize, Serialize, ToSchema)]
pub struct Transaction {
    pub id: u32,
    pub operation: Operation,
    pub previous: f64,
    pub delta: f64,
    pub current: f64,
}

impl Transaction {
    /// Конструктор транзакции.
    pub fn new(id: u32, operation: Operation, previous: f64, delta: f64, current: f64) -> Self {
        Self {
            id,
            operation,
            previous,
            delta,
            current,
        }
    }
}

/// Структура для парсинга деталий транзакции.
#[derive(Deserialize, ToSchema)]
pub struct TransactionRequest {
    pub account_id: u32,
    pub transaction_value: f64,
}

/// Структура ответа совершенной транзакции.
#[derive(Serialize, ToSchema)]
pub struct TransactionResponse {
    pub account_id: u32,
    pub transaction_id: u32,
    pub balance: f64,
}

impl TransactionResponse {
    /// Конструктор ответа совершенной транзакции.
    pub fn new(account_id: u32, transaction_id: u32, balance: f64) -> Self {
        Self {
            account_id,
            transaction_id,
            balance,
        }
    }
}

/// Структура для парсинга деталий перевода со счета на счет.
#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub struct TransferRequest {
    pub account_from: u32,
    pub account_to: u32,
    pub transfer_value: f64,
}

/// Структура ответа успешного перевода.
#[derive(Serialize, ToSchema)]
pub struct TransferResponse {
    pub info: String,
    pub details: TransferRequest,
}

impl TransferResponse {
    /// Конструктор ответа успешного перевода.
    pub fn new(details: TransferRequest) -> Self {
        Self {
            info: "successfully transfered".to_string(),
            details,
        }
    }
}
