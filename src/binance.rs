use hmac::{Hmac, Mac};
use reqwest::header::HeaderValue;
use reqwest::{header::HeaderMap, Client, Url};
use serde_json::Value;
use sha2::Sha256;
use std::error::Error;
use std::time::SystemTime;

use crate::settings::Settings;

#[derive(Debug)]
pub struct Position {
    asset: String,
    value: f64,
}

pub struct Binance {
    pub base_url: Url,
    pub tracked_assets: Vec<String>,
    api_key: String,
    secret_key: String,
}

impl Binance {
    pub fn new() -> Self {
        let base_url = Url::parse("https:testnet.binance.vision").unwrap();

        let settings = match Settings::default() {
            Ok(result) => result,
            Err(e) => panic!("Failed to get settings: {}", e),
        };

        Self {
            base_url,
            api_key: settings.api_key,
            secret_key: settings.secret_key,
            tracked_assets: settings.tracked_assets,
        }
    }

    fn signed_request_headers(&self) -> Result<HeaderMap, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        let api_key = HeaderValue::from_str(self.api_key.as_str())?;
        headers.insert("x-mbx-apikey", api_key);

        Ok(headers)
    }

    fn signed_request_query(&self) -> Result<String, Box<dyn Error>> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let mut signed_key = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes())?;

        let query = format!("timestamp={}", timestamp);

        signed_key.update(query.as_bytes());
        let signature = hex::encode(signed_key.finalize().into_bytes());

        Ok(format!("{}&signature={}", query, signature))
    }

    pub async fn get_positions(&self) -> Result<Vec<Position>, Box<dyn Error>> {
        let mut url = self.base_url.to_owned();
        let headers = self.signed_request_headers()?;
        let query = self.signed_request_query()?;

        let client = Client::new();

        url.set_path("/api/v3/account");
        url.set_query(Some(query.as_str()));

        let response = client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        let parsed_response: Value = serde_json::from_str(response.as_str())?;
        match parsed_response["balances"].as_array() {
            Some(balances) => Ok(balances
                .iter()
                .map(|value| {
                    let mut asset = value["asset"].to_string();
                    asset.retain(|c| c != '"');

                    Position {
                        asset,
                        value: value["free"].as_str().unwrap().parse::<f64>().unwrap(),
                    }
                })
                .filter(|position| self.tracked_assets.contains(&position.asset))
                .collect::<Vec<Position>>()),
            None => Err("Failed to get account balances".into()),
        }
    }
}
