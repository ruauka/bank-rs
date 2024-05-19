use crate::entities::Account;
use crate::invoker::URL;
use crate::schemas::{
    request::{ChangeBalanceRequest, TransferRequest},
    response::{BalanceResponse, TransactionResponse, TransferResponse},
};
use reqwest::{Client, Response};
use std::path::Path;

/// Структура объекта вызова для работы со счетами.
pub struct AccountInvoke {
    client: Client,
}

impl AccountInvoke {
    /// Конструктор.
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

/// Интерфейс работы со счетами.
#[trait_variant::make(IntFactory: Send)]
pub trait AccountInvoker {
    /// Создание счета.
    async fn create(&self) -> Result<TransactionResponse, Box<dyn std::error::Error>>;

    /// Пополнение счета.
    async fn replenish(
        &self,
        account_id: u32,
        transaction_value: f64,
    ) -> Result<TransactionResponse, Box<dyn std::error::Error>>;

    /// Снятие со счета.
    async fn withdraw(
        &self,
        account_id: u32,
        transaction_value: f64,
    ) -> Result<TransactionResponse, Box<dyn std::error::Error>>;

    /// Преевод с одного счета на другой.
    async fn transfer(
        &self,
        account_from: u32,
        account_to: u32,
        transfer_value: f64,
    ) -> Result<TransferResponse, Box<dyn std::error::Error>>;

    /// Запрос баланса счета.
    async fn balance(&self, account_id: u32)
        -> Result<BalanceResponse, Box<dyn std::error::Error>>;

    /// Получение всей истории счета.
    async fn account(&self, account_id: u32) -> Result<Account, Box<dyn std::error::Error>>;
}

impl AccountInvoker for AccountInvoke {
    async fn create(&self) -> Result<TransactionResponse, Box<dyn std::error::Error>> {
        // путь
        let path: String = Path::new(URL).join("account/new").display().to_string();
        // запрос
        let res: Response = self.client.post(path).send().await?;
        // парсинг ответа
        Ok(res.json::<TransactionResponse>().await?)
    }

    async fn replenish(
        &self,
        account_id: u32,
        transaction_value: f64,
    ) -> Result<TransactionResponse, Box<dyn std::error::Error>> {
        // req body
        let req: ChangeBalanceRequest = ChangeBalanceRequest::new(account_id, transaction_value);
        // путь
        let path: String = Path::new(URL)
            .join("account/replenish")
            .display()
            .to_string();
        // запрос
        let resp: Response = self.client.post(path).json(&req).send().await?;
        // парсинг ответа
        Ok(resp.json::<TransactionResponse>().await?)
    }

    async fn withdraw(
        &self,
        account_id: u32,
        transaction_value: f64,
    ) -> Result<TransactionResponse, Box<dyn std::error::Error>> {
        // req body
        let req: ChangeBalanceRequest = ChangeBalanceRequest::new(account_id, transaction_value);
        // путь
        let path: String = Path::new(URL)
            .join("account/withdraw")
            .display()
            .to_string();
        // запрос
        let resp: Response = self.client.post(path).json(&req).send().await?;
        // парсинг ответа
        Ok(resp.json::<TransactionResponse>().await?)
    }

    async fn transfer(
        &self,
        account_from: u32,
        account_to: u32,
        transfer_value: f64,
    ) -> Result<TransferResponse, Box<dyn std::error::Error>> {
        // req body
        let req: TransferRequest = TransferRequest::new(account_from, account_to, transfer_value);
        // путь
        let path: String = Path::new(URL)
            .join("account/transfer")
            .display()
            .to_string();
        // запрос
        let resp: Response = self.client.post(path).json(&req).send().await?;
        // парсинг ответа
        Ok(resp.json::<TransferResponse>().await?)
    }

    async fn balance(
        &self,
        account_id: u32,
    ) -> Result<BalanceResponse, Box<dyn std::error::Error>> {
        // путь
        let path: String = Path::new(URL)
            .join("account/balance")
            .join(account_id.to_string())
            .display()
            .to_string();
        // запрос
        let resp: Response = self.client.get(path).send().await?;
        // парсинг ответа
        Ok(resp.json::<BalanceResponse>().await?)
    }

    async fn account(&self, account_id: u32) -> Result<Account, Box<dyn std::error::Error>> {
        // путь
        let path: String = Path::new(URL)
            .join("account")
            .join(account_id.to_string())
            .display()
            .to_string();
        // запрос
        let resp: Response = self.client.get(path).send().await?;
        // парсинг ответа
        Ok(resp.json::<Account>().await?)
    }
}
