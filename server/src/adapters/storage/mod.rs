pub mod cache;

use crate::adapters::storage::cache::{Cache, CacheImpl};
use std::sync::{Arc, RwLock};

/// Тип 'state' сервиса.
pub type StorageState = Arc<RwLock<Storage>>;

/// Структура 'state' сервиса.
#[derive(Debug, Default)]
pub struct Storage {
    pub db: CacheImpl,
}

/// Трейт для слоя usecases.
pub trait Storages {
    type CacheImpl: Cache;

    fn db(&mut self) -> &mut Self::CacheImpl;
}

/// Имплементация Storages с &mut владением.
impl Storages for Storage {
    type CacheImpl = CacheImpl;

    fn db(&mut self) -> &mut Self::CacheImpl {
        &mut self.db
    }
}
