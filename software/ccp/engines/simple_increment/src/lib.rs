use wasm_bindgen::prelude::*;
use tokio::runtime::Runtime;
use warp::ws::{Message, WebSocket};
use warp::Filter;
use futures::{FutureExt, StreamExt};

#[wasm_bindgen]
pub struct IncrementEngine {
    current_value: i32,
}

#[wasm_bindgen]
impl IncrementEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> IncrementEngine {
        IncrementEngine { current_value: 0 }
    }

    pub fn set_value(&mut self, value: i32) {
        self.current_value = value;
    }

    pub fn get_value(&self) -> i32 {
        self.current_value
    }

    pub fn increment(&mut self) {
        self.current_value += 1;
    }
}

#[tokio::main]
async fn main() {
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(handle_socket)
        });

    warp::serve(ws_route).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_socket(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();

    while let Some(result) = rx.next().await {
        match result {
            Ok(msg) => {
                if let Ok(text) = msg.to_str() {
                    println!("Received: {}", text);
                    // Interact with the WASM component here
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
}

