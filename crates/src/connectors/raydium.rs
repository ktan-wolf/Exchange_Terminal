use super::state::PriceUpdate;
use futures_util::StreamExt;
use serde::Deserialize;
use tokio::sync::broadcast::Sender;
use tokio_tungstenite::connect_async;

#[derive(Debug, Deserialize)]
struct PythPriceMsg {
    id: String,

    #[serde(rename = "price")]
    price: f64,

    #[serde(rename = "conf")]
    confidence: f64,

    #[serde(rename = "ts")]
    timestamp: u64,
}

pub async fn run_raydium_connector(tx: Sender<PriceUpdate>) {
    // Pyth price feed for SOL/USD
    let sol_feed = "J83w4HKfqZz8Q4yPDR5iXe9pnKtxY8a9oJG6p7f3XaCg";

    let url = format!("wss://hermes.pyth.network/v2/updates?ids[]={}", sol_feed);

    println!("[Raydium/Pyth] connecting : {}", url);

    match connect_async(&url).await {
        Ok((mut ws_stream, _)) => {
            println!("[Raydium/Pyth] ✅ Connected");

            while let Some(msg) = ws_stream.next().await {
                if let Ok(msg) = msg {
                    if msg.is_text() {
                        let text = msg.to_text().unwrap();

                        if let Ok(parsed) = serde_json::from_str::<Vec<PythPriceMsg>>(text) {
                            // Pyth sends a vector, we take the first element
                            if let Some(price_msg) = parsed.first() {
                                let _ = tx.send(PriceUpdate {
                                    source: "Raydium".to_string(),
                                    pair: "SOL/USDT".to_string(),
                                    price: price_msg.price,
                                });
                            }
                        }
                    }
                }
            }
        }

        Err(e) => eprintln!("[Raydium/Pyth] ❌ Connection error: {:?}", e),
    }
}

