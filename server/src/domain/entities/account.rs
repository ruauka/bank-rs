use crate::domain::entities::transaction::Transaction;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Название первого счета в мапе.
const DEFAULT_ACCOUNT_NAME: &str = "account";

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
    pub name: String,
    pub status: Status,
    pub balance: f64,
    pub transactions: Vec<Transaction>,
}

impl Account {
    /// Открытие счета.
    pub fn new(last_name: String) -> Self {
        let mut acc: Account = Default::default();

        // если db пустая то дефолтоное значение первого счета
        if last_name.is_empty() {
            acc.name = format!("{}_1", DEFAULT_ACCOUNT_NAME);
        } else {
            // берем число из названия счета - account_1 (cur_idx = 1)
            let cur_idx: u32 = last_name[8..].trim().parse::<u32>().unwrap();
            // создаем название счета: дефолтный account_ и число + 1
            acc.name = format!("{}_{}", DEFAULT_ACCOUNT_NAME, cur_idx + 1);
        }

        acc
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
