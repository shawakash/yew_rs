use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TickerData {
    pub symbol: String,
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: String,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "highPrice")]
    pub high_price: String,
    #[serde(rename = "lowPrice")]
    pub low_price: String,
}

pub async fn get_all_tickers() -> Result<Vec<TickerData>, gloo_net::Error> {
    let response = Request::get("https://api.binance.com/api/v3/ticker/24hr")
        .send()
        .await?
        .json::<Vec<TickerData>>()
        .await?;

    let mut filtered: Vec<TickerData> = response
        .into_iter()
        .filter(|ticker| ticker.symbol.ends_with("USDC") || ticker.symbol.ends_with("USDT"))
        .collect();

    filtered.sort_by(|a, b| {
        let vol_a = a.volume.parse::<f64>().unwrap_or(0.0);
        let vol_b = b.volume.parse::<f64>().unwrap_or(0.0);
        vol_b
            .partial_cmp(&vol_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(filtered.into_iter().take(200).collect())
}

pub async fn get_single_ticker(symbol: &str) -> Result<TickerData, gloo_net::Error> {
    let response = Request::get(&format!(
        "https://api.binance.com/api/v3/ticker/24hr?symbol={}",
        symbol
    ))
    .send()
    .await?
    .json::<TickerData>()
    .await?;

    Ok(response)
}

pub fn get_price_change_class(price_change: &str) -> &'static str {
    if let Ok(change) = price_change.parse::<f64>() {
        if change > 0.0 {
            "text-green-500"
        } else if change < 0.0 {
            "text-red-500"
        } else {
            "text-gray-400"
        }
    } else {
        "text-gray-400"
    }
}

pub fn format_volume(volume: &str) -> String {
    if let Ok(vol) = volume.parse::<f64>() {
        if vol > 1_000_000.0 {
            format!("{:.2}M", vol / 1_000_000.0)
        } else if vol > 1_000.0 {
            format!("{:.2}K", vol / 1_000.0)
        } else {
            volume.to_string()
        }
    } else {
        volume.to_string()
    }
}

pub fn format_number(number: &str) -> String {
    if let Ok(num) = number.parse::<f64>() {
        format!("{:.2}", num)
    } else {
        number.to_string()
    }
}
