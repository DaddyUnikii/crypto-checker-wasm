use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn get_crypto_price(coin: &str) -> f64 {
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={coin}&vs_currencies=usd");
    let res = reqwest::get(&url).await.unwrap();
    let data: serde_json::Value = res.json().await.unwrap();
    data[coin]["usd"].as_f64().unwrap_or(0.0)
}

#[wasm_bindgen(start)]
pub fn run() {
    // Auto-run when WASM loads
}
