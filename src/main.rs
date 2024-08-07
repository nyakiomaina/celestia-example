use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::signers::Wallet;
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
    let private_key_hex = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    let cleaned_private_key = if private_key_hex.starts_with("0x") {
        &private_key_hex[2..]
    } else {
        &private_key_hex[..]
    };

    let api_endpoint = "http://celestia-node:26658";
    let anvil_url = "http://anvil:8545";

    let celestia_client = Client::new(api_endpoint, token.as_deref()).await?;
    let provider = Provider::<Http>::try_from(anvil_url)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    println!("Connected to chain with chain id: {}", chain_id);

    let private_key_bytes = decode(cleaned_private_key).expect("Invalid private key format");

    let wallet = Wallet::from_bytes(&private_key_bytes).expect("Invalid private key");
    let wallet = wallet.with_chain_id(chain_id);

    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));

    for block_number in 1..=10 {
        if let Ok(block_hash) = get_block_hash(&celestia_client, block_number).await {
            let calldata = format!("{}{}", hex::encode(block_number.to_be_bytes()), block_hash);
            if let Ok(decoded_calldata) = hex::decode(&calldata) {
                let nonce = client.get_transaction_count(wallet.address(), None).await?;
                let tx = TransactionRequest {
                    from: Some(client.address()),
                    to: Some("0xff00000000000000000000000000000000000010".parse()?),
                    value: Some(0u64.into()),
                    data: Some(decoded_calldata.into()),
                    chain_id: Some(chain_id.into()),
                    nonce: Some(nonce),
                    ..Default::default()
                };
                let tx_hash = client.send_transaction(tx, None).await?;
                println!("Transaction sent for block {}: {:?}", block_number, tx_hash);
            }
        }
    }

    Ok(())
}
