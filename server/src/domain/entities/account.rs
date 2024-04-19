use crate::domain::entities::transaction::Transaction;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Статусы счета.
#[derive(Debug, Default, Clone, Deserialize, Serialize, ToSchema)]
pub enum Status {
    #[default]
    Opened,
    Closed,
}

/// Структура счета.
#[derive(Debug, Default, Clone, Deserialize, Serialize, ToSchema)]
pub struct Account {
    pub id: u32,
    pub status: Status,
    pub balance: f64,
    pub transactions: Vec<Transaction>,
}

impl Account {
    /// Открытие счета.
    pub fn new() -> Self {
        Default::default()
    }
}

/// Структура ответа баланса счета.
#[derive(Serialize, ToSchema)]
pub struct BalanceResponse {
    pub balance: f64,
}

impl BalanceResponse {
    /// Конструктор ответа баланса счета.
    pub fn new(balance: f64) -> Self {
        Self { balance }
    }
}
