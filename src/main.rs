use reqwest::Error;
use serde::Deserialize;
use tokio;

#[derive(Deserialize, Debug)]
struct Header {
    hash: String,
}

#[derive(Deserialize, Debug)]
struct Block {
    header: Header,
}

#[derive(Deserialize, Debug)]
struct BlockResponse {
    block: Block,
}

async fn get_block_hash(api_endpoint: &str, block_number: u64) -> Result<String, String> {
    let url = format!("{}/block?height={}", api_endpoint, block_number);
    println!("Requesting URL: {}", url);
    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    println!("Response Status: {}", response.status());

    if response.status().is_success() {
        let response_text = response.text().await.map_err(|e| e.to_string())?;
        println!("Response Body: {}", response_text);

        let block_response: BlockResponse = serde_json::from_str(&response_text).map_err(|e| e.to_string())?;
        Ok(block_response.block.header.hash)
    } else {
        Err(format!("Failed to retrieve block hash: {}", response.status()))
    }
}

#[tokio::main]
async fn main() {
    let api_endpoints = [
        "https://public-celestia-rpc.numia.xyz",
        "https://celestia-rpc.spidey.services",
        "https://rpc-celestia.contributiondao.com",
        "https://celestia.rpc.stakin-nodes.com",
        "https://celestia.rpc.archives.validao.xyz",
        "https://rpc-archive.celestia.bitszn.com",
        "https://celestia-rpc.noders.services"
    ];

    for api_endpoint in api_endpoints.iter() {
        for block_number in [0u64].iter() {
            match get_block_hash(api_endpoint, *block_number).await {
                Ok(block_hash) => println!("Block Hash for block number {} from {}: {}", block_number, api_endpoint, block_hash),
                Err(e) => println!("Error for block number {} from {}: {}", block_number, api_endpoint, e),
            }
        }
    }
}
