use std::pin::Pin;

use futures_util::{Stream, StreamExt};
use tonic::{Request, Response, Status};

use crate::{model::Symbol, order_book};

pub mod proto {
    tonic::include_proto!("orderbook");
}

#[derive(Debug)]
pub struct OrderbookAggregator {
    pub symbol: Symbol,
}

#[tonic::async_trait]
impl proto::orderbook_aggregator_server::OrderbookAggregator for OrderbookAggregator {
    type BookSummaryStream = Pin<Box<dyn Stream<Item = Result<proto::Summary, Status>> + Send>>;

    async fn book_summary(
        &self,
        _: Request<proto::Empty>,
    ) -> Result<Response<Self::BookSummaryStream>, Status> {
        let binance_levels = crate::binance::levels(
            url::Url::parse("wss://stream.binance.com:9443/ws").unwrap(),
            self.symbol.clone(),
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        let bitstamp_levels = crate::bitstamp::levels(
            url::Url::parse("wss://ws.bitstamp.net").unwrap(),
            self.symbol.clone(),
        )
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let output_stream = crate::logic::merge(binance_levels, bitstamp_levels).map(
            |order_book::Summary { asks, bids, spread }| {
                Result::<_, Status>::Ok(proto::Summary {
                    spread: spread.0.try_into().expect("Decimal converts"),
                    asks: asks
                        .into_iter()
                        .map(
                            |order_book::Level {
                                 exchange,
                                 price,
                                 amount,
                             }| {
                                proto::Level {
                                    exchange: exchange.to_string(),
                                    price: price.0.try_into().expect("Decimal converts"),
                                    amount: amount.0.try_into().expect("Decimal converts"),
                                }
                            },
                        )
                        .collect(),
                    bids: bids
                        .into_iter()
                        .map(
                            |order_book::Level {
                                 exchange,
                                 price,
                                 amount,
                             }| {
                                proto::Level {
                                    exchange: exchange.to_string(),
                                    price: price.0.try_into().expect("Decimal converts"),
                                    amount: amount.0.try_into().expect("Decimal converts"),
                                }
                            },
                        )
                        .collect(),
                })
            },
        );
        Ok(Response::new(
            Box::pin(output_stream) as Self::BookSummaryStream
        ))
    }
}
