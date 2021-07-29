# scpsl-api
[![crates.io](https://img.shields.io/crates/v/scpsl-api.svg)](https://crates.io/crates/scpsl-api) [![docs.rs](https://docs.rs/scpsl-api/badge.svg)](https://docs.rs/scpsl-api)
---
A SCP: Secret Laboratory API wrapper (see [official API reference](https://api.scpslgame.com)).
## Example
```rust
use scpsl_api::server_info::{get, RequestParameters, Response};
use std::env::var;
use url::Url;

#[tokio::main]
async fn main() {
    let account_id = var("ACCOUNT_ID")
        .expect("Expected an account id in the environment")
        .parse::<u64>()
        .unwrap();
    let api_key = var("API_KEY").expect("Expected an account id in the environment");

    let url = Url::parse("https://api.scpslgame.com/serverinfo.php").unwrap();

    let parameters = RequestParameters::builder()
        .url(&url)
        .id(account_id)
        .key(api_key.as_str())
        .players(true)
        .build();

    if let Response::Success(response) = get(&parameters).await.unwrap() {
        println!(
            "Total players on your servers: {}",
            response
                .servers()
                .iter()
                .map(|server| server.players_count().unwrap().current_players())
                .sum::<u32>()
        )
    }
}
```