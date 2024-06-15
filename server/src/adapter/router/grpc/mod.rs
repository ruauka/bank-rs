use crate::adapter::router::grpc::proto::bank_server::Bank;
use crate::adapter::storage::StorageState;
use crate::domain::entities::account::{Account, BalanceResponse};
use crate::domain::entities::transaction::Operation::{Replenish, Withdraw};
use crate::domain::entities::transaction::{
    Transaction, TransactionResponse, TransferRequest, TransferResponse,
};
use crate::domain::usecases;
use std::collections::HashMap;
use tonic::{Request, Response, Status};

/// сгенерированный gRPC-код.
pub mod proto {
    tonic::include_proto!("bank");
}

/// gRPC приложение.
#[derive(Debug, Default)]
pub struct BankService {
    pub state: StorageState,
}

/// gRPC хендлеры.
#[tonic::async_trait]
impl Bank for BankService {
    /// Создание нового счета.
    async fn new_account(
        &self,
        _: Request<()>,
    ) -> Result<Response<proto::TransactionResponse>, Status> {
        // создание счета
        let tx: TransactionResponse = usecases::account::new_account(self.state.clone());
        // gRPC ответ
        let resp = proto::TransactionResponse {
            account_id: tx.account_id,
            transaction_id: tx.transaction_id,
            balance: tx.balance as f32,
        };

        Ok(Response::new(resp))
    }

    /// Пополнение счета.
    async fn replenish(
        &self,
        request: Request<proto::TransactionRequest>,
    ) -> Result<Response<proto::TransactionResponse>, Status> {
        // запрос
        let input: &proto::TransactionRequest = request.get_ref();
        // изменение баланса счета
        let tx: TransactionResponse = match usecases::account::change_acc_balance(
            &self.state,
            input.transaction_value as f64,
            input.account_id,
            Replenish,
        ) {
            Ok(tx) => tx,
            Err(err) => return Err(Status::aborted(err.to_string())),
        };
        // gRPC ответ
        let resp = proto::TransactionResponse {
            account_id: tx.account_id,
            transaction_id: tx.transaction_id,
            balance: tx.balance as f32,
        };

        Ok(Response::new(resp))
    }

    /// Списание со счета.
    async fn withdraw(
        &self,
        request: Request<proto::TransactionRequest>,
    ) -> Result<Response<proto::TransactionResponse>, Status> {
        // запрос
        let input: &proto::TransactionRequest = request.get_ref();
        // изменение баланса счета
        let tx: TransactionResponse = match usecases::account::change_acc_balance(
            &self.state,
            input.transaction_value as f64,
            input.account_id,
            Withdraw,
        ) {
            Ok(tx) => tx,
            Err(err) => return Err(Status::aborted(err.to_string())),
        };
        // gRPC ответ
        let resp = proto::TransactionResponse {
            account_id: tx.account_id,
            transaction_id: tx.transaction_id,
            balance: tx.balance as f32,
        };

        Ok(Response::new(resp))
    }

    /// Перевод со счета на счет.
    async fn transfer(
        &self,
        request: Request<proto::TransferRequest>,
    ) -> Result<Response<proto::TransferResponse>, Status> {
        // запрос
        let input: &proto::TransferRequest = request.get_ref();
        // перекладка в schema
        let req = TransferRequest {
            account_from: input.account_from,
            account_to: input.account_to,
            transfer_value: input.transfer_value as f64,
        };
        // перевод
        let tx: TransferResponse = match usecases::account::transfer(&self.state, req) {
            Ok(tx) => tx,
            Err(err) => return Err(Status::aborted(err.to_string())),
        };
        // gRPC ответ
        let resp = proto::TransferResponse {
            info: tx.info,
            details: Some(proto::TransferRequest {
                account_from: tx.details.account_from,
                account_to: tx.details.account_to,
                transfer_value: tx.details.transfer_value as f32,
            }),
        };

        Ok(Response::new(resp))
    }

