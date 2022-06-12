#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Exchange {
    Binance,
    Bitstamp,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Currency {
    Eth,
    Btc,
    Usd,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Symbol {
    base: Currency,
    quote: Currency,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Market {
    exchange: Exchange,
    symbol: Symbol,
}
