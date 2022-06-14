use async_stream::stream;
use futures_util::{Stream, StreamExt};
use rust_decimal::Decimal;

use crate::order_book::{Level, Spread, Summary};

#[derive(Default)]
struct State(Levels, Levels);

type Levels = (Vec<Level>, Vec<Level>);

pub fn merge<T, U>(first: T, second: U) -> impl Stream<Item = Summary>
where
    T: Stream<Item = Levels> + 'static,
    U: Stream<Item = Levels> + 'static,
{
    let mut first = Box::pin(first);
    let mut second = Box::pin(second);
    let mut state = State::default();
    stream! {
        loop {
            tokio::select! {
                Some(update) = first.next() => {
                    state.0 = update;
                    yield compute(&state);
                }
                Some(update) = second.next() => {
                    state.1 = update;
                    yield compute(&state);
                }
                else => break
            }
        }
    }
}

fn compute(state: &State) -> Summary {
    let mut asks = Vec::new();
    asks.extend(&state.0 .0);
    asks.extend(&state.1 .0);
    asks.sort_by_key(|l| cmp_key(l));
    let mut bids = Vec::new();
    bids.extend(&state.0 .1);
    bids.extend(&state.1 .1);
    bids.sort_by_key(|l| cmp_key(l));
    bids.reverse();
    Summary {
        spread: Spread(asks[0].price.0 - bids[0].price.0),
        asks: asks.into_iter().take(10).cloned().collect(),
        bids: bids.into_iter().take(10).cloned().collect(),
    }
}

fn cmp_key(level: &Level) -> (Decimal, Decimal) {
    (level.price.0, -level.amount.0)
}

#[cfg(test)]
mod tests {
    use futures_util::{stream, StreamExt};

    use crate::{
        model::Exchange,
        order_book::{Amount, Level, Price, Spread, Summary},
    };

    #[tokio::test]
    async fn merge() -> Result<(), Box<dyn std::error::Error>> {
        use Exchange::*;
        let binance_levels = vec![
            (
                vec![Level {
                    exchange: Binance,
                    price: Price("17000".parse()?),
                    amount: Amount("1.24".parse()?),
                }],
                vec![Level {
                    exchange: Binance,
                    price: Price("16000".parse()?),
                    amount: Amount("1.13".parse()?),
                }],
            ),
            (
                vec![Level {
                    exchange: Binance,
                    price: Price("18000".parse()?),
                    amount: Amount("1.26".parse()?),
                }],
                vec![Level {
                    exchange: Binance,
                    price: Price("19000".parse()?),
                    amount: Amount("1.32".parse()?),
                }],
            ),
        ];
        let bitstamp_levels = vec![
            (
                vec![Level {
                    exchange: Bitstamp,
                    price: Price("27000".parse()?),
                    amount: Amount("0.24".parse()?),
                }],
                vec![Level {
                    exchange: Bitstamp,
                    price: Price("26000".parse()?),
                    amount: Amount("0.13".parse()?),
                }],
            ),
            (
                vec![Level {
                    exchange: Bitstamp,
                    price: Price("28000".parse()?),
                    amount: Amount("0.26".parse()?),
                }],
                vec![Level {
                    exchange: Bitstamp,
                    price: Price("29000".parse()?),
                    amount: Amount("0.32".parse()?),
                }],
            ),
        ];
        let merged: Vec<_> =
            super::merge(stream::iter(binance_levels), stream::iter(bitstamp_levels))
                .collect()
                .await;
        assert_eq!(
            Summary {
                spread: Spread("-11000".parse()?),
                bids: vec![
                    Level {
                        exchange: Bitstamp,
                        price: Price("29000".parse()?),
                        amount: Amount("0.32".parse()?)
                    },
                    Level {
                        exchange: Binance,
                        price: Price("19000".parse()?),
                        amount: Amount("1.32".parse()?)
                    }
                ],
                asks: vec![
                    Level {
                        exchange: Binance,
                        price: Price("18000".parse()?),
                        amount: Amount("1.26".parse()?)
                    },
                    Level {
                        exchange: Bitstamp,
                        price: Price("28000".parse()?),
                        amount: Amount("0.26".parse()?)
                    }
                ]
            },
            merged.last().cloned().ok_or("empty merged")?
        );
        Ok(())
    }
}
