#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use client::invoker::http::account::AccountInvoke;
use client::invoker::http::account::AccountInvoker;
use client::invoker::http::storage::StorageInvoke;
use client::invoker::http::storage::StorageInvoker;
use client::invoker::http::transaction::TransactionInvoke;
use client::invoker::http::transaction::TransactionInvoker;
use client::invoker::http::HttpInvoker;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client: Client = Client::builder().build().unwrap();
    let account = AccountInvoke::new(client.clone());
    let transaction = TransactionInvoke::new(client.clone());
    let storage = StorageInvoke::new(client.clone());

    let invoker = HttpInvoker::new(account, transaction, storage);

    let acc1 = invoker.account.create().await.unwrap();
    println!("{:#?}", acc1);

    let repl1 = invoker
        .account
        .replenish(acc1.account_id.clone(), 100_f64)
        .await
        .unwrap();
    println!("{:#?}", repl1);

    let acc1_history = invoker
        .account
        .account(acc1.account_id.clone())
        .await
        .unwrap();
    println!("{:#?}", acc1_history);

    let tr = invoker
        .transaction
        .transaction(acc1.account_id.clone(), 1)
        .await
        .unwrap();
    println!("{:#?}", tr);

    let acc2 = invoker.account.create().await.unwrap();
    let repl1 = invoker
        .account
        .replenish(acc1.account_id.clone(), 100_f64)
        .await
        .unwrap();
    println!("{:#?}", repl1);
    let repl2 = invoker
        .account
        .replenish(acc1.account_id.clone(), 100_f64)
        .await
        .unwrap();
    println!("{:#?}", repl2);

    let with1 = invoker
        .account
        .withdraw(acc1.account_id.clone(), 30_f64)
        .await;
    println!("{:#?}", with1);

    let tr = invoker
        .account
        .transfer(acc1.account_id.clone(), acc2.account_id.clone(), 30_f64)
        .await
        .unwrap();
    println!("{:#?}", with1);

    let res = invoker
        .account
        .balance(acc1.account_id.clone())
        .await
        .unwrap();
    println!("{:#?}", res);

    let backup = invoker.storage.backup().await.unwrap();
    println!("{:#?}", backup);

    let history = invoker.storage.history().await.unwrap();
    println!("{:#?}", history);
}
