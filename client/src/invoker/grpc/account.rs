use crate::invoker::grpc::proto;
use crate::invoker::grpc::proto::bank_client::BankClient;
use tonic::transport::Channel;
use tonic::Response;

/// Структура объекта вызова для работы со счетами.
pub struct AccountInvoke {
    client: BankClient<Channel>,
}

impl AccountInvoke {
    /// Конструктор.
    pub fn new(client: BankClient<Channel>) -> Self {
        Self { client }
    }
}

/// Интерфейс работы со счетами.
#[trait_variant::make(Send)]
pub trait AccountInvoker {
    /// Создание счета.
    async fn create(
        &mut self,
    ) -> Result<Response<proto::TransactionResponse>, Box<dyn std::error::Error>>;

    /// Пополнение счета
    async fn replenish(
        &mut self,
        account_id: u32,
        transaction_value: f64,
    ) -> Result<Response<proto::TransactionResponse>, Box<dyn std::error::Error>>;

    /// Списание со счета.
    async fn withdraw(
        &mut self,
        account_id: u32,
        transaction_value: f64,
    ) -> Result<Response<proto::TransactionResponse>, Box<dyn std::error::Error>>;

    /// Перевод со счета на счет
    async fn transfer(
        &mut self,
        account_from: u32,
        account_to: u32,
        transfer_value: f64,
    ) -> Result<Response<proto::TransferResponse>, Box<dyn std::error::Error>>;

    /// Запрос баланса счета.
    async fn balance(
        &mut self,
        account_id: u32,
    ) -> Result<Response<proto::BalanceResponse>, Box<dyn std::error::Error>>;

    /// Получение счета.
    async fn account(
        &mut self,
        account_id: u32,
    ) -> Result<Response<proto::AccountResponse>, Box<dyn std::error::Error>>;
}

impl AccountInvoker for AccountInvoke {
    async fn create(
        &mut self,
    ) -> Result<Response<proto::TransactionResponse>, Box<dyn std::error::Error>> {
        // запрос
        let resp: Response<proto::TransactionResponse> = self.client.new_account(()).await?;
        // парсинг ответа
        Ok(resp)
    }

    async fn replenish(
        &mut self,
        account_id: u32,
        transaction_value: f64,
    ) -> Result<Response<proto::TransactionResponse>, Box<dyn std::error::Error>> {
        // req body
        let req: proto::TransactionRequest = proto::TransactionRequest {
            account_id,
            transaction_value: transaction_value as f32,
        };
        // запрос
        let resp: Response<proto::TransactionResponse> = self.client.replenish(req).await?;

        Ok(resp)
    }

    async fn withdraw(
        &mut self,
        account_id: u32,
        transaction_value: f64,
    ) -> Result<Response<proto::TransactionResponse>, Box<dyn std::error::Error>> {
        // req body
        let req: proto::TransactionRequest = proto::TransactionRequest {
            account_id,
            transaction_value: transaction_value as f32,
        };
        // запрос
        let resp: Response<proto::TransactionResponse> = self.client.withdraw(req).await?;

        Ok(resp)
    }

    async fn transfer(
        &mut self,
        account_from: u32,
        account_to: u32,
        transfer_value: f64,
    ) -> Result<Response<proto::TransferResponse>, Box<dyn std::error::Error>> {
        // req body
        let req: proto::TransferRequest = proto::TransferRequest {
            account_from,
            account_to,
            transfer_value: transfer_value as f32,
        };
        // запрос
        let resp: Response<proto::TransferResponse> = self.client.transfer(req).await?;

        Ok(resp)
    }

    async fn balance(
        &mut self,
        account_id: u32,
    ) -> Result<Response<proto::BalanceResponse>, Box<dyn std::error::Error>> {
        // req body
        let req: proto::AccountId = proto::AccountId { account_id };
        // запрос
        let resp: Response<proto::BalanceResponse> = self.client.balance(req).await?;

        Ok(resp)
    }

    async fn account(
        &mut self,
        account_id: u32,
    ) -> Result<Response<proto::AccountResponse>, Box<dyn std::error::Error>> {
        // req body
        let req: proto::AccountId = proto::AccountId { account_id };
        // запрос
        let resp: Response<proto::AccountResponse> = self.client.account(req).await?;

        Ok(resp)
    }
}
