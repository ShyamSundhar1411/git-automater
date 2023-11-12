use dialoguer::{{console::Style, theme::ColorfulTheme,Input }};
use crate::license;
use chrono::{Datelike, Utc};
use::std::process::Output;

pub fn status_printer(output:&Output){
    let status_out: String = String::from_utf8_lossy(&output.stdout).to_string();
    let status_err: String = String::from_utf8_lossy(&output.stderr).to_string();
    if !(status_err.is_empty()){
        println!("{}", Style::new()
        .for_stderr()
        .red()
        .apply_to(status_err));
    }
    else{
        println!("{}", Style::new()
        .for_stderr()
        .green()
        .apply_to(status_out));
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

            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter Author Name")
                .default(name)
                .interact_text()
                .unwrap();

            name
        }
        None => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Name")
                .interact_text()
                .unwrap();

            input
        }
    };

    name
}
pub fn get_year() -> String{
    let current_year = Utc::now().year();
    let year: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Enter year").default(current_year.to_string()).interact_text().unwrap();
    year
}