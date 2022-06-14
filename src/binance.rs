use std::str::FromStr;

use futures_util::{SinkExt, Stream, StreamExt};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::{
    model::{Exchange, Symbol},
    order_book::{self, Amount, Price},
};

pub async fn levels(
    websocket_url: url::Url,
    symbol: Symbol,
) -> Result<
    impl Stream<Item = (Vec<order_book::Level>, Vec<order_book::Level>)>,
    Box<dyn std::error::Error>,
> {
    let (ws_stream, _) = connect_async(websocket_url).await?;
    let (mut write, read) = ws_stream.split();
    write
        .send(Message::Text(serde_json::to_string(
            &ClientRawMessage::from(ClientMessage(symbol.into())),
        )?))
        .await?;
    let summaries = read
        .map(|ws_meessage| {
            let text = ws_meessage?.into_text()?;
            let raw_messsage: ServerRawMessage = serde_json::from_str(&text)?;
            ServerMessage::try_from(raw_messsage)
        })
        .filter_map(|rm| async move {
            rm.ok().and_then(|m| match m {
                ServerMessage::PartialBookDepth { asks, bids } => Some((
                    asks.into_iter().map(order_book::Level::from).collect(),
                    bids.into_iter().map(order_book::Level::from).collect(),
                )),
                _ => None,
            })
        });
    Ok(summaries)
}

#[derive(Debug, Eq, PartialEq)]
struct CurrencyPair(String);

impl std::convert::From<Symbol> for CurrencyPair {
    fn from(symbol: Symbol) -> Self {
        let Symbol { base, quote } = symbol;
        let base_str = base.to_string().to_lowercase();
        let quote_str = quote.to_string().to_lowercase();
        CurrencyPair(format!("{}{}", base_str, quote_str))
    }
}

struct ClientMessage(CurrencyPair);

#[derive(Serialize, Debug)]
struct StreamName(String);

#[derive(Serialize, Debug)]
#[serde(tag = "method", rename = "SUBSCRIBE")]
struct ClientRawMessage {
    params: Vec<StreamName>,
    id: i64,
}

impl std::convert::From<ClientMessage> for ClientRawMessage {
    fn from(message: ClientMessage) -> Self {
        ClientRawMessage {
            params: vec![StreamName(format!("{}@depth20@100ms", message.0 .0))],
            id: 1,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct Level {
    price: Price,
    amount: Amount,
}

impl std::convert::From<Level> for order_book::Level {
    fn from(level: Level) -> Self {
        let Level { price, amount } = level;
        Self {
            exchange: Exchange::Binance,
            price,
            amount,
        }
    }
}

impl std::convert::TryFrom<&Vec<String>> for Level {
    type Error = Box<dyn std::error::Error>;

    fn try_from(values: &Vec<String>) -> Result<Self, Self::Error> {
        if values.len() != 2 {
            Err("Incorrect number fields for a level".into())
        } else {
            Ok(Level {
                price: Price(Decimal::from_str(&values[0])?),
                amount: Amount(Decimal::from_str(&values[1])?),
            })
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ServerMessage {
    PartialBookDepth { asks: Vec<Level>, bids: Vec<Level> },
    SubscriptionSucceeded,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ServerResult {
    Null,
    Error { msg: String },
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ServerRawMessage {
    Response {
        result: ServerResult,
    },
    Payload {
        asks: Vec<Vec<String>>,
        bids: Vec<Vec<String>>,
    },
}

impl std::convert::TryFrom<ServerRawMessage> for ServerMessage {
    type Error = Box<dyn std::error::Error>;

    fn try_from(raw_message: ServerRawMessage) -> Result<Self, Self::Error> {
        match raw_message {
            ServerRawMessage::Response { result } => match result {
                ServerResult::Null => Ok(ServerMessage::SubscriptionSucceeded),
                ServerResult::Error { msg } => Err(format!("Error response: {}", msg).into()),
            },
            ServerRawMessage::Payload { asks, bids } => {
                let asks = asks.iter().map(Level::try_from).collect::<Result<_, _>>()?;
                let bids = bids.iter().map(Level::try_from).collect::<Result<_, _>>()?;
                Ok(ServerMessage::PartialBookDepth { asks, bids })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn level_into() {
        let level: order_book::Level = Level::default().into();
        assert_eq!(Exchange::Binance, level.exchange)
    }

    #[test]
    fn server_message_parse() -> Result<(), Box<dyn std::error::Error>> {
        let raw_message: ServerRawMessage = serde_json::from_value(json!(
        {
            "result": {
                "code": 1,
                "msg": "Invalid value type: expected Boolean"
            },
            "id": 1,
        }))?;
        assert!(ServerMessage::try_from(raw_message).is_err());
        use ServerMessage::*;
        let raw_message: ServerRawMessage = serde_json::from_value(json!(
        {
            "result": null,
            "id": 1,
        }))?;
        assert_eq!(SubscriptionSucceeded, ServerMessage::try_from(raw_message)?);
        let raw_message: ServerRawMessage = serde_json::from_value(json!(
        {
            "asks": [
                [
                    "28000",
                    "0.26"
                ]
            ],
            "bids": [
                [
                    "30000",
                    "0.33"
                ]
            ]
        }))?;
        assert_eq!(
            PartialBookDepth {
                asks: vec![Level {
                    price: Price("28000".parse()?),
                    amount: Amount("0.26".parse()?)
                }],
                bids: vec![Level {
                    price: Price("30000".parse()?),
                    amount: Amount("0.33".parse()?)
                }],
            },
            ServerMessage::try_from(raw_message)?
        );
        Ok(())
    }

    #[test]
    fn client_message_gen() -> Result<(), Box<dyn std::error::Error>> {
        let raw_message = json!(
        {
            "method": "SUBSCRIBE",
            "params": [
                "btcusd@depth20@100ms"
            ],
            "id": 1
        });
        assert_eq!(
            raw_message,
            serde_json::to_value(ClientRawMessage::from(ClientMessage(CurrencyPair(
                "btcusd".to_string()
            ))))?
        );
        Ok(())
    }
}
