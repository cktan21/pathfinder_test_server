mod live_state;
use live_state::process_tycho_stream;

mod block_state;
use block_state::get_block_state;

mod all_tokens;
use all_tokens::all_tokens;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::Deserialize;
use serde_json::{from_str, to_string_pretty};

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

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

#[post("/state/live")]
async fn live(params: web::Json<LiveParams>) -> impl Responder {
    // Set defaults to remove_tvl_threshold=900 and add_tvl_threshold=1000 respectively
    // remove_tvl_threshold => any tokens below that threshold will be removed 
    // add_tvl_threshold=> any tokens above that threshold will be added

    let remove_tvl_threshold = params.remove_tvl_threshold.unwrap_or(900.0);
    let add_tvl_threshold = params.add_tvl_threshold.unwrap_or(1000.0);

    match process_tycho_stream(remove_tvl_threshold, add_tvl_threshold).await {
        Ok(json) => {
            let pretty_json = serde_json::to_string_pretty(&json).unwrap_or_default();
            HttpResponse::Ok().body(pretty_json)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to get live data: {}", e)),
    }
}

#[post("/state")]
async fn state(req_body: String) -> impl Responder {
    let params: Result<BlockParams, _> = serde_json::from_str(&req_body);
    match params {
        Ok(params) => {
            let block_hash = params.block_hash;
            let block_number = params.block_number;

            match get_block_state(block_hash.as_deref(), block_number).await {
                Ok(json_data) => {
                    let json_str = serde_json::to_string_pretty(&json_data).unwrap_or_default();
                    HttpResponse::Ok().body(json_str)
                }
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Invalid request body: {}", e)),
    }
}

#[get("/tokens")]
async fn tokens() -> impl Responder {
    match all_tokens().await {
        Ok(tokens) => {
            let json = serde_json::to_string_pretty(&tokens).unwrap_or_default();
            HttpResponse::Ok().body(json)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        // .service(echo)
        .service(live)
        .service(state)
        .service(tokens)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}