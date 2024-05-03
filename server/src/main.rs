// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_assignments)]
// #![allow(unused_mut)]

mod adapters;
mod domain;
mod server;

use crate::server::execute;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    execute().await
}
