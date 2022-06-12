use std::str::FromStr;

use chrono::{offset::TimeZone, DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::order_book::{Amount, Price};

#[derive(Debug, Eq, PartialEq, Serialize)]
struct CurrencyPair(String);

enum ClientMessage {
    Heartbeat,
    Subscribe(CurrencyPair),
}

#[derive(Serialize, Debug)]
struct ClientData {
    channel: String,
}

#[derive(Serialize, Debug)]
struct ClientRawMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<ClientData>,
    event: String,
}

impl std::convert::From<ClientMessage> for ClientRawMessage {
    fn from(message: ClientMessage) -> Self {
        match message {
            ClientMessage::Heartbeat => ClientRawMessage {
                data: None,
                event: "bts:heartbeat".to_string(),
            },
            ClientMessage::Subscribe(CurrencyPair(cp)) => ClientRawMessage {
                data: Some(ClientData {
                    channel: format!("order_book_{}", cp),
                }),
                event: "bts:subscribe".to_string(),
            },
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct Level {
    price: Price,
    amount: Amount,
}

impl std::convert::From<Level> for crate::order_book::Level {
    fn from(level: Level) -> Self {
        let Level { price, amount } = level;
        Self {
            exchange: crate::model::Exchange::Bitstamp,
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
    Heartbeat,
    LiveOrderBookData {
        asks: Vec<Level>,
        bids: Vec<Level>,
        currency_pair: CurrencyPair,
        timestamp: DateTime<Utc>,
    },
    RequestReconnect,
    SubscriptionSuceeded(CurrencyPair),
}

#[derive(Deserialize, Debug)]
enum ServerData {
    #[serde(rename = "data")]
    Object {
        asks: Option<Vec<Vec<String>>>,
        bids: Option<Vec<Vec<String>>>,
        timestamp: Option<String>,
    },
    #[serde(rename = "data")]
    Str(String),
}

#[derive(Deserialize, Debug)]
struct ServerRawMessage {
    channel: Option<String>,
    #[serde(flatten)]
    data: Option<ServerData>,
    event: String,
}

fn parse_currency_pair(
    channel_name: &Option<String>,
) -> Result<CurrencyPair, Box<dyn std::error::Error>> {
    if let Some(channel_name) = channel_name {
        let parts: Vec<_> = channel_name.split('_').collect();
        if parts.len() == 3 && parts[0] == "order" && parts[1] == "book" {
            Ok(CurrencyPair(parts[2].to_string()))
        } else {
            Err(format!("Could not parse channel name: {}", channel_name).into())
        }
    } else {
        Err("JSON field \"channel\" absent".into())
    }
}

impl std::convert::TryFrom<ServerRawMessage> for ServerMessage {
    type Error = Box<dyn std::error::Error>;

    fn try_from(raw_message: ServerRawMessage) -> Result<Self, Self::Error> {
        match raw_message.event.as_str() {
            "bts:heartbeat" => Ok(ServerMessage::Heartbeat),
            "bts:request_reconnect" => Ok(ServerMessage::RequestReconnect),
            "bts:subscription_succeeded" => Ok(ServerMessage::SubscriptionSuceeded(
                parse_currency_pair(&raw_message.channel)?,
            )),
            "data" => {
                if let Some(ServerData::Object {
                    asks,
                    bids,
                    timestamp,
                }) = raw_message.data
                {
                    let currency_pair = parse_currency_pair(&raw_message.channel)?;
                    let timestamp = Utc.datetime_from_str(
                        &timestamp.ok_or("JSON field \"timestamp\" absent")?,
                        "%s",
                    )?;
                    let asks = asks
                        .ok_or("JSON field \"asks\" absent")?
                        .iter()
                        .map(Level::try_from)
                        .collect::<Result<_, _>>()?;
                    let bids = bids
                        .ok_or("JSON field \"bids\" absent")?
                        .iter()
                        .map(Level::try_from)
                        .collect::<Result<_, _>>()?;
                    Ok(ServerMessage::LiveOrderBookData {
                        asks,
                        bids,
                        currency_pair,
                        timestamp,
                    })
                } else {
                    Err("JSON field \"data\" absent".into())
                }
            }
            _ => Err(format!(
                "Unknown value for JSON field \"event\": {}",
                raw_message.event
            )
            .into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn level_into() {
        let level: crate::order_book::Level = Level::default().into();
        assert_eq!(crate::model::Exchange::Bitstamp, level.exchange)
    }

    #[test]
    fn server_message_parse() -> Result<(), Box<dyn std::error::Error>> {
        use ServerMessage::*;
        let raw_message: ServerRawMessage = serde_json::from_value(json!(
        {
            "event": "bts:heartbeat",
            "channel": "",
            "data": {
                "status": "success"
            }
        }))?;
        assert_eq!(Heartbeat, ServerMessage::try_from(raw_message)?);
        let raw_message: ServerRawMessage = serde_json::from_value(json!(
        {
            "event": "data",
            "channel": "order_book_btcusd",
            "data": {
                "timestamp": "1655017448",
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
            }
        }))?;
        assert_eq!(
            LiveOrderBookData {
                asks: vec![Level {
                    price: Price("28000".parse()?),
                    amount: Amount("0.26".parse()?)
                }],
                bids: vec![Level {
                    price: Price("30000".parse()?),
                    amount: Amount("0.33".parse()?)
                }],
                currency_pair: CurrencyPair("btcusd".to_string()),
                timestamp: Utc.datetime_from_str("1655017448", "%s")?
            },
            ServerMessage::try_from(raw_message)?
        );
        let raw_message: ServerRawMessage = serde_json::from_value(json!(
        {
            "event": "bts:request_reconnect",
            "channel": "",
            "data": ""
        }))?;
        assert_eq!(RequestReconnect, ServerMessage::try_from(raw_message)?);
        let raw_message: ServerRawMessage = serde_json::from_value(json!(
        {
            "event": "bts:subscription_succeeded",
            "channel": "order_book_btcusd",
            "data": {}
        }))?;
        assert_eq!(
            SubscriptionSuceeded(CurrencyPair("btcusd".to_string())),
            ServerMessage::try_from(raw_message)?
        );
        Ok(())
    }

    #[test]
    fn client_message_gen() -> Result<(), Box<dyn std::error::Error>> {
        use ClientMessage::*;
        let raw_message = json!(
        {
            "event": "bts:heartbeat",
        });
        assert_eq!(
            raw_message,
            serde_json::to_value(ClientRawMessage::from(Heartbeat))?
        );
        let raw_message = json!(
        {
            "event": "bts:subscribe",
            "data": {
                "channel": "order_book_btcusd",
            }
        });
        assert_eq!(
            raw_message,
            serde_json::to_value(ClientRawMessage::from(Subscribe(CurrencyPair(
                "btcusd".to_string()
            ))))?
        );
        Ok(())
    }
}
