use hmac::{Hmac, Mac};
use reqwest::header::HeaderValue;
use reqwest::{header::HeaderMap, Client, Url};
use serde_json::Value;
use sha2::Sha256;
use std::time::SystemTime;

pub struct Binance {
    pub client: Client,
    pub base_url: Url,
}

impl Binance {
    pub fn new() -> Self {
        let base_url = Url::parse("https://testnet.binancefuture.com").unwrap();
        let client = Client::new();

        Self { base_url, client }
    }

    pub async fn new_buy_order(self, _symbol: String) -> Result<(), Box<dyn std::error::Error>> {
        let client = self.client;
        let mut url = self.base_url;
        let mut headers = HeaderMap::new();

        let since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let timestamp =
            since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000;

        let api_key = HeaderValue::from_static("");

        url.set_path("/fapi/v1/order");

        headers.insert("x-mbx-apikey", api_key);

        let mut signed_key = Hmac::<Sha256>::new_from_slice(b"").unwrap();
        let query = format!(
            "timestamp={}&side=BUY&symbol=BTCUSDT&quantity=1&type=MARKET",
            timestamp
        );
        signed_key.update(query.as_bytes());
        let signature = hex::encode(signed_key.finalize().into_bytes());

        url.set_query(Some(&format!("{}&signature={}", query, signature).as_str()));
        dbg!(url.query());

        let response = client
            .post(url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        let parsed_response: Value = serde_json::from_str(response.as_str())?;
        println!("{:#?}", parsed_response);

        Ok(())
    }
}
