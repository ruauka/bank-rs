pub mod account;
pub mod storage;
pub mod transaction;

use crate::invoker::http::account::{AccountInvoke, AccountInvoker};
use crate::invoker::http::storage::{StorageInvoke, StorageInvoker};
use crate::invoker::http::transaction::{TransactionInvoke, TransactionInvoker};
use reqwest::Client;

/// URL сервера.
const URL: &str = "http://localhost:8080";

/// Общая структура объектов вызова хендлеров сервера.
pub struct HttpInvoker<A, T, S>
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

// ЭТО ПЛОХОЙ ВАРИАНТ КОНСТРУКТОРА, ТК ПОЛЬЗОВАТЕЛЬ ДОЛЖЕН СОЗДАВАТЬ И ПРОКИДЫВАТЬ СЮДА ОБЪЕКТЫ ТЕЙТОВ.
// НИЖЕ, ЗАКОММЕНТИРОВАННЫЙ ВАРИАНТ, КАК ХОЧУ СДЕЛАТЬ
impl<A: AccountInvoker, T: TransactionInvoker, S: StorageInvoker> HttpInvoker<A, T, S> {
    /// Конструктор.
    pub fn new(account: A, transaction: T, storage: S) -> Self {
        Self {
            account,
            transaction,
            storage,
        }
    }
}

// Вопросы:
// 1. Так ли принято делать в rust (прокидывание трейтов в поля HttpInvoker).
// Делал на манер Go, там принято прокидывать интерфейсы в поля слоя, который выше (так в Go реализуется чистая архитектура)
// 2. как сделать конструктор клиента как указано ниже, что бы все объекты инициализировались в самом конструкторе,
// а не как в example/src/main.rs (18,19, 20, 21 строки)
// Указанный ниже вариант не работает
// 3. как быть с warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified

// impl<A: AccountInvoker, T: TransactionInvoker, S: StorageInvoker> HttpInvoker<A, T, S> {
//     /// Конструктор.
//     pub fn new() -> Self {
//         // Клиент для создания TCP соединения
//         let client: Client = Client::builder().build().unwrap();
//
//         Self {
//             account: AccountInvoke::new(client.clone()),
//             transaction: TransactionInvoke::new(client.clone()),
//             storage: StorageInvoke::new(client),
//         }
//     }
// }
