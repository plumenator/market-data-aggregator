pub enum Exchange {
    Binance,
    BitStamp,
}

enum Currency {
    Eth,
    Btc,
    Usd,
}

struct Symbol {
    base: Currency,
    quote: Currency,
}

struct Market {
    exchange: Exchange,
    symbol: Symbol,
}
