use std::env;

use futures_util::{pin_mut, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::tungstenite::protocol::Message;

use orderbook_grpc::orderbook_aggregator_server::OrderbookAggregator;

use keyrock_tech_challenge::{binance, bitstamp, model};

pub mod orderbook_grpc {
    tonic::include_proto!("orderbook");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connect_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("this program requires at least one argument"));

    let url = url::Url::parse(&connect_addr).unwrap();
    let levels = binance::levels(
        url,
        model::Symbol {
            base: model::Currency::Eth,
            quote: model::Currency::Btc,
        },
    )
    .await?;
    let ws_to_stdout = {
        levels.for_each(|(asks, bids)| async move {
            tokio::io::stdout()
                .write_all(format!("asks:\n{:#?}\nbids:\n{:#?}", asks, bids).as_bytes())
                .await
                .unwrap();
        })
    };

    pin_mut!(ws_to_stdout);
    ws_to_stdout.await;
    Ok(())
}
