# How to Use

## Local Deployement

- runs on http://localhost:8080/

```bash
cargo run

## if you want the tycho-simulation files stored locally
git clone https://github.com/propeller-heads/tycho-simulation.git
cd tycho_simulation
cargo build
cd ../data_ingestor_json_retreival

## Alternatively if you want it referenced via github
## uncomment the # tycho-simulation = { git = "https://github.com/propeller-heads/tycho-simulation.git",  branch = "main" } line
```

# Endpoints

## GET (`/tokens`)
- Returns all tokens currently present in the network (ETH)

> Sample Response

```json
{
    "0x34bff799359519e6c18b4a97995244e7d7170919": {
        "address": "0x34bff799359519e6c18b4a97995244e7d7170919",
        "decimals": 9,
        "symbol": "NINA",
        "gas": [
            55612
        ]
    },
    "0xb1373733e161e7aa0eed3c9ca0549e200a977fa4": {
        "address": "0xb1373733e161e7aa0eed3c9ca0549e200a977fa4",
        "decimals": 18,
        "symbol": "OMNIB",
        "gas": [
            29938
        ]
    },
    // and more
}
```

> full response in tokens.json

## POST (`/state`)
- returns the specified state of the blockchain network
- returns the `"state_msgs"` component directly

> Sample Input 
```json
{
    "block_hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
    "block_number": 22637843
}
```

> Sample Response

```json
{
    "pancakeswap_v3" {
        "header": {
            "hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
            "number": 22650031,
            "parent_hash": "0x6746f2d5ecd7ee187a733debc928ca9bb7ebdccb734ea9e40886ee5b4c0b4cbe",
            "revert": false
        },
        "snapshots": {
            "states": {
                "0x8c1c499b1796d7f3c2521ac37186b52de024e58c": {
                    "state": {
                        "component_id": "0x8c1c499b1796d7f3c2521ac37186b52de024e58c",
                        "attributes": {
                            "reserve1": "0x016587d64c88",
                            "reserve0": "0x01d82cc77873129b4d3330"
                        },
                        "balances": {
                            "0x31c8eacbffdd875c74b94b077895bd78cf1e64a3": "0x01d82cc77873129b4d3330",
                            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48": "0x016587d64c88"
                        }
                    },
                    "component": {
                        "id": "0x8c1c499b1796d7f3c2521ac37186b52de024e58c",
                        "protocol_system": "uniswap_v2",
                        "protocol_type_name": "uniswap_v2_pool",
                        "chain": "ethereum",
                        "tokens": [
                            "0x31c8eacbffdd875c74b94b077895bd78cf1e64a3",
                            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
                        ],
                        "contract_ids": [],
                        "static_attributes": {
                            "pool_address": "0x8c1c499b1796d7f3c2521ac37186b52de024e58c",
                            "fee": "0x1e"
                        },
                        "change": "Creation",
                        "creation_tx": "0x6a28c3a497c1c414b8b5805e417224d51c2d76291b7dca28fa0ec9c3e38449a0",
                        "created_at": "2021-03-18T09:43:21"
                    },
                    "component_tvl": null
                },
            },
            // AND more
        }
    }
}
```

> full response in liquidity_data.json

## POST (`/state/live`)

- returns the live state of the blockchain network
- allows optional tvl specification
  - ``remove_tvl_threshold`` => any tokens below that threshold will be removed (defaults to 900.0)
  - ``add_tvl_threshold``=> any tokens above that threshold will be added (defaults to 1000.0)
- if no tvl specification, empty json is still required ie `{}`

> Sample input

```json
{
    "remove_tvl_threshold": 800.0,
    "add_tvl_threshold": 1200.0
}
```

> Sample Output

