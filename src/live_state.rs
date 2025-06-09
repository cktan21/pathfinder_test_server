use tycho_client::{feed::component_tracker::ComponentFilter, stream::TychoStreamBuilder};
use tycho_common::dto::Chain;
// use std::fs::File;
// use std::io::BufWriter;
// use std::sync::{Arc, Mutex};
// use serde_json::to_writer;

pub async fn process_tycho_stream(
    remove_tvl_threshold: f64,
    add_tvl_threshold: f64,
) -> Result<String, Box<dyn std::error::Error>> {

    // let remove_tvl_threshold = remove_tvl_threshold.unwrap_or(900.0);
    // let add_tvl_threshold = add_tvl_threshold.unwrap_or(1000.0);

    // Create a new Tycho stream for Ethereum blockchain
    let (_, mut receiver) =
        TychoStreamBuilder::new("tycho-beta.propellerheads.xyz", Chain::Ethereum)
            .auth_key(Some("sampletoken".into()))
            .exchange("uniswap_v2", ComponentFilter::with_tvl_range(remove_tvl_threshold, add_tvl_threshold))
            .exchange("uniswap_v3", ComponentFilter::with_tvl_range(remove_tvl_threshold, add_tvl_threshold))
            .exchange("uniswap_v4", ComponentFilter::with_tvl_range(remove_tvl_threshold, add_tvl_threshold))
            .exchange("ekubo_v2", ComponentFilter::with_tvl_range(remove_tvl_threshold, add_tvl_threshold))
            .exchange("pancakeswap_v2", ComponentFilter::with_tvl_range(remove_tvl_threshold, add_tvl_threshold))
            .exchange("pancakeswap_v3", ComponentFilter::with_tvl_range(remove_tvl_threshold, add_tvl_threshold))
            .exchange("sushiswap_v2", ComponentFilter::with_tvl_range(remove_tvl_threshold, add_tvl_threshold))
            .exchange("vm:balancer_v2", ComponentFilter::with_tvl_range(remove_tvl_threshold, add_tvl_threshold))
            .exchange("vm:curve", ComponentFilter::with_tvl_range(remove_tvl_threshold, add_tvl_threshold))
            .build()
            .await?;

    while let Some(msg) = receiver.recv().await {
        // Convert message to JSON string
        let json_msg = serde_json::to_string(&msg)?;

        println!("Received message: {:?}", msg);

        // Optionally Save to JSON
        // // Open a JSON file for writing
        // let file = File::create("./json/output.json")?;
        // let writer = BufWriter::new(file);
        // let writer = Arc::new(Mutex::new(writer));

        // // Write message to JSON
        // let mut writer = writer.lock().unwrap();
        // match to_writer(&mut *writer, &msg) {
        //     Ok(_) => println!("Message written to output.json"),
        //     Err(e) => eprintln!("Failed to write to JSON: {}", e),
        // }

        return Ok(json_msg);
    }

    Err("No message received".into())
}