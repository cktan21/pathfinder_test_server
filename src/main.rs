/* main.rs */

mod live_state;
use live_state::process_tycho_stream;

mod block_state;
use block_state::get_block_state;

mod all_tokens;
use all_tokens::all_tokens;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use tracing::{info, error};
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Deserialize)]
struct BlockParams {
    block_hash: Option<String>,
    block_number: Option<u64>,
}

#[derive(Deserialize)]
pub struct LiveParams {
    remove_tvl_threshold: Option<f64>,
    add_tvl_threshold: Option<f64>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("yes Im alive")
}

#[post("/state/live")]
async fn live(params: web::Json<LiveParams>) -> impl Responder {
    let remove_tvl_threshold = params.remove_tvl_threshold.unwrap_or(900.0);
    let add_tvl_threshold = params.add_tvl_threshold.unwrap_or(1000.0);

    info!("Received /state/live request with remove_tvl_threshold={} and add_tvl_threshold={}", remove_tvl_threshold, add_tvl_threshold);

    match process_tycho_stream(remove_tvl_threshold, add_tvl_threshold).await {
        Ok(json) => {
            let pretty_json = serde_json::to_string_pretty(&json).unwrap_or_default();
            HttpResponse::Ok().body(pretty_json)
        }
        Err(e) => {
            error!("Failed to get live data: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to get live data: {}", e))
        }
    }
}

#[post("/state")]
async fn state(req_body: String) -> impl Responder {
    info!("Received /state request with body: {}", req_body);
    let params: Result<BlockParams, _> = serde_json::from_str(&req_body);

    match params {
        Ok(params) => {
            let block_hash = params.block_hash;
            let block_number = params.block_number;
            info!("Calling get_block_state with hash={:?}, number={:?}", block_hash, block_number);

            match get_block_state(block_hash.as_deref(), block_number).await {
                Ok(json_data) => {
                    let json_str = serde_json::to_string_pretty(&json_data).unwrap_or_default();
                    HttpResponse::Ok().body(json_str)
                }
                Err(e) => {
                    error!("get_block_state error: {}", e);
                    HttpResponse::InternalServerError().body(format!("Error: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Invalid request body: {}", e);
            HttpResponse::BadRequest().body(format!("Invalid request body: {}", e))
        }
    }
}

#[get("/tokens")]
async fn tokens() -> impl Responder {
    info!("Received /tokens request");
    match all_tokens().await {
        Ok(tokens) => {
            let json = serde_json::to_string_pretty(&tokens).unwrap_or_default();
            HttpResponse::Ok().body(json)
        }
        Err(e) => {
            error!("Failed to get tokens: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logging setup with file output
    let file_appender = tracing_appender::rolling::hourly("../logs/22694045", "server.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::fmt()
        .json() //formmated to JSON
        .with_writer(non_blocking)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting HTTP server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(live)
            .service(state)
            .service(tokens)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}