use rust_decimal::Decimal;

use crate::model::Exchange;

struct Spread(Decimal);

struct Price(Decimal);

struct Amount(Decimal);

struct Level {
    exchange: Exchange,
    price: Price,
    amount: Amount,
}

struct Summary {
    spread: Spread,
    bids: Vec<Level>,
    asks: Vec<Level>,
}
