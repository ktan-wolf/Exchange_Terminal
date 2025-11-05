use axum::{
    Router,
    extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(handle_ws));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("✅ Web3 Terminal backend running at ws://{addr}/ws");

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handle_ws(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let msg = Message::Text(Utf8Bytes::from_static("Hello, from Axum!"));

    if socket.send(msg).await.is_err() {
        println!("client disconnected before first message");
        return;
    }

    // Echo any incoming messages (for quick testing)
    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(txt) = msg {
            println!("Received from client: {txt}");
        }
    }
}
