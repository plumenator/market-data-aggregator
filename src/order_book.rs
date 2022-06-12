use rust_decimal::Decimal;

use crate::model::Exchange;

#[derive(Debug, Default, Eq, PartialEq)]
struct Spread(Decimal);

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct Price(pub Decimal);

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct Amount(pub Decimal);

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Level {
    pub exchange: Exchange,
    pub price: Price,
    pub amount: Amount,
}

#[derive(Debug, Eq, PartialEq)]
struct Summary {
    spread: Spread,
    bids: Vec<Level>,
    asks: Vec<Level>,
}
