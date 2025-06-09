use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

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

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_static("sampletoken"));

    let mut dict_store: HashMap<String, serde_json::Value> = HashMap::new();

    let mut tbr = HashMap::<&'static str, HashMap<String, serde_json::Value>>::new();

    let proc_system_res = client
        .post("https://tycho-beta.propellerheads.xyz/v1/protocol_systems")
        .headers(headers.clone())
        .json(&json!({ "chain": "ethereum" }))
        .send()
        .await?;

    let proc_systems: ProtocolSystemsResponse = proc_system_res.json().await?;

    for system in proc_systems.protocol_systems {
        let state_res = client
            .post("https://tycho-beta.propellerheads.xyz/v1/protocol_state")
            .headers(headers.clone())
            .json(&json!({
                "protocol_system": system,
                "version": {
                    "block": {
                        "chain": "ethereum",
                        "hash": bloc_hash,
                        "number": bloc_no
                    }
                }
            }))
            .send()
            .await?;

        let state_data: ProtocolStateResponse = match state_res.json().await {
            Ok(data) => data,
            Err(_) => continue, // skip on error
        };

        let mut snapshot_states = serde_json::Map::new();

        for state in state_data.states {
            // if let Some(liquidity) = state.attributes.get("liquidity") {
            //     if liquidity == "0x00" {
            //         continue;
            //     }

                let component_res = client
                    .post("https://tycho-beta.propellerheads.xyz/v1/protocol_components")
                    .headers(headers.clone())
                    .json(&json!({
                        "chain": "ethereum",
                        "protocol_system": system,
                        "component_ids": [state.component_id]
                    }))
                    .send()
                    .await?;

                let component_data: ComponentResponse = match component_res.json().await {
                    Ok(data) => data,
                    Err(_) => continue,
                };

                let mut entry = serde_json::Map::new();
                entry.insert("state".to_string(), serde_json::to_value(state)?);
                entry.insert("component".to_string(), serde_json::to_value(component_data.protocol_components)?);

                snapshot_states.insert(entry["state"]["component_id"].as_str().unwrap().to_string(), serde_json::Value::Object(entry));
            // }
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

        dict_store.insert(system, protocol_entry);
    }

    tbr.insert("state_msgs", dict_store);

    // Optionally Save to JSON
    let json_output = serde_json::to_string_pretty(&tbr)?;
    let mut file = File::create(format!("./json/state_outputs/liquidity_state_data_{bloc_no}.json"))?;
    file.write_all(json_output.as_bytes())?;

    Ok(serde_json::to_value(tbr)?)
}
