use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize,Debug)]
pub struct License{
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
    pub node_id: String,
}
#[tokio::main]
pub async fn fetch_licenses() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/licenses");
    let response = reqwest::get(&request_url).await?;
    let response_text = response.text().await?;
    // let licenses: Vec<License> = response.json().await?;
    println!("API Response: {:?}", response_text);
    Ok(())
}