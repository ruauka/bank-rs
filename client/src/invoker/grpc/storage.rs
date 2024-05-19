use crate::invoker::grpc::proto;
use crate::invoker::grpc::proto::bank_client::BankClient;
use tonic::transport::Channel;
use tonic::Response;

/// Структура объекта вызова для работы с БД.
pub struct StorageInvoke {
    client: BankClient<Channel>,
}

impl StorageInvoke {
    /// Конструктор.
    pub fn new(client: BankClient<Channel>) -> Self {
        Self { client }
    }
}

/// Интерфейс работы с БД.
#[trait_variant::make(Send)]
pub trait StorageInvoker {
    /// Запрос всей БД (все счета и их транзакции).
    async fn history(
        &mut self,
    ) -> Result<Response<proto::HistoryResponse>, Box<dyn std::error::Error>>;
    /// Backup БД из реплики.
    /// Реплика создается и хранится на стороне сервера (server/src/backup/backup.json).
    async fn backup(&mut self) -> Result<Response<()>, Box<dyn std::error::Error>>;
}

impl StorageInvoker for StorageInvoke {
    async fn history(
        &mut self,
    ) -> Result<Response<proto::HistoryResponse>, Box<dyn std::error::Error>> {
        // запрос
        let resp: Response<proto::HistoryResponse> = self.client.history(()).await?;
        Ok(resp)
    }

    async fn backup(&mut self) -> Result<Response<()>, Box<dyn std::error::Error>> {
        // запрос
        let resp: Response<()> = self.client.backup(()).await?;
        Ok(resp)
    }
}
