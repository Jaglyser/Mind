use reqwest::{get, Client};

use serde_json::Value;


async fn get_stock() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.avanza.se/_api/search/global-search";
    let res = ""
}

#[tokio::main]
async fn get_stock_data() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.avanza.se/_api/price-chart/stock/5364?timePeriod=one_week";
    let res = reqwest::get(url)
        .await?
        .json::<Value>()
        .await?;

    println!("{:#?}", res);
    Ok(())
}
