use celestia_rpc::{Client, HeaderClient};
use tokio;
use dotenv::dotenv;
use std::env;

async fn get_block_hash(block_number: u64, token: Option<&str>) -> Result<String, String> {
    let api_endpoint = "http://localhost:26658";
    eprintln!("Using API endpoint: {}", api_endpoint);

    let client = Client::new(api_endpoint, token).await.map_err(|e| {
        eprintln!("Error creating client: {}", e);
        e.to_string()
    })?;

    eprintln!("Attempting to fetch block header for block number: {}", block_number);

    match client.header_get_by_height(block_number).await {
        Ok(header) => {
            let hash_bytes = header.hash();
            let hex_hash = hex::encode(hash_bytes);
            println!("Received header: {:?}", header);
            println!("Block hash: {}", hex_hash);
            Ok(hex_hash)
        },
        Err(e) => {
            eprintln!("Failed to retrieve block header for block number {}: {}", block_number, e);
            Err(format!("Failed to retrieve block header: {}", e))
        },
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("AUTH_TOKEN").ok();

    let block_number = 1u64;
    match get_block_hash(block_number, token.as_deref()).await {
        Ok(block_hash) => println!("Block Hash for block number {}: {}", block_number, block_hash),
        Err(e) => eprintln!("Error for block number {}: {}", block_number, e),
    }
}
