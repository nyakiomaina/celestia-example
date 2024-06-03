use celestia_rpc::{Client, HeaderClient};
use tokio;

async fn get_block_hash(block_number: u64, token: Option<&str>) -> Result<String, String> {
    let api_endpoint = "https://public-celestia-rpc.numia.xyz";
    let client = Client::new(api_endpoint, token).await.map_err(|e| e.to_string())?;

    match client.header_wait_for_height(block_number).await {
        Ok(header) => {
            println!("Block hash: {:?}", header.hash());
            Ok(header.hash().to_string())
        },
        Err(e) => {
            println!("Failed to retrieve block hash: {}", e);
            Err(format!("Failed to retrieve block hash: {}", e))
        },
    }
}

#[tokio::main]
async fn main() {
    let token: Option<&str> = None;
    let block_numbers = [1u64];

    for block_number in block_numbers.iter() {
        match get_block_hash(*block_number, token).await {
            Ok(block_hash) => println!("Block Hash for block number {}: {}", block_number, block_hash),
            Err(e) => println!("Error for block number {}: {}", block_number, e),
        }
    }
}
