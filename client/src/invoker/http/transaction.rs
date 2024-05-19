use crate::entities::Transaction;
use crate::invoker::URL;
use reqwest::{Client, Response};
use std::path::Path;

/// Структура объекта вызова для работы с транзакциями.
pub struct TransactionInvoke {
    client: Client,
}

impl TransactionInvoke {
    /// Конструктор.
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

/// Интерфейс работы с транзакциями.
#[trait_variant::make(IntFactory: Send)]
pub trait TransactionInvoker {
    /// Получение транзакции по ID.
    async fn transaction(
        &self,
        account_id: u32,
        transaction_id: u32,
    ) -> Result<Transaction, Box<dyn std::error::Error>>;
}

impl TransactionInvoker for TransactionInvoke {
    async fn transaction(
        &self,
        account_id: u32,
        transaction_id: u32,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        // путь
        let path: String = Path::new(URL)
            .join("transaction")
            .join(account_id.to_string())
            .join(transaction_id.to_string())
            .display()
            .to_string();
        // запрос
        let resp: Response = self.client.get(path).send().await?;
        // парсинг ответа
        Ok(resp.json::<Transaction>().await?)
    }
}