```json
{
    "state_msg" {
        "pancakeswap_v3" {
            "header": {
                "hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
                "number": 22650031,
                "parent_hash": "0x6746f2d5ecd7ee187a733debc928ca9bb7ebdccb734ea9e40886ee5b4c0b4cbe",
                "revert": false
            },
            "snapshots": {
                "states": {
                    "0x8c1c499b1796d7f3c2521ac37186b52de024e58c": {
                        "state": {
                            "component_id": "0x8c1c499b1796d7f3c2521ac37186b52de024e58c",
                            "attributes": {
                                "reserve1": "0x016587d64c88",
                                "reserve0": "0x01d82cc77873129b4d3330"
                            },
                            "balances": {
                                "0x31c8eacbffdd875c74b94b077895bd78cf1e64a3": "0x01d82cc77873129b4d3330",
                                "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48": "0x016587d64c88"
                            }
                        },
                        "component": {
                            "id": "0x8c1c499b1796d7f3c2521ac37186b52de024e58c",
                            "protocol_system": "uniswap_v2",
                            "protocol_type_name": "uniswap_v2_pool",
                            "chain": "ethereum",
                            "tokens": [
                                "0x31c8eacbffdd875c74b94b077895bd78cf1e64a3",
                                "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
                            ],
                            "contract_ids": [],
                            "static_attributes": {
                                "pool_address": "0x8c1c499b1796d7f3c2521ac37186b52de024e58c",
                                "fee": "0x1e"
                            },
                            "change": "Creation",
                            "creation_tx": "0x6a28c3a497c1c414b8b5805e417224d51c2d76291b7dca28fa0ec9c3e38449a0",
                            "created_at": "2021-03-18T09:43:21"
                        },
                        "component_tvl": null
                    },
                },
                // AND more
            },
            "deltas": {
                "extractor": "uniswap_v2",
                "chain": "ethereum",
                "block": {
                    "number": 22650031,
                    "hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
                    "parent_hash": "0x6746f2d5ecd7ee187a733debc928ca9bb7ebdccb734ea9e40886ee5b4c0b4cbe",
                    "chain": "ethereum",
                    "ts": "2025-06-07T03:30:11"
                },
                "finalized_block_height": 22649958,
                "revert": false,
                "new_tokens": {},
                "account_updates": {},
                "state_updates": {
                    "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852": {
                        "component_id": "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852",
                        "updated_attributes": {
                            "reserve1": "0x1241d599aff7",
                            "reserve0": "0x01b5e16ec49e87f7b57e"
                        },
                        "deleted_attributes": []
                    },
                    // and more
                },
                "new_protocol_components": {},
                "deleted_protocol_components": {},
                "component_balances": {
                    "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852": {
                        "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2": {
                            "token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
                            "balance": "0x01b5e16ec49e87f7b57e",
                            "balance_float": 8.077471297302211e21,
                            "modify_tx": "0xaa3b2a60d0664be84e019c9a12aa72fd064fc3c253015b9a219c4036991da027",
                            "component_id": "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852"
                        },
                        "0xdac17f958d2ee523a2206206994597c13d831ec7": {
                            "token": "0xdac17f958d2ee523a2206206994597c13d831ec7",
                            "balance": "0x1241d599aff7",
                            "balance_float": 20073965793271.0,
                            "modify_tx": "0xaa3b2a60d0664be84e019c9a12aa72fd064fc3c253015b9a219c4036991da027",
                            "component_id": "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852"
                        }
                    },
                    // And more
                },
                "account_balances": {},
                "component_tvl": {
                    "0xc555d55279023e732ccd32d812114caf5838fd46": 2804.756282460753,
                    "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852": 16174.388501250103
                }
            },
            "removed_components": {}
        }
    },
    "sync_states": {
        "uniswap_v3": {
            "status": "ready",
            "hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
            "number": 22650031,
            "parent_hash": "0x6746f2d5ecd7ee187a733debc928ca9bb7ebdccb734ea9e40886ee5b4c0b4cbe",
            "revert": false
        },
        "pancakeswap_v2": {
            "status": "ready",
            "hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
            "number": 22650031,
            "parent_hash": "0x6746f2d5ecd7ee187a733debc928ca9bb7ebdccb734ea9e40886ee5b4c0b4cbe",
            "revert": false
        },
        "ekubo_v2": {
            "status": "ready",
            "hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
            "number": 22650031,
            "parent_hash": "0x6746f2d5ecd7ee187a733debc928ca9bb7ebdccb734ea9e40886ee5b4c0b4cbe",
            "revert": false
        },
        "sushiswap_v2": {
            "status": "ready",
            "hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
            "number": 22650031,
            "parent_hash": "0x6746f2d5ecd7ee187a733debc928ca9bb7ebdccb734ea9e40886ee5b4c0b4cbe",
            "revert": false
        },
        "vm:balancer_v2": {
            "status": "ready",
            "hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
            "number": 22650031,
            "parent_hash": "0x6746f2d5ecd7ee187a733debc928ca9bb7ebdccb734ea9e40886ee5b4c0b4cbe",
            "revert": false
        },
        "pancakeswap_v3": {
            "status": "ready",
            "hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
            "number": 22650031,
            "parent_hash": "0x6746f2d5ecd7ee187a733debc928ca9bb7ebdccb734ea9e40886ee5b4c0b4cbe",
            "revert": false
        },
        "uniswap_v4": {
            "status": "ready",
            "hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
            "number": 22650031,
            "parent_hash": "0x6746f2d5ecd7ee187a733debc928ca9bb7ebdccb734ea9e40886ee5b4c0b4cbe",
            "revert": false
        },
        "uniswap_v2": {
            "status": "ready",
            "hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
            "number": 22650031,
            "parent_hash": "0x6746f2d5ecd7ee187a733debc928ca9bb7ebdccb734ea9e40886ee5b4c0b4cbe",
            "revert": false
        },
        "vm:curve": {
            "status": "ready",
            "hash": "0x5f75ec3a80c85e0f3b61c521e031dfd92fe8e12e34254aff19c73bfda5a07e62",
            "number": 22650031,
            "parent_hash": "0x6746f2d5ecd7ee187a733debc928ca9bb7ebdccb734ea9e40886ee5b4c0b4cbe",
            "revert": false
        }
    }     
}

```

> full response in output.json