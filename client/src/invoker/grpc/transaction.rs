use crate::invoker::grpc::proto;
use crate::invoker::grpc::proto::bank_client::BankClient;
use tonic::transport::Channel;
use tonic::Response;

/// Структура объекта вызова для работы с транзакциями.
pub struct TransactionInvoke {
    client: BankClient<Channel>,
}

impl TransactionInvoke {
    /// Конструктор.
    pub fn new(client: BankClient<Channel>) -> Self {
        Self { client }
    }
}

/// Интерфейс работы с транзакциями.
#[trait_variant::make(IntFactory: Send)]
pub trait TransactionInvoker {
    /// Получение транзакции по id.
    async fn transaction(
        &mut self,
        account_id: u32,
        transaction_id: u32,
    ) -> Result<Response<proto::Transaction>, Box<dyn std::error::Error>>;
}

impl TransactionInvoker for TransactionInvoke {
    async fn transaction(
        &mut self,
        account_id: u32,
        transaction_id: u32,
    ) -> Result<Response<proto::Transaction>, Box<dyn std::error::Error>> {
        // req body
        let req: proto::Trans = proto::Trans {
            account_id,
            transaction_id,
        };
        // запрос
        let resp: Response<proto::Transaction> = self.client.get_transaction(req).await?;

        Ok(resp)
    }
}
