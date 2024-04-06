use crate::adapters::storage::storage::AccountStorage;
use crate::adapters::storage::{StorageState, Storages};
use crate::domain::entities::account::Account;
use crate::domain::entities::transaction::Operation::{
    Replenish, TransferDecrease, TransferIncrease, Withdraw,
};
use crate::domain::entities::transaction::{
    Operation, Transaction, TransactionRequest, TransactionResponse, TransferRequest,
    TransferResponse,
};
use crate::domain::errors::AppError;
use crate::domain::errors::AppError::{
    AccountExistsErr, OverdraftErr, SelfTransactionErr, ZeroValueTransactionErr,
};
use crate::domain::usecases;
use axum::extract::State;
use axum::Json;
use std::sync::{Arc, RwLock};

pub fn new_account<S: Storages>(storage: Arc<RwLock<S>>) -> TransactionResponse {
    // получение названия последнего счета
    let last_name: String = storage.write().unwrap().db().get_last_account_name();

    // создание нового счета
    let mut account: Account = Account::new(last_name);
    let acc_name: String = account.name.clone();

    // создание транзакиции о создании счета
    let trans: Transaction = Transaction::new(
        0_u32,
        Operation::Registration.to_string().to_lowercase(),
        f64::default(),
        f64::default(),
        f64::default(),
    );
    // добавление транзакции в список транзакций счета
    account.transactions.push(trans);
    // добавление счета в db
    storage.write().unwrap().db().create_account(account);
    // body
    let tr: TransactionResponse = TransactionResponse::new(Some(acc_name), Some(0_u32), None);
    // backup
    storage.write().unwrap().db().backup_store();

    tr
}

/// Изменение баланса счета.
pub fn change_acc_balance<S: Storages>(
    storage: &Arc<RwLock<S>>,
    trans_value: f64,
    account_name: &String,
    operation: Operation,
) -> Result<TransactionResponse, AppError> {
    // проверка наличия счета
    if !storage.write().unwrap().db().check_key(&account_name) {
        return Err(AccountExistsErr(account_name.to_string()));
    }
    // проверка на наличие изменение баланса на 0 или меньше
    if trans_value <= 0_f64 {
        return Err(ZeroValueTransactionErr);
    }

    let mut binding = storage.write().unwrap();
    // получение счета
    let mut cur_acc: &mut Account = binding.db().get_mut_account(&account_name);
    // проверка на снятие или перевод больше, чем есть на счете
    if (operation == Withdraw || operation == TransferDecrease) && cur_acc.balance < trans_value {
        return Err(OverdraftErr);
    }
    // id последней транзакции
    let last_tr_idx: u32 = (cur_acc.transactions.len() - 1) as u32;
    // id новой транзакции
    let new_trans_id: u32 = last_tr_idx + 1;
    // новый баланс счета
    let new_balance: f64 = if operation == Replenish || operation == TransferIncrease {
        // пополнение счета
        cur_acc.transactions[last_tr_idx as usize].current + trans_value
    } else {
        // списание со счета
        cur_acc.transactions[last_tr_idx as usize].current - trans_value
    };
    // создание новой транзакции
    let trans: Transaction = Transaction::new(
        new_trans_id,
        operation.to_string().to_lowercase(),
        cur_acc.transactions[last_tr_idx as usize].current,
        trans_value,
        new_balance,
    );
    // добавление транзакции в бд
    cur_acc.transactions.push(trans);
    // обновление текущего баланса счета
    cur_acc.balance = new_balance;
    // body
    let tr: TransactionResponse =
        TransactionResponse::new(Some(account_name.to_string()), Some(new_trans_id), None);
    // backup
    binding.db().backup_store();

    Ok(tr)
}

/// Перевод со счета на счет.
pub fn transfer<S: Storages>(
    storage: Arc<RwLock<S>>,
    payload: TransferRequest,
) -> Result<TransferResponse, AppError> {
    let p = payload.clone();
    let trans_value: f64 = payload.transfer_value;
    // проверка на наличие изменение баланса на 0 или меньше
    if trans_value <= 0_f64 {
        return Err(ZeroValueTransactionErr);
    }
    // проверка на перевод самому себе
    if payload.account_to == payload.account_from {
        return Err(SelfTransactionErr);
    }
    // проверка наличия счета
    if !storage
        .write()
        .unwrap()
        .db()
        .check_key(&payload.account_from)
    {
        return Err(AccountExistsErr(payload.account_from));
    }
    // проверка наличия счета
    if !storage.write().unwrap().db().check_key(&payload.account_to) {
        return Err(AccountExistsErr(payload.account_to));
    }
    // списание со счета отправителя
    if let Err(err) = change_acc_balance(
        &storage,
        trans_value,
        &payload.account_from,
        TransferDecrease,
    ) {
        return Err(err);
    }
    // пополнение счета получателя
    if let Err(err) =
        change_acc_balance(&storage, trans_value, &payload.account_to, TransferIncrease)
    {
        return Err(err);
    }
    // body
    let tr: TransferResponse = TransferResponse::new(p);
    // backup
    storage.write().unwrap().db().backup_store();

    Ok(tr)
}

/// Баланса счета.
pub fn get_balance<S: Storages>(
    storage: Arc<RwLock<S>>,
    account_name: String,
) -> Result<TransactionResponse, AppError> {
    // проверка наличия счета
    if !storage.write().unwrap().db().check_key(&account_name) {
        return Err(AccountExistsErr(account_name));
    }
    let mut binding = storage.write().unwrap();
    // получение счета
    let account: &Account = binding.db().get_account(&account_name);
    // body
    let tr: TransactionResponse =
        TransactionResponse::new(Some(account_name), None, Some(account.balance));

    Ok(tr)
}
