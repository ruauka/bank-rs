use serde::Deserialize;

/// Статусы счета.
#[derive(Debug, Clone, Deserialize)]
pub enum Status {
    Opened,
    Closed,
}

/// Структура транзакции.
#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    pub id: u32,
    pub operation: String,
    pub previous: f64,
    pub delta: f64,
    pub current: f64,
}

/// Структура счета.
#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    pub id: u32,
    pub status: Status,
    pub balance: f64,
    pub transactions: Vec<Transaction>,
}
