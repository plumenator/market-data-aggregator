use std::net::ToSocketAddrs;

use structopt::StructOpt;
use tonic::transport::Server;

use keyrock_tech_challenge::{
    grpc,
    model::{Currency, Symbol, CURRENCY_VARIANTS},
};

/// A basic example
#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "b", long)]
    #[structopt(possible_values = &CURRENCY_VARIANTS, case_insensitive = true)]
    base: Currency,

    #[structopt(short = "q", long)]
    #[structopt(possible_values = &CURRENCY_VARIANTS, case_insensitive = true)]
    quote: Currency,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let Opt { base, quote } = Opt::from_args();
    let symbol = Symbol { base, quote };
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
