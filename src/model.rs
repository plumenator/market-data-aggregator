#[derive(Debug, Eq, PartialEq)]
pub enum Exchange {
    Binance,
    Bitstamp,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Currency {
    Eth,
    Btc,
    Usd,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Symbol {
    pub base: Currency,
    pub quote: Currency,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Market {
    exchange: Exchange,
    symbol: Symbol,
}
