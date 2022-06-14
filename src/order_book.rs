use rust_decimal::Decimal;

use crate::model::Exchange;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Spread(pub Decimal);

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Price(pub Decimal);

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Amount(pub Decimal);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Level {
    pub exchange: Exchange,
    pub price: Price,
    pub amount: Amount,
}

impl Default for Level {
    fn default() -> Self {
        Level {
            exchange: Exchange::Binance,
            price: Price::default(),
            amount: Amount::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Summary {
    pub spread: Spread,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}
