use crate::domain::entities::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::fmt;
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
    pub name: String,
    pub status: Status,
    pub balance: f64,
    pub transactions: Vec<Transaction>,
}

/// Порядковый номер счета.
#[derive(Debug)]
struct AccountName(u32);

impl fmt::Display for AccountName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // название первого счета: account_1
        write!(f, "account_{}", self.0)?;
        Ok(())
    }
}

impl Account {
    /// Открытие счета.
    pub fn new(last_name: String) -> Self {
        let mut acc: Account = Default::default();

        // если db пустая то дефолтоное значение первого счета
        if last_name.is_empty() {
            acc.name = AccountName(1).to_string();
        } else {
            // берем число из названия счета - account_1 (cur_idx = 1)
            let cur_idx: u32 = last_name[8..].trim().parse::<u32>().unwrap();
            // создаем название счета: дефолтный account_ и число + 1
            acc.name = AccountName(cur_idx + 1).to_string();
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
