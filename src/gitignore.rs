use reqwest;
use dialoguer::{{console::Style, theme::ColorfulTheme, Input }};
use std::{fs,io};

#[tokio::main]
pub async fn get_gitignore_template(languages: &[&str]) -> Result<String, reqwest::Error>{
    let languages_str = languages.join(",");
    let url = format!("https://www.gitignore.io/api/{}",languages_str);
    let response = reqwest::get(&url).await?.text().await?;
    Ok(response)
}
fn write_gitignore_file(content: &str) -> Result<(),io::Error>{
    let path: &str = "./gitignore";
    let body = content.to_string();
    let output = match fs::metadata(path).is_ok(){
        true=>{
            let path = Input::with_theme(&ColorfulTheme::default()).with_prompt("Gitignore Found!!. New Gitignore path (leave blank to overwrite existing)").default(path.to_string()).interact_text().unwrap();
            fs::write(path,&body)
        }
        false=>{
            fs::write(path,body)
        }
    };
    output
}
pub fn generate_gitignore(){
    let languages_input: String = Input::new()
        .with_prompt("Enter languages (comma-separated)")
        .interact_text()
        .expect("Failed to read input");
    let languages: Vec<&str> = languages_input.split(',').map(|s| s.trim()).collect();
    if languages.is_empty() {
        eprintln!("No languages provided. Exiting.");
        return;
    }
    let content = match get_gitignore_template(&languages){
        Ok(response) => response,
        Err(err) => {
            println!("Error fetching content: {}", err);
            return;
        }
    };
    let _output = match write_gitignore_file(&content){
        Ok(_)=> println!("{}",Style::new().for_stderr().green().italic().apply_to(".gitignore created successfully")),
        Err(err)=>{
            let error_message = format!("Error creating .gitignore: {}", err);
            print!("{}",Style::new().for_stderr().red().italic().apply_to(&error_message));
            return ;
        }
    };

}