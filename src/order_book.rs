use futures_util::{stream::BoxStream, Future};
use rust_decimal::Decimal;

use crate::model::Exchange;

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct Spread(Decimal);

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Price(pub Decimal);

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Amount(pub Decimal);

#[derive(Debug, Eq, PartialEq)]
pub struct Level {
    pub exchange: Exchange,
    pub price: Price,
    pub amount: Amount,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Summary {
    pub spread: Spread,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

pub(crate) trait Orderbook {
    fn summary(
        &self,
    ) -> Box<dyn Future<Output = Result<BoxStream<Summary>, Box<dyn std::error::Error>>>>;
}
