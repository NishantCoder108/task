use std::sync::{Arc, Mutex};

use axum::{Json, Router, extract::State, routing::get};
use serde::de::value::Error;

struct AppState {
    counter: Mutex<u32>,
}

async fn increment(State(data): State<Arc<AppState>>) -> Json<u32> {
    let mut state = data.counter.lock().unwrap();
    *state += 1;

    Json(*state)
}

#[tokio::main]

async fn main() {
    let state = Arc::new(AppState {
        counter: Mutex::new(0),
    });
    let app = Router::new()
        .route("/increment", get(increment))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

/*
Mutex: Only one thread access the data at a time
Arc: Atomic reference counter, It help to share the ownership of same data
State:

*/
