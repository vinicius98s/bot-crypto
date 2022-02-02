mod client;

use crate::client::Binance;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binance_client = Binance::new();

    let mut url = binance_client.base_url;
    url.set_path("/fapi/v1/klines");
    url.set_query(Some("symbol=BTCBUSD&interval=1m"));

    let response = binance_client.client.get(url).send().await?.text().await?;

    let parsed_response: Value = serde_json::from_str(response.as_str())?;
    let values: &Vec<Value> = parsed_response.as_array().unwrap();

    println!("{:#?}", values.last());

    Ok(())
}
