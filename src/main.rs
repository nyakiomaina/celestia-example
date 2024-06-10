use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::signers::{LocalWallet, SigningKey};
use std::sync::Arc;
use tokio;
use dotenv::dotenv;
use std::env;
use hex::decode;
use celestia_rpc::{Client, HeaderClient};

async fn get_block_hash(client: &Client, block_number: u64) -> Result<String, String> {
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = env::var("AUTH_TOKEN").ok();
    println!("Using AUTH_TOKEN: {:?}", token);

    let api_endpoint = "http://localhost:26658";
    let anvil_url = "http://localhost:8545";

    let celestia_client = Client::new(api_endpoint, token.as_deref()).await?;
    let provider = Provider::<Http>::try_from(anvil_url)?;
    let wallet = LocalWallet::from(
        SigningKey::from_bytes(&decode("key")?).expect("Decoding failed")
    );
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    let block_number = 1u64;
    let block_hash = get_block_hash(&celestia_client, block_number).await?;
    println!("Block Hash for block number {}: {}", block_number, block_hash);

    let tx = TransactionRequest::new()
        .to("0xFF00000000000000000000000000000000000010")
        .value(0u64.into())
        .data(hex::encode(format!("0x{}", block_hash)).into())
        .gas(100000.into());

    let tx_hash = client.send_transaction(tx, None).await?;
    println!("Transaction sent with hash: {:?}", tx_hash);

    Ok(())
}
