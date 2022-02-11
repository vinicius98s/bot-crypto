use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub tracked_assets: Vec<String>,
    pub buying_asset: String,
    pub min_sell_profit: u64,
    pub api_key: String,
    pub secret_key: String,
}

impl Settings {
    pub fn default() -> Result<Settings, Box<dyn std::error::Error>> {
        let mut config = Config::default();
        config.merge(File::with_name("config.yml"))?;

        let settings = config.try_into::<Settings>()?;

        Ok(settings)
    }
}
