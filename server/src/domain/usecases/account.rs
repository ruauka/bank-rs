use crate::adapters::storage::cache::Cache;
use crate::adapters::storage::Storages;
use crate::domain::entities::account::{Account, BalanceResponse};
use crate::domain::entities::transaction::Operation::{
    Replenish, TransferDecrease, TransferIncrease, Withdraw,
};
use crate::domain::entities::transaction::{
    Operation, Transaction, TransactionResponse, TransferRequest, TransferResponse,
};
use crate::domain::errors::AppError;
use crate::domain::errors::AppError::{
    AccountNotExists, Overdraft, SelfTransfer, ZeroValueTransaction,
};
use std::sync::{Arc, RwLock};

/// Создание нового счета.
pub fn new_account<S: Storages>(storage: Arc<RwLock<S>>) -> TransactionResponse {
    // // получение названия последнего счета
    // let last_name: String = storage.write().unwrap().db().get_last_account_name();

    // создание нового счета
    let mut account: Account = Account::new();
    // let acc_id: u32 = account.id.clone();

    // создание транзакиции о создании счета
    let tx_new: Transaction = Transaction::new(
        0_u32,
        Operation::default(),
        f64::default(),
        f64::default(),
        f64::default(),
    );
    // добавление транзакции в список транзакций счета
    account.transactions.push(tx_new);
    // добавление счета в db
    let acc_id: u32 = storage.write().unwrap().db().create_account(account);
    // body
    let tx: TransactionResponse = TransactionResponse::new(acc_id, 0_u32, 0_f64);
    // backup
    storage.write().unwrap().db().backup_store();

    tx
}

/// Изменение баланса счета.
pub fn change_acc_balance<S: Storages>(
    storage: &Arc<RwLock<S>>,
    trans_value: f64,
    account_id: u32,
    operation: Operation,
) -> Result<TransactionResponse, AppError> {
    // проверка наличия счета
    if !storage.write().unwrap().db().check_key(account_id) {
        return Err(AccountNotExists(account_id.to_string()));
    }
    // проверка на наличие изменение баланса на 0 или меньше
    if trans_value <= 0_f64 {
        return Err(ZeroValueTransaction);
    }

    let mut binding = storage.write().unwrap();
    // получение счета
    let cur_acc: &mut Account = binding.db().get_mut_account(account_id);
    // проверка на снятие или перевод больше, чем есть на счете
    if matches!(operation, Withdraw | TransferDecrease) && cur_acc.balance < trans_value {
        return Err(Overdraft);
    }
    // id последней транзакции (совпадает с индексом)
    let last_tx_id: u32 = (cur_acc.transactions.len() - 1) as u32;
    // id новой транзакции
    let new_tx_id: u32 = last_tx_id + 1;
    // новый баланс счета
    let new_balance: f64 = if operation == Replenish || operation == TransferIncrease {
        // пополнение счета
        cur_acc.transactions[last_tx_id as usize].current + trans_value
    } else {
        // списание со счета
        cur_acc.transactions[last_tx_id as usize].current - trans_value
    };
    // создание новой транзакции
    let tx_new: Transaction = Transaction::new(
        new_tx_id,
        operation,
        cur_acc.transactions[last_tx_id as usize].current,
        trans_value,
        new_balance,
    );
    // добавление транзакции в бд
    cur_acc.transactions.push(tx_new);
    // обновление текущего баланса счета
    cur_acc.balance = new_balance;
    // body
    let tx: TransactionResponse = TransactionResponse::new(account_id, new_tx_id, cur_acc.balance);
    // backup
    binding.db().backup_store();

    Ok(tx)
}

/// Перевод со счета на счет.
pub fn transfer<S: Storages>(
    storage: &Arc<RwLock<S>>,
    payload: TransferRequest,
) -> Result<TransferResponse, AppError> {
    let p = payload.clone();
    let tx_value: f64 = payload.transfer_value;
    // проверка на наличие изменение баланса на 0 или меньше
    if tx_value <= 0_f64 {
        return Err(ZeroValueTransaction);
    }
    // проверка на перевод самому себе
    if payload.account_to == payload.account_from {
        return Err(SelfTransfer);
    }
    // проверка наличия счета
    if !storage
        .write()
        .unwrap()
        .db()
        .check_key(payload.account_from)
    {
        return Err(AccountNotExists(payload.account_from.to_string()));
    }
    // проверка наличия счета
    if !storage.write().unwrap().db().check_key(payload.account_to) {
        return Err(AccountNotExists(payload.account_to.to_string()));
    }
    // списание со счета отправителя
    change_acc_balance(storage, tx_value, payload.account_from, TransferDecrease)?;
    // пополнение счета получателя
    change_acc_balance(storage, tx_value, payload.account_to, TransferIncrease)?;

    // body
    let tx: TransferResponse = TransferResponse::new(p);
    // backup
    storage.write().unwrap().db().backup_store();

    Ok(tx)
}

/// Баланса счета.
pub fn balance<S: Storages>(
    storage: &Arc<RwLock<S>>,
    account_id: u32,
) -> Result<BalanceResponse, AppError> {
    // проверка наличия счета
    if !storage.write().unwrap().db().check_key(account_id) {
        return Err(AccountNotExists(account_id.to_string()));
    }
    let mut binding = storage.write().unwrap();
    // получение счета
    let account: &Account = binding.db().get_account(account_id);
    // body
    let balance: BalanceResponse = BalanceResponse::new(account.balance);

    Ok(balance)
}

/// Получение всех транзакций счета.
pub fn account<S: Storages>(
    storage: &Arc<RwLock<S>>,
    account_id: u32,
) -> Result<Account, AppError> {
    // проверка наличия счета
    if !storage.write().unwrap().db().check_key(account_id) {
        return Err(AccountNotExists(account_id.to_string()));
    }

    let mut binding = storage.write().unwrap();
    // получение счета
    let account: &Account = binding.db().get_account(account_id);

    Ok(account.clone())
}
