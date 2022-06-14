use std::pin::Pin;

use futures_util::{Stream, StreamExt};

use keyrock_tech_challenge::grpc;

use rustea::{
    command,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    App, Command, Message,
};

struct Model {
    stream: Pin<Box<dyn Stream<Item = grpc::proto::Summary> + Send>>,
    summary: Option<grpc::proto::Summary>,
}

impl App for Model {
    fn init(&self) -> Option<Command> {
        Some(Box::new(move || Some(Box::new(InitMessage))))
    }

    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Some(key_event) = msg.downcast_ref::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                if let KeyCode::Char('c') = key_event.code {
                    return Some(Box::new(command::quit));
                }
            }
        } else if let Some(summary) = msg.downcast_ref::<SummaryMessage>() {
            self.summary.replace(summary.0.clone());
        }
        Some(make_request_command(&mut self.stream))
    }

    fn view(&self) -> String {
        if let Some(ref summary) = &self.summary {
            let mut out = format!("Spread: {}", summary.spread);
            out.push_str(&format!(
                "\n{:<8}\t{:<8}\t{:<8}\t{:<8}\t{:<8}\t{:<8}",
                "", "Asks", "", "", "Bids", ""
            ));
            out.push_str(&format!("\n{:-<88}", ""));
            out.push_str(&format!(
                "\n{:<8}\t{:<8}\t{:<8}\t{:<8}\t{:<8}\t{:<8}",
                "Exchange", "Price", "Amount", "Exchange", "Price", "Amount"
            ));
            out.push_str(&format!(
                "\n{:-<8}\t{:-<8}\t{:-<8}\t{:-<8}\t{:-<8}\t{:-<8}",
                "", "", "", "", "", ""
            ));
            for (ask, bid) in std::iter::zip(&summary.asks, &summary.bids) {
                out.push_str(&format!(
                    "\n{:<8}\t{:<8}\t{:<8}\t{:<8}\t{:<8}\t{:<8}",
                    ask.exchange, ask.price, ask.amount, bid.exchange, bid.price, bid.amount
                ));
            }
            out
        } else {
            String::new()
        }
    }
}

struct InitMessage;
struct SummaryMessage(grpc::proto::Summary);

fn make_request_command(
    stream: &mut Pin<Box<dyn Stream<Item = grpc::proto::Summary> + Send>>,
) -> Command {
    let fut = stream.next();
    let summary = futures::executor::block_on(fut);
    Box::new(move || {
        // It's okay to block since commands are multi threaded
        Some(Box::new(SummaryMessage(summary.expect("have summary"))))
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut client = grpc::proto::orderbook_aggregator_client::OrderbookAggregatorClient::connect(
        "http://[::1]:50051",
    )
    .await?;
    let stream = client
        .book_summary(grpc::proto::Empty {})
        .await?
        .into_inner();
    rustea::run(Model {
        stream: Box::pin(stream.filter_map(|r| async { r.ok() })),
        summary: None,
    })?;
    Ok(())
}
