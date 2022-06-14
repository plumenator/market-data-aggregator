use futures_util::StreamExt;

use keyrock_tech_challenge::grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut client = grpc::proto::orderbook_aggregator_client::OrderbookAggregatorClient::connect(
        "http://[::1]:50051",
    )
    .await
    .unwrap();
    let mut stream = client
        .book_summary(grpc::proto::Empty {})
        .await
        .unwrap()
        .into_inner();

    while let Some(item) = stream.next().await {
        eprintln!("\treceived: {:#?}", item);
    }

    Ok(())
}
