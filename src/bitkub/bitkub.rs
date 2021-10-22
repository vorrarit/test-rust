use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use reqwest::Response;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Ticker {
    pub last: f32
}

pub async fn exchange_get_tickers() -> Result<HashMap<String, Ticker>, reqwest::Error> {
    reqwest::get("https://api.bitkub.com/api/market/ticker").await?.json::<HashMap<String, Ticker>>().await

}