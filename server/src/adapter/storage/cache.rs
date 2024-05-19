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
    pub id: u32,
    pub cache: HashMap<u32, Account>,
}

/// Трейт бд
pub trait Cache {
    /// Создание нового счета.
    fn create_account(&mut self, account: Account) -> u32;
    /// Проверка наличия счета
    fn check_key(&mut self, acc_id: u32) -> bool;
    /// Получение счета для изменения баланса и добавления транзакций.
    fn get_mut_account(&mut self, acc_id: u32) -> &mut Account;
    /// Получение счета.
    fn get_account(&self, acc_id: u32) -> &Account;
    /// Получение всех счетов.
    fn get_accounts(&self) -> &HashMap<u32, Account>;
    /// Репликация бд в файл backup.json.
    fn backup_store(&mut self);
    /// Восстановление бд из файла backup.json.
    fn backup_load(&mut self) -> Result<(), AppError>;
}

impl Cache for CacheImpl {
    fn create_account(&mut self, mut account: Account) -> u32 {
        // инкрементирование номера счета
        self.id += 1;
        // добавление id счета
        account.id = self.id;
        // добавление в кэш
        self.cache.insert(self.id, account);

        self.id
    }

    fn check_key(&mut self, acc_id: u32) -> bool {
        self.cache.contains_key(&acc_id)
    }

    fn get_mut_account(&mut self, acc_id: u32) -> &mut Account {
        self.cache.get_mut(&acc_id).unwrap()
    }

    fn get_account(&self, acc_id: u32) -> &Account {
        self.cache.get(&acc_id).unwrap()
    }

    fn get_accounts(&self) -> &HashMap<u32, Account> {
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
        let backup_bd: HashMap<u32, Account> =
            serde_json::from_str(&backup_payload).map_err(|_| InvalidBackupFile)?;
        self.cache = backup_bd;

        Ok(())
    }
}