    /// Баланс счета.
    async fn balance(
        &self,
        request: Request<proto::AccountId>,
    ) -> Result<Response<proto::BalanceResponse>, Status> {
        // запрос
        let input: &proto::AccountId = request.get_ref();
        // баланс
        let tx: BalanceResponse = match usecases::account::balance(&self.state, input.account_id) {
            Ok(tx) => tx,
            Err(err) => return Err(Status::aborted(err.to_string())),
        };
        // gRPC ответ
        let resp = proto::BalanceResponse {
            balance: tx.balance as f32,
        };

        Ok(Response::new(resp))
    }

    /// Получение счета.
    async fn account(
        &self,
        request: Request<proto::AccountId>,
    ) -> Result<Response<proto::AccountResponse>, Status> {
        // запрос
        let input: &proto::AccountId = request.get_ref();
        // баланс
        let tx: Account = match usecases::account::account(&self.state, input.account_id) {
            Ok(tx) => tx,
            Err(err) => return Err(Status::aborted(err.to_string())),
        };
        // вектор для переладки в proto::Transaction
        let mut grpc_tx_vec: Vec<proto::Transaction> = Vec::with_capacity(tx.transactions.len());
        // наполнение вектора
        for trans in tx.transactions {
            let t = proto::Transaction {
                id: trans.id,
                operation: trans.operation as i32,
                previous: trans.previous as f32,
                delta: trans.delta as f32,
                current: trans.current as f32,
            };
            grpc_tx_vec.push(t)
        }
        // gRPC ответ
        let resp = proto::AccountResponse {
            id: tx.id,
            status: tx.status as i32,
            balance: tx.balance as f32,
            transaction: grpc_tx_vec,
        };

        Ok(Response::new(resp))
    }

    /// Получение транзакции по id.
    async fn get_transaction(
        &self,
        request: Request<proto::Trans>,
    ) -> Result<Response<proto::Transaction>, Status> {
        // запрос
        let input: &proto::Trans = request.get_ref();
        // транзакция
        let tx: Transaction = match usecases::transaction::transaction(
            &self.state,
            input.account_id,
            input.transaction_id,
        ) {
            Ok(tx) => tx,
            Err(err) => return Err(Status::aborted(err.to_string())),
        };
        // gRPC ответ
        let resp = proto::Transaction {
            id: tx.id,
            operation: tx.operation as i32,
            previous: tx.previous as f32,
            delta: tx.delta as f32,
            current: tx.current as f32,
        };

        Ok(Response::new(resp))
    }

    /// получение всех транзакций бд в разбивке по счетам.
    async fn history(&self, _: Request<()>) -> Result<Response<proto::HistoryResponse>, Status> {
        let db = match usecases::storage::history(&self.state) {
            Ok(tx) => tx,
            Err(err) => return Err(Status::aborted(err.to_string())),
        };
        // мапа для перекладки в grpc формат
        let mut grpc_map = HashMap::<u32, proto::AccountResponse>::new();

        for (acc_id, acc_val) in db {
            // вектор для переладки в proto::Transaction
            let mut grpc_tx_vec: Vec<proto::Transaction> =
                Vec::with_capacity(acc_val.transactions.len());
            // наполнение вектора
            for trans in acc_val.transactions {
                let t = proto::Transaction {
                    id: trans.id,
                    operation: trans.operation as i32,
                    previous: trans.previous as f32,
                    delta: trans.delta as f32,
                    current: trans.current as f32,
                };
                grpc_tx_vec.push(t)
            }
            // перекладка счета
            let grpc_acc = proto::AccountResponse {
                id: acc_val.id,
                status: acc_val.status as i32,
                balance: acc_val.balance as f32,
                transaction: grpc_tx_vec,
            };
            // добавление в grpc map
            grpc_map.insert(acc_id, grpc_acc);
        }

        Ok(Response::new(proto::HistoryResponse { response: grpc_map }))
    }

    /// Восстановление бд
    async fn backup(&self, _: Request<()>) -> Result<Response<()>, Status> {
        usecases::storage::backup(&self.state).unwrap();
        Ok(Response::new(()))
    }
}
