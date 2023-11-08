use serde::Deserialize;
use reqwest::{Error, Client};

#[derive(Deserialize,Debug)]
pub struct License{
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
    pub node_id: String,
}

#[derive(Deserialize,Debug)]
pub struct LicenseContent{
    pub key: String,
    pub name: String,
    pub url: String,
    pub body: String,
}
#[tokio::main]
pub async fn fetch_licenses() -> Result<Vec<License>, Error> {
    let client = Client::new();
    let url = "https://api.github.com/licenses";
    let user_agent = "git-automater";
    let response = client.get(url).header(reqwest::header::USER_AGENT,user_agent).send().await?;
    let licenses: Vec<License> = response.json().await?;
    Ok(licenses)
}

#[tokio::main]
pub async fn fetch_license_content(license: &String) -> Result<LicenseContent,Error>{
    let client = Client::new();
    let url = format!("https://api.github.com/licenses/{}",license);
    let user_agent = "git-automater";
    let response = client.get(url).header(reqwest::header::USER_AGENT,user_agent).send().await?;
    let license: LicenseContent = response.json().await?;
    Ok(license)
}