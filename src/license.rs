use dialoguer::{theme::ColorfulTheme, Input};
use serde::Deserialize;
use reqwest::{Error, Client};
use std::{process::Command,fs,io};

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

pub fn get_git_user_name() -> Option<String> {
    let output = Command::new("git").arg("config").arg("--global").arg("user.name").output().ok()?;
    let result: Option<String> = match output.status.success(){
        true=>Option::from(String::from_utf8_lossy(&output.stdout).to_string()),
        false=>Option::from(None),
    };
    result
}

pub fn write_license_file(content: &str, name: &str, year: &str) -> Result<(), io::Error>{
    let mut body  = content.to_string();
    body = body.replace("<name of author>",&name).replace("<year>",&year).replace("[yyyy]",&year).
    replace("[name of copyright owner]",&name).replace("[ yyyy ]",&year).replace("[ name ]",&name).
    replace("[name]",&name).replace("[year]",&year).replace("[fullname]",&name);

    let path = "./LICENSE";
    let output =  match fs::metadata(path).is_ok(){
        true=>{
            let path = Input::with_theme(&ColorfulTheme::default()).with_prompt("License Found!!. New license name (leave blank to overwrite existing)").default(path.to_string()).interact_text().unwrap();
            fs::write(path,&body)
        }
        false => {
            fs::write(path,&body)
        }
    };
    output

}