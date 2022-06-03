use crate::model::Exchange;

struct Spread(f64);

struct Price(f64);

struct Amount(f64);

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
