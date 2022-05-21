use crate::utils::get_env;
use serde::Deserialize;
use std::collections::HashMap;

pub async fn send_get_request(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    Ok(body)
}

#[derive(Deserialize, Debug)]
struct LocalBlockNumber {
    // jsonrpc: String,
    // id: String,
    result: String,
}

pub async fn get_local_block_hash() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    // Polygon node: "http://localhost:8545/"
    let json_rpc_url = get_env("JSON_RPC_URL");

    let mut map = HashMap::new();
    map.insert("jsonrpc", "2.0");
    map.insert("method", "eth_blockNumber");
    map.insert("id", "0");

    let res = client.post(json_rpc_url).json(&map).send().await?;
    let json: LocalBlockNumber = res.json().await?;
    Ok(json.result)
}

#[cfg(test)]
mod test {
    use crate::request;
    use crate::utils::get_env;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_send_get_request() {
        dotenv().ok();
        let url = format!("{}block/number", get_env("EXPRESS_SERVER_URL"));
        let res = request::send_get_request(&url).await;
        assert_eq!(res.unwrap().parse::<u64>().unwrap() > 0, true);
    }

    #[tokio::test]
    async fn test_get_local_block_hash() {
        dotenv().ok();
        let res = request::get_local_block_hash().await;
        assert_ne!(&res.unwrap(), "0x");
    }
}
