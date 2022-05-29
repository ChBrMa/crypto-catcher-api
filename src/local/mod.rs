use reqwest::Error;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct StatusCrypto {
    pub gecko_says: String,
}

pub async fn status() -> Result<StatusCrypto, Error> {
    let status: StatusCrypto = reqwest::Client::new()
        .get("https://api.coingecko.com/api/v3/ping")
        .send()
        .await?
        .json()
        .await?;
    
    
    Ok(status)
}