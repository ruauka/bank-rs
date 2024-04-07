use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

/// Статусы транзакции.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, ToSchema)]
pub enum Operation {
    Registration,
    Replenish,
    Withdraw,
    TransferIncrease,
    TransferDecrease,
}

/// to_string().to_lowercase() для значения в Json
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Структура транзакции.
#[derive(Debug, Default, Clone, Deserialize, Serialize, ToSchema)]
pub struct Transaction {
    pub id: u32,
    pub operation: String,
    pub previous: f64,
    pub delta: f64,
    pub current: f64,
}

impl Transaction {
    /// Конструктор транзакции.
    pub fn new(id: u32, operation: String, previous: f64, delta: f64, current: f64) -> Self {
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
    pub account_name: String,
    pub transaction_value: Option<f64>,
}

/// Структура ответа совершенной транзакции.
#[derive(Serialize, ToSchema)]
pub struct TransactionResponse {
    pub account_name: String,
    pub transaction_id: u32,
    pub balance: f64,
}

impl TransactionResponse {
    /// Конструктор ответа совершенной транзакции.
    pub fn new(account_name: String, transaction_id: u32, balance: f64) -> Self {
        Self {
            account_name,
            transaction_id,
            balance,
        }
    }
}

/// Структура для парсинга деталий перевода со счета на счет.
#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub struct TransferRequest {
    pub account_from: String,
    pub account_to: String,
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
            info: "successfully transfer".to_string(),
            details,
        }
    }
}
