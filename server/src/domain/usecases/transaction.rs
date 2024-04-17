use crate::adapters::storage::cache::Cache;
use crate::adapters::storage::Storages;
use crate::domain::entities::account::Account;
use crate::domain::entities::transaction::Transaction;
use crate::domain::errors::AppError;
use crate::domain::errors::AppError::{AccountNotExists, TransactionNotExists};
use std::sync::{Arc, RwLock};

/// Получение транзакции счета по id.
pub fn transaction<S: Storages>(
    storage: Arc<RwLock<S>>,
    account_id: u32,
    transaction_id: u32,
) -> Result<Transaction, AppError> {
    // проверка наличия счета
    if !storage.write().unwrap().db().check_key(account_id) {
        return Err(AccountNotExists(account_id.to_string()));
    }

    let mut binding = storage.write().unwrap();
    // получение счета
    let account: &Account = binding.db().get_account(account_id);
    // текущая транзакция
    let Some(tx) = account.transactions.get(transaction_id as usize) else {
        return Err(TransactionNotExists(
            account_id.to_string(),
            transaction_id.to_string(),
        ));
    };

    Ok(tx.clone())
}
