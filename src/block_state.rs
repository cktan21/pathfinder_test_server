use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use tracing::{info, error, debug};

#[derive(Debug, Deserialize)]
struct ProtocolSystemsResponse {
    protocol_systems: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ProtocolStateResponse {
    states: Vec<State>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct State {
    component_id: String,
    attributes: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    balances: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct ComponentResponse {
    protocol_components: Vec<HashMap<String, serde_json::Value>>,
}

pub async fn get_block_state(
    block_hash: Option<&str>,
    block_number: Option<u64>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = Client::new();

    let bloc_hash = block_hash.unwrap_or("0x878ccb82e46332081d32b7e2c9c81976a4cd8dcefe327ef6e6432460527ae272");
    let bloc_no = block_number.unwrap_or(22637843);

    info!("Fetching block state at hash={}, number={}", bloc_hash, bloc_no);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_static("7cdb5b0f-7ab7-4162-8d77-e142819f2144"));

    let mut dict_store: HashMap<String, serde_json::Value> = HashMap::new();
    let mut tbr = HashMap::<&'static str, HashMap<String, serde_json::Value>>::new();

    let proc_system_res = client
        .post("https://tycho-beta.propellerheads.xyz/v1/protocol_systems")
        .headers(headers.clone())
        .json(&json!({ "chain": "ethereum" }))
        .send()
        .await?;

    let proc_systems: ProtocolSystemsResponse = proc_system_res.json().await?;
    info!("Received {} protocol systems", proc_systems.protocol_systems.len());

    let mut tasks = FuturesUnordered::new();
    for system in proc_systems.protocol_systems {
        if system == "balancer_v3" {
            debug!("Skipping system: balancer_v3");
            continue;
        }

        let client = client.clone();
        let headers = headers.clone();
        let system = system.clone();
        let bloc_hash = bloc_hash.to_string();

        tasks.push(tokio::spawn(async move {
            let mut snapshot_states = serde_json::Map::new();
            let mut n = 1;

            loop {
                let state_res = client
                    .post("https://tycho-beta.propellerheads.xyz/v1/protocol_state")
                    .headers(headers.clone())
                    .json(&json!({
                        "protocol_system": &system,
                        "version": {
                            "block": {
                                "chain": "ethereum",
                                "hash": bloc_hash,
                                "number": bloc_no
                            }
                        },
                        "pagination": {
                            "page": n,
                            "page_size": 100
                        }
                    }))
                    .send()
                    .await;

                let state_data: ProtocolStateResponse = match state_res {
                    Ok(res) => match res.json().await {
                        Ok(data) => data,
                        Err(e) => {
                            error!("Decode state JSON failed for system={} page={}: {}", system, n, e);
                            continue;
                        }
                    },
                    Err(e) => {
                        error!("Fetch state failed for system={} page={}: {}", system, n, e);
                        continue;
                    }
                };

                if state_data.states.is_empty() {
                    break;
                }

                let mut component_futures = FuturesUnordered::new();
                for state in state_data.states {
                    let state_clone = state.clone();
                    let client = client.clone();
                    let headers = headers.clone();
                    let system = system.clone();

                    component_futures.push(async move {
                        let component_res = client
                            .post("https://tycho-beta.propellerheads.xyz/v1/protocol_components")
                            .headers(headers.clone())
                            .json(&json!({
                                "chain": "ethereum",
                                "protocol_system": system,
                                "component_ids": [state_clone.component_id.clone()]
                            }))
                            .send()
                            .await;

                        match component_res {
                            Ok(resp) => match resp.json::<ComponentResponse>().await {
                                Ok(component_data) => {
                                    let mut entry = serde_json::Map::new();
                                    entry.insert("state".to_string(), serde_json::to_value(state_clone).unwrap());
                                    entry.insert("component".to_string(), serde_json::to_value(component_data.protocol_components).unwrap());
                                    debug!("Decoded component JSON successfully");

                                    Some((entry["state"]["component_id"].as_str().unwrap().to_string(), serde_json::Value::Object(entry)))
                                }
                                Err(e) => {
                                    error!("Decode component JSON failed for component={}: {}", state_clone.component_id, e);
                                    None
                                }
                            },
                            Err(e) => {
                                error!("Fetch component failed for component={}: {}", state_clone.component_id, e);
                                None
                            }
                        }
                    });
                }

                while let Some(result) = component_futures.next().await {
                    if let Some((key, value)) = result {
                        snapshot_states.insert(key, value);
                    }
                }

                n += 1;
            }

            let protocol_entry = json!({
                "headers": {
                    "hash": bloc_hash,
                    "number": bloc_no,
                    "revert": false,
                    "parent_hash": "",
                },
                "snapshots": {
                    "states": snapshot_states
                }
            });

            Some((system, protocol_entry))
        }));
    }

    while let Some(res) = tasks.next().await {
        if let Ok(Some((system, entry))) = res {
            dict_store.insert(system, entry);
        }
    }

    tbr.insert("state_msgs", dict_store);

    let json_output = serde_json::to_string_pretty(&tbr)?;
    let mut file = File::create(format!("./json/states/liquidity_state_data_{}.json", bloc_no))?;
    file.write_all(json_output.as_bytes())?;

    info!("Saved block state to ./json/states/liquidity_state_data_{}.json", bloc_no);

    Ok(serde_json::to_value(tbr)?)
}
