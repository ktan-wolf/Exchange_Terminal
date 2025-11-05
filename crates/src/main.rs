use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use futures_util::SinkExt;
use std::net::SocketAddr;
use tokio::{net::TcpListener, sync::broadcast};

mod connectors;
use connectors::binance::{PriceUpdate, run_binance_connector};

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel::<PriceUpdate>(100);

    // spawn Binance connector
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        run_binance_connector(tx_clone).await;
    });

    // WebSocket route
    let app = Router::new().route(
        "/ws",
        get({
            let tx = tx.clone();
            move |ws: WebSocketUpgrade| ws_handler(ws, tx.clone())
        }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("✅ Web3 Terminal backend running at ws://{addr}/ws");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, tx: broadcast::Sender<PriceUpdate>) -> impl IntoResponse {
    let rx = tx.subscribe();
    ws.on_upgrade(move |socket| handle_socket(socket, rx))
}

async fn handle_socket(mut socket: WebSocket, mut rx: broadcast::Receiver<PriceUpdate>) {
    println!("⚡ Client connected");

    while let Ok(update) = rx.recv().await {
        if let Ok(msg) = serde_json::to_string(&update) {
            if socket.send(Message::Text(msg.into())).await.is_err() {
                println!("❌ Client disconnected");
                break;
            }
        }
    }
}

