pub mod storage;

use crate::adapters::storage::storage::{AccountStorage, AccountStorageImpl};
use crate::domain::entities::account::Account;
use axum::Extension;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Тип 'state' сервиса.
pub type StorageState = Arc<RwLock<Storage>>;

/// Структура 'state' сервиса.
#[derive(Debug, Default)]
pub struct Storage {
    pub db: AccountStorageImpl,
}

/// Трейт для слоя usecases.
pub trait Storages {
    type AccountStorageImpl: AccountStorage;

    fn db(&mut self) -> &mut Self::AccountStorageImpl;
}

/// Имплементация Storages с &mut владением.
impl Storages for Storage {
    type AccountStorageImpl = AccountStorageImpl;

    fn db(&mut self) -> &mut Self::AccountStorageImpl {
        &mut self.db
    }
}
