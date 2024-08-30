use crate::license;
use inquire::{Text, CustomType};
use chrono::{Datelike, Utc};
use::std::process::Output;
use::inquire::{Select, InquireError};

pub fn status_printer(output:&Output){
    let status_out: String = String::from_utf8_lossy(&output.stdout).to_string();
    let status_err: String = String::from_utf8_lossy(&output.stderr).to_string();
    if !(status_err.is_empty()){
        println!("\x1b[31m{}\x1b[0m", status_err);
    }else{
        println!("\x1b[32m{}\x1b[0m", status_out);
    }
}
pub fn get_name() -> String {
    let name: String = match license::get_git_user_name() {
        Some(mut name) => {
            // removing trailing newline (cross platform way)
            if name.ends_with("\n") {
                name.pop();

                if name.ends_with("\r") {
                    name.pop();
                }
            }

            let name: String = Text::new("Enter Author  Name")
            .with_default(&name)
            .prompt()
            .unwrap();

            name
        }
        None => {
            let input: String = Text::new("Enter Author  Name").prompt().unwrap();
            input
        }
    };

    name
}
pub fn get_year() -> String{
    let current_year = Utc::now().year();
    let year: String = CustomType::new("Enter year")
    .with_default(current_year)
    .prompt()
    .unwrap()
    .to_string();
    year
}
pub fn display_options(prompt: &str,items: Vec<String>) -> Result<String,InquireError>{
    let branch_options = Select::new(prompt,items).prompt();
    return branch_options;
}