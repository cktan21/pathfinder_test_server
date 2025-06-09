use std::{env, str::FromStr};

use clap::Parser;
use tycho_simulation::{
    utils::load_all_tokens,
};
use tycho_common::models::Chain;

use dotenv::dotenv;

// use std::fs::File;
// use std::io::BufWriter;
// use serde_json::to_writer;

// use tracing_subscriber::{fmt, EnvFilter};

pub(super) fn get_default_url(chain: &Chain) -> Option<String> {
    match chain {
        Chain::Ethereum => Some("tycho-beta.propellerheads.xyz".to_string()),
        Chain::Base => Some("tycho-base-beta.propellerheads.xyz".to_string()),
        Chain::Unichain => Some("tycho-unichain-beta.propellerheads.xyz".to_string()),
        _ => None,
    }
}

#[derive(Parser)]
struct Cli {
    /// The tvl threshold to filter the graph by
    #[arg(short, long, default_value_t = 1000.0)]
    tvl_threshold: f64,
    /// The target blockchain
    #[clap(long, default_value = "ethereum")]
    pub chain: String,
}

pub async fn all_tokens() -> Result<serde_json::Value, Box<dyn std::error::Error>>  {

    dotenv().ok();

    // Parse command-line arguments into a Cli struct
    let cli = Cli::parse();
    let chain =
        Chain::from_str(&cli.chain).unwrap_or_else(|_| panic!("Unknown chain {}", cli.chain));

    let tycho_url = env::var("TYCHO_URL").unwrap_or_else(|_| {
        get_default_url(&chain).unwrap_or_else(|| panic!("Unknown URL for chain {}", cli.chain))
    });

    let tycho_api_key: String =
        env::var("TYCHO_API_KEY").unwrap_or_else(|_| "sampletoken".to_string());

    // Perform an early check to ensure `RPC_URL` is set.
    // This prevents errors from occurring later during UI interactions.
    // Can be commented out if only using the example with uniswap_v2, uniswap_v3 and balancer_v2.
    
    env::var("RPC_URL").expect("RPC_URL env variable should be set");

    let all_tokens = load_all_tokens(
            tycho_url.as_str(),
            false,
            Some(tycho_api_key.as_str()),
            chain,
            None,
            None,
        ).await;

    // Optionally Save to JSON
    // let file = File::create("./json/tokens.json").expect("Failed to create tokens.json");
    // let writer = BufWriter::new(file);
    // to_writer(writer, &all_tokens).expect("Failed to serialize tokens");

    // println!("Tokens saved to tokens.json");

    // Serialize the HashMap into a serde_json::Value
    let tokens_json = serde_json::to_value(all_tokens)?;

    Ok(tokens_json)
}
