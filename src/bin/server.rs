use std::{env, net::ToSocketAddrs};

use keyrock_tech_challenge::{
    grpc,
    model::{Currency, Symbol},
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let symbol = Symbol {
        base: Currency::Eth,
        quote: Currency::Btc,
    };
    let server = grpc::OrderbookAggregator { symbol };
    Server::builder()
        .add_service(
            grpc::proto::orderbook_aggregator_server::OrderbookAggregatorServer::new(server),
        )
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
    Ok(())
}
