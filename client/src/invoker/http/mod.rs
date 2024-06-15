pub mod account;
pub mod storage;
pub mod transaction;

use crate::invoker::http::account::{AccountInvoke, AccountInvoker};
use crate::invoker::http::storage::{StorageInvoke, StorageInvoker};
use crate::invoker::http::transaction::{TransactionInvoke, TransactionInvoker};
use reqwest::Client;

/// Общая структура объектов вызова хендлеров сервера.
#[derive(Default)]
pub struct HttpInvoker<A, T, S>
where
    A: AccountInvoker,
    T: TransactionInvoker,
    S: StorageInvoker,
{
    // счета
    pub account: A,
    // транзакции
    pub transaction: T,
    // БД
    pub storage: S,
}

impl HttpInvoker<AccountInvoke, TransactionInvoke, StorageInvoke> {
    /// Конструктор.
    pub fn new() -> Self {
        // Клиент для создания TCP соединения
        let client: Client = Client::builder().build().unwrap();

        Self {
            account: AccountInvoke::new(client.clone()),
            transaction: TransactionInvoke::new(client.clone()),
            storage: StorageInvoke::new(client),
        }
    }
}
