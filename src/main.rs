use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{Json, Router, extract::State, routing::get};

struct AppState {
    counter: Mutex<u32>,
}

async fn increment(State(data): State<Arc<AppState>>) -> Json<u32> {
    let mut state = data.counter.lock().await;
    *state += 1;

    Json(*state)
}

async fn decrement(State(data): State<Arc<AppState>>) -> Json<u32> {
    let mut state = data.counter.lock().await;

    if *state <= 0 {
        *state = 0;
    } else {
        *state -= 1;
    }

    Json(*state)
}

async fn get_counter(State(data): State<Arc<AppState>>) -> Json<u32> {
    let state = data.counter.lock().await;
    Json(*state)
}

#[tokio::main]

async fn main() {
    let state = Arc::new(AppState {
        counter: Mutex::new(0),
    });
    let app = Router::new()
        .route("/increment", get(increment))
        .route("/decrement", get(decrement))
        .route("/counter", get(get_counter))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

/*
Mutex: Only one thread access the data at a time
Arc: Atomic reference counter, It help to share the ownership of same data
State:

*/
