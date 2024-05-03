use crate::entities::Account;
use crate::invoker::URL;
use reqwest::{Client, Response};
use std::collections::HashMap;
use std::path::Path;

/// Структура объекта вызова для работы с БД.
pub struct StorageInvoke {
    client: Client,
}

impl StorageInvoke {
    /// Конструктор.
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

/// Интерфейс работы с БД.
#[trait_variant::make(IntFactory: Send)]
pub trait StorageInvoker {
    /// Запрос всей БД (все счета и их транзакции).
    async fn history(&self) -> Result<HashMap<u32, Account>, Box<dyn std::error::Error>>;
    /// Backup БД из реплики.
    /// Реплика создается и хранится на стороне сервера (server/src/backup/backup.json).
    async fn backup(&self) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>;
}

impl StorageInvoker for StorageInvoke {
    async fn history(&self) -> Result<HashMap<u32, Account>, Box<dyn std::error::Error>> {
        // путь
        let path: String = Path::new(URL).join("storage/history").display().to_string();
        // запрос
        let res: Response = self.client.get(path).send().await?;
        // парсинг ответа
        Ok(res.json::<HashMap<u32, Account>>().await?)
    }

    async fn backup(&self) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        // путь
        let path: String = Path::new(URL).join("storage/backup").display().to_string();
        // запрос
        let res: Response = self.client.post(path).send().await?;
        // парсинг ответа
        Ok(res.json::<HashMap<String, String>>().await?)
    }
}
