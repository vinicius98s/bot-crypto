mod binance;
mod settings;
use crate::binance::Binance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binance = Binance::new();
    let positions = binance.get_positions().await?;
    dbg!(positions);

    Ok(())
}
