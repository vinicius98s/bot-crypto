use reqwest::{Client, Url};

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
}
