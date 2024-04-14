use crate::adapters::storage::cache::Cache;
use crate::adapters::storage::Storages;
use crate::domain::entities::account::Account;
use crate::domain::entities::transaction::{Transaction, TransactionResponse};
use crate::domain::errors::AppError;
use crate::domain::errors::AppError::{AccountExistsErr, TransactionExistsErr};
use std::sync::{Arc, RwLock};

/// Получение транзакции счета по id.
pub fn transaction<S: Storages>(
    storage: Arc<RwLock<S>>,
    account_name: String,
    transaction_id: u32,
) -> Result<Transaction, AppError> {
    // проверка наличия счета
    if !storage.write().unwrap().db().check_key(&account_name) {
        return Err(AccountExistsErr(account_name));
    }

    let mut binding = storage.write().unwrap();
    // получение счета
    let account: &Account = binding.db().get_account(&account_name);
    // текущая транзакция
    let Some(tx) = account.transactions.get(transaction_id as usize) else {
        return Err(TransactionExistsErr(
            account_name.to_string(),
            transaction_id.to_string(),
        ));
    };

    Ok(tx.clone())
}
