use crate::domain::entities::account::Account;
use crate::domain::errors::AppError;
use crate::domain::errors::AppError::{BackupLoadFile, EmptyBackupFile, InvalidBackupFile};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Путь к backup.json для бэкапа db.
pub const PATH: &str = "server/backup";

/// Структура db (in-memory).
#[derive(Debug, Default)]
pub struct CacheImpl {
    pub cache: HashMap<String, Account>,
}

/// Трейт бд
pub trait Cache {
    /// Получение названия последнего счета для инкремента и создания нового счета.
    fn get_last_account_name(&self) -> String;
    /// Создание нового счета.
    fn create_account(&mut self, account: Account);
    /// Проверка наличия счета
    fn check_key(&mut self, acc_name: &str) -> bool;
    /// Получение счета для изменения баланса и добавления транзакций.
    fn get_mut_account(&mut self, acc_name: &str) -> &mut Account;
    /// Получение счета.
    fn get_account(&self, acc_name: &str) -> &Account;
    /// Получение всех счетов.
    fn get_accounts(&self) -> &HashMap<String, Account>;
    /// Репликация бд в файл backup.json.
    fn backup_store(&mut self);
    /// Восстановление бд из файла backup.json.
    fn backup_load(&mut self) -> Result<(), AppError>;
}

impl Cache for CacheImpl {
    fn get_last_account_name(&self) -> String {
        // проверка на пустое db
        if self.cache.is_empty() {
            return "".to_string();
        };
        // получение названия последнего счета
        let mut acc_names: Vec<&String> = self.cache.keys().collect();
        acc_names.sort();
        acc_names.last().unwrap().to_string()
    }

    fn create_account(&mut self, account: Account) {
        let key: String = account.name.to_string();
        self.cache.insert(key, account);
    }

    fn check_key(&mut self, acc_name: &str) -> bool {
        self.cache.contains_key(acc_name)
    }

    fn get_mut_account(&mut self, acc_name: &str) -> &mut Account {
        self.cache.get_mut(acc_name).unwrap()
    }

    fn get_account(&self, acc_name: &str) -> &Account {
        self.cache.get(acc_name).unwrap()
    }

    fn get_accounts(&self) -> &HashMap<String, Account> {
        &self.cache
    }

    fn backup_store(&mut self) {
        // Получение данных из мапы и преобразование к строке с json отступами (4).
        let obj = json!(&self.cache);
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        obj.serialize(&mut ser).unwrap();
        let payload: String = String::from_utf8(buf).unwrap();

        // запись в файл для бэкапа
        if let Err(err) = fs::write(Path::new(&PATH).join("backup.json"), payload) {
            panic!("file backup.json write err: {}", err);
        }
    }

    fn backup_load(&mut self) -> Result<(), AppError> {
        // чтение файла backup.json
        let backup_payload: String =
            fs::read_to_string(Path::new(&PATH).join("backup.json")).map_err(|_| BackupLoadFile)?;
        // проверка на пустую реплику
        if backup_payload.is_empty() {
            return Err(EmptyBackupFile);
        }
        // backup
        let backup_bd: HashMap<String, Account> =
            serde_json::from_str(&backup_payload).map_err(|_| InvalidBackupFile)?;
        self.cache = backup_bd;

        Ok(())
    }
}
