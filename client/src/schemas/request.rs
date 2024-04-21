use serde::{Deserialize, Serialize};

/// Структура для парсинга ответа перевода со счета на счет.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRequest {
    pub account_from: u32,
    pub account_to: u32,
    pub transfer_value: f64,
}

impl TransferRequest {
    /// Конструктор.
    pub fn new(account_from: u32, account_to: u32, transfer_value: f64) -> Self {
        Self {
            account_from,
            account_to,
            transfer_value,
        }
    }
}

/// Структура запроса изменения баланма счета.
#[derive(Debug, Serialize)]
pub struct ChangeBalanceRequest {
    pub account_id: u32,
    pub transaction_value: f64,
}

impl ChangeBalanceRequest {
    /// Конструктор
    pub fn new(account_id: u32, transaction_value: f64) -> Self {
        Self {
            account_id,
            transaction_value,
        }
    }
}
