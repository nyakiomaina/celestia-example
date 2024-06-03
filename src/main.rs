use reqwest::Error;
use tokio;

async fn get_block_hash(api_endpoint: &str, block_number: u64) -> Result<String, String> {
    let url = format!("{}/block?height={}", api_endpoint, block_number);
    println!("Requesting URL: {}", url);
    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    println!("Response Status: {}", response.status());

    if response.status().is_success() {
        let response_text = response.text().await.map_err(|e| e.to_string())?;
        println!("Response Body: {}", response_text);
        Ok(response_text)  // Return the raw response text
    } else {
        Err(format!("Failed to retrieve block hash: {}", response.status()))
    }
}

#[tokio::main]
async fn main() {
    let api_endpoints = [
        "https://public-celestia-rpc.numia.xyz/"
    ];

    for api_endpoint in api_endpoints.iter() {
        for block_number in [1u64].iter() {
            match get_block_hash(api_endpoint, *block_number).await {
                Ok(block_hash) => println!("Block Hash for block number {} from {}: {}", block_number, api_endpoint, block_hash),
                Err(e) => println!("Error for block number {} from {}: {}", block_number, api_endpoint, e),
            }
        }
    }
}
