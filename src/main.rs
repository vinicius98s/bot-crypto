mod binance;

use crate::binance::Binance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binance_client = Binance::new();

    binance_client.new_buy_order("BTCUSDT".into()).await?;

    Ok(())
}
