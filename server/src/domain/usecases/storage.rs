use crate::adapters::storage::cache::Cache;
use crate::adapters::storage::Storages;
use crate::domain::entities::account::Account;
use crate::domain::errors::AppError;
use crate::domain::errors::AppError::EmptyDbErr;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Получение всех счетов.
pub fn history<S: Storages>(storage: Arc<RwLock<S>>) -> Result<HashMap<String, Account>, AppError> {
    // копия бд
    let db: HashMap<String, Account> = storage.write().unwrap().db().get_accounts().clone();
    // проверка на пустую бд
    if db.is_empty() {
        return Err(EmptyDbErr);
    }

    Ok(db)
}

/// Backup БД.
pub fn backup<S: Storages>(storage: Arc<RwLock<S>>) -> Result<(), AppError> {
    storage.write().unwrap().db().backup_load()
}
