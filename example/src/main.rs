#![allow(dead_code)]

use client::invoker::grpc::account::AccountInvoke as grpc_acc_invoke;
use client::invoker::grpc::account::AccountInvoker as grpc_acc_invoker;
use client::invoker::grpc::storage::StorageInvoke as grpc_st_invoke;
use client::invoker::grpc::storage::StorageInvoker as grpc_st_invoker;
use client::invoker::grpc::transaction::TransactionInvoke as grpc_tx_invoke;
use client::invoker::grpc::transaction::TransactionInvoker as grpc_tx_invoker;
use client::invoker::grpc::GRPCInvoker;
use client::invoker::http::account::AccountInvoke;
use client::invoker::http::account::AccountInvoker;
use client::invoker::http::storage::StorageInvoke;
use client::invoker::http::storage::StorageInvoker;
use client::invoker::http::transaction::TransactionInvoke;
use client::invoker::http::transaction::TransactionInvoker;
use client::invoker::http::HttpInvoker;

/// Пример http вызовов.
async fn http_call(
    invoker: HttpInvoker<AccountInvoke, TransactionInvoke, StorageInvoke>,
) -> Result<(), Box<dyn std::error::Error>> {
    let acc1 = invoker.account.create().await.unwrap();
    println!("{:#?}", acc1);

    let repl1 = invoker
        .account
        .replenish(acc1.account_id, 100_f64)
        .await
        .unwrap();
    println!("{:#?}", repl1);

    let acc1_history = invoker.account.account(acc1.account_id).await.unwrap();
    println!("{:#?}", acc1_history);

    let tr = invoker
        .transaction
        .transaction(acc1.account_id, 1)
        .await
        .unwrap();
    println!("{:#?}", tr);

    let acc2 = invoker.account.create().await.unwrap();
    let repl1 = invoker
        .account
        .replenish(acc1.account_id, 100_f64)
        .await
        .unwrap();
    println!("{:#?}", repl1);
    let repl2 = invoker
        .account
        .replenish(acc1.account_id, 100_f64)
        .await
        .unwrap();
    println!("{:#?}", repl2);

    let with1 = invoker.account.withdraw(acc1.account_id, 30_f64).await;
    println!("{:#?}", with1);

    let tr = invoker
        .account
        .transfer(acc1.account_id, acc2.account_id, 30_f64)
        .await
        .unwrap();
    println!("{:#?}", tr);

    let res = invoker.account.balance(acc1.account_id).await.unwrap();
    println!("{:#?}", res);

    let backup = invoker.storage.backup().await.unwrap();
    println!("{:#?}", backup);

    let history = invoker.storage.history().await.unwrap();
    println!("{:#?}", history);
    Ok(())
}

/// Пример gRPC вызовов.
async fn grpc_call(
    mut invoker: GRPCInvoker<grpc_acc_invoke, grpc_tx_invoke, grpc_st_invoke>,
) -> Result<(), Box<dyn std::error::Error>> {
    let acc1 = invoker.account.create().await?;

    let a = acc1.get_ref();
    println!("Response: {:?}", a);

    let res = invoker.account.replenish(a.account_id, 120_f64).await?;
    println!("Response: {:?}", res.get_ref());

    let res = invoker.account.withdraw(a.account_id, 20_f64).await?;
    println!("Response: {:?}", res.get_ref());

    let acc2 = invoker.account.create().await?;
    let a2 = acc2.get_ref();
    println!("Response: {:?}", a2);

    let res = invoker
        .account
        .transfer(a.account_id, a2.account_id, 30_f64)
        .await?;
    println!("Response: {:?}", res.get_ref());

    let res = invoker.account.balance(a2.account_id).await?;
    println!("Response: {:?}", res.get_ref());

    let res = invoker.account.account(a2.account_id).await?;
    println!("Response: {:?}", res.get_ref());

    let res = invoker.transaction.transaction(a2.account_id, 1).await?;
    println!("Response: {:?}", res.get_ref());

    let res = invoker.storage.history().await?;
    println!("Response: {:?}", res.get_ref());

    let res = invoker.storage.backup().await?;
    if res.metadata().get("grpc-status").unwrap() == "0" {
        println!("Response: ok",);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // что то одно должно быть закомментированно

    // // пример http вызовов
    // let invoker = HttpInvoker::new();
    // http_call(invoker).await?;

    //  пример gRPC вызовов
    let invoker = GRPCInvoker::new().await;
    grpc_call(invoker).await?;

    Ok(())
}
