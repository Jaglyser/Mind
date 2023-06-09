mod get_stock_data;
use std::collections::HashMap;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.avanza.se/_api/price-chart/stock/5364?timePeriod=one_week";
    let res = reqwest::get(url)
        .await?
        .json::<Value>()
        .await?;

    println!("{:#?}", res);
    Ok(())
}
