use std::{net::SocketAddr, sync::Arc};
use axum::{
    extract::State,
    http::Method,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    total: Arc<Mutex<u64>>,
    yes: Arc<Mutex<u64>>,
    no: Arc<Mutex<u64>>,
}

#[derive(Deserialize)]
struct BetRequest {
    choice: String,
    amount: u64,
}

#[derive(Serialize)]
struct MarketResponse {
    market_id: String,
    total_pool: u64,
    yes_pool: u64,
    no_pool: u64,
    contract_id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🥊 ========================================================");
    println!("🤖 MIDEN ARENA REAL-TIME EVENT RELAYER (PORT: 8080)");
    println!("📡 Waiting for real user predictions from Frontend UI...");
    println!("========================================================\n");

    let state = AppState {
        total: Arc::new(Mutex::new(20)),
        yes: Arc::new(Mutex::new(20)),
        no: Arc::new(Mutex::new(0)),
    };

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/market", get(get_market))
        .route("/submit", post(submit_prediction))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("🚀 Relayer server online at http://127.0.0.1:8080");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_market(State(state): State<AppState>) -> impl IntoResponse {
    let total = *state.total.lock().await;
    let yes = *state.yes.lock().await;
    let no = *state.no.lock().await;

    Json(MarketResponse {
        market_id: "0x00000000000000000000000000000001".to_string(),
        total_pool: total,
        yes_pool: yes,
        no_pool: no,
        contract_id: "0x4fd1531ea602bd513c5b87df3d8332".to_string(),
    })
}

async fn submit_prediction(
    State(state): State<AppState>,
    Json(payload): Json<BetRequest>,
) -> impl IntoResponse {
    let mut total = state.total.lock().await;
    let mut yes = state.yes.lock().await;
    let mut no = state.no.lock().await;

    let bet_amount = if payload.amount > 0 { payload.amount } else { 10 };

    *total += bet_amount;
    if payload.choice.to_uppercase() == "NO" {
        *no += bet_amount;
    } else {
        *yes += bet_amount;
    }

    println!(
        "⚡ [AUTO-RELAYER] New Note Consumed! Choice: {} | Amount: {} SKS -> NEW STATE: Total={} SKS | YES={} SKS | NO={} SKS",
        payload.choice, bet_amount, *total, *yes, *no
    );

    Json(MarketResponse {
        market_id: "0x00000000000000000000000000000001".to_string(),
        total_pool: *total,
        yes_pool: *yes,
        no_pool: *no,
        contract_id: "0x4fd1531ea602bd513c5b87df3d8332".to_string(),
    })
}
