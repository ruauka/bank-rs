pub mod account;
pub mod storage;
pub mod transaction;

use crate::invoker::grpc::account::{AccountInvoke, AccountInvoker};
use crate::invoker::grpc::storage::{StorageInvoke, StorageInvoker};
use crate::invoker::grpc::transaction::{TransactionInvoke, TransactionInvoker};
use crate::invoker::URL;
use proto::bank_client::BankClient;

pub mod proto {
    tonic::include_proto!("bank");
}

#[derive(Default)]
pub struct GRPCInvoker<A, T, S>
where
    A: AccountInvoker,
    T: TransactionInvoker,
    S: StorageInvoker,
{
    // счета
    pub account: A,
    // транзакции
    pub transaction: T,
    // БД
    pub storage: S,
}

impl GRPCInvoker<AccountInvoke, TransactionInvoke, StorageInvoke> {
    /// Конструктор.
    pub async fn new() -> Self {
        // Клиент для создания TCP соединения
        let client = BankClient::connect(URL).await.unwrap();

        Self {
            account: AccountInvoke::new(client.clone()),
            transaction: TransactionInvoke::new(client.clone()),
            storage: StorageInvoke::new(client),
        }
    }
}
