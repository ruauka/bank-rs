use crate::domain::entities::account::Account;
use crate::domain::errors::AppError;
use crate::domain::errors::AppError::BackupLoadFileErr;
use axum::async_trait;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, RwLock};

/// Путь к backup.json для бэкапа db.
pub const PATH: &str = "src/backup";

/// Структура db (in-memory).
#[derive(Debug, Default)]
pub struct AccountStorageImpl {
    pub accounts: HashMap<String, Account>,
}

/// Трейт бд
#[async_trait]
pub trait AccountStorage {
    fn get_last_account_name(&mut self) -> String;
    fn create_account(&mut self, account: Account);
    fn check_key(&mut self, acc_name: &String) -> bool;
    fn get_mut_account(&mut self, acc_name: &String) -> &mut Account;
    fn get_account(&self, acc_name: &String) -> &Account;
    fn get_accounts(&self) -> &HashMap<String, Account>;
    fn backup_store(&mut self);
    fn backup_load(&mut self) -> Result<(), AppError>;
}

/// Имплементация трейта бд
// #[async_trait]
impl AccountStorage for AccountStorageImpl {
    /// Получение названия последнего счета для инкремента и созданиея нового счета.
    fn get_last_account_name(&mut self) -> String {
        // проверка на пустое db
        if self.accounts.is_empty() {
            return "".to_string();
        };
        // получение названия последнего счета
        let mut acc_names: Vec<&String> = self.accounts.keys().collect();
        acc_names.sort();
        acc_names.last().unwrap().to_string()
    }

    /// Создание нового счета.
    fn create_account(&mut self, account: Account) {
        let key: String = account.name.to_string();
        self.accounts.insert(key, account);
    }

    /// Проверка наличия счета
    fn check_key(&mut self, acc_name: &String) -> bool {
        self.accounts.contains_key(acc_name)
    }

    /// Получение счета для изменения баланса и добавления транзакций.
    fn get_mut_account(&mut self, acc_name: &String) -> &mut Account {
        self.accounts.get_mut(acc_name).unwrap()
    }

    /// Получение баланса счета.
    fn get_account(&self, acc_name: &String) -> &Account {
        self.accounts.get(acc_name).unwrap()
    }

    /// Получение всех счетов.
    fn get_accounts(&self) -> &HashMap<String, Account> {
        &self.accounts
    }

    /// Репликация бд в файл backup.json.
    fn backup_store(&mut self) {
        // Получение данных из мапы и преобразование к строке с json отступами (4).
        let obj = json!(&self.accounts);
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        obj.serialize(&mut ser).unwrap();
        let payload: String = String::from_utf8(buf).unwrap();

        // запись в файл для бэкапа
        if let Err(err) = fs::write(format!("{}/{}", &PATH, "backup.json"), &payload) {
            panic!("file backup.json write err: {}", err);
        }
    }

    /// Восстановление бд из файла backup.json.
    fn backup_load(&mut self) -> Result<(), AppError> {
        // чтение файла backup.json
        let backup_payload: String =
            match fs::read_to_string(format!("{}/{}", &PATH, "backup.json")) {
                Ok(res) => res,
                Err(_) => {
                    return Err(BackupLoadFileErr);
                }
            };

        let backup_bd: HashMap<String, Account> = serde_json::from_str(&backup_payload).unwrap();
        self.accounts = backup_bd;

        Ok(())
    }
}