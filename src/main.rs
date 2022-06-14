use std::env;

use futures_util::{pin_mut, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::tungstenite::protocol::Message;

use orderbook_proto::orderbook_aggregator_server::OrderbookAggregator;

use keyrock_tech_challenge::{
    binance, bitstamp,
    logic::merge,
    model::{Currency, Symbol},
    order_book::Summary,
};

pub mod orderbook_proto {
    tonic::include_proto!("orderbook");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbol = Symbol {
        base: Currency::Eth,
        quote: Currency::Btc,
    };
    let binance_levels = binance::levels(
        url::Url::parse("wss://stream.binance.com:9443/ws").unwrap(),
        symbol.clone(),
    )
    .await?;
    let bitstamp_levels =
        bitstamp::levels(url::Url::parse("wss://ws.bitstamp.net").unwrap(), symbol).await?;
    let ws_to_stdout = {
        merge(binance_levels, bitstamp_levels).for_each(
            |Summary { asks, bids, spread }| async move {
                tokio::io::stdout()
                    .write_all(
                        format!(
                            "asks:\n{:#?}\nbids:\n{:#?}\nspread:{}\n",
                            asks, bids, spread.0
                        )
                        .as_bytes(),
                    )
                    .await
                    .unwrap();
            },
        )
    };

    pin_mut!(ws_to_stdout);
    ws_to_stdout.await;
    Ok(())
}
