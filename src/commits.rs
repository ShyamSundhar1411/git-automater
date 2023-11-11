use dialoguer::{{console::Style, theme::ColorfulTheme, FuzzySelect, Input }};
use std::process::{Command,exit};
use crate::helpers::{self, status_printer};
pub struct Commit{
    description: String,
    commit_type: String,
    body: Option<String>,
    footer: Option<String>,
}
impl Commit{
    pub fn new(commit_type: &str, description: &str,body: Option<&str>, footer: Option<&str>) -> Self{
        Commit{
            commit_type: commit_type.to_string(),
            description: description.to_string(),
            body: body.map(String::from),
            footer: footer.map(String::from),
        }
    }
    pub fn to_string(&self) -> String{
        let mut commit_message = String::new();
        commit_message.push_str(&format!("{}: {}", self.commit_type, self.description));

        if let Some(ref body) = self.body {
            commit_message.push_str("\n\n");
            commit_message.push_str(body);
        }

        if let Some(ref footer) = self.footer {
            commit_message.push_str("\n\n");
            commit_message.push_str(footer);
        }

        commit_message
    }
}
pub fn add_files(){
    let file_name: String = Input::new().with_prompt("File Name").default(".".to_string()).interact_text().unwrap();
    if file_name != "."{
        let output = Command::new("git").arg("add").arg(file_name).output().expect("failed to add files");
        helpers::status_printer(&output);
    }
    else{
        let output = Command::new("git").arg("add").arg(".").output().expect("failed to add files");   
        helpers::status_printer(output);
    }   
}

pub fn commit_function(){
    let conventional_commit_types = vec!["feat","fix","docs","style","refractor","perf","test","build","ci","chore","revert"];
    let type_select = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Choose the type of commit").items(&conventional_commit_types).interact().unwrap();
    let description: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Enter a short description").interact_text().unwrap();
    let commit_type = conventional_commit_types[type_select];
    let body: Option<String> = Some(Input::with_theme(&ColorfulTheme::default()).with_prompt("Enter brief description").allow_empty(true).interact_text().unwrap_or_default());
    let footer: Option<String> = Some(Input::with_theme(&ColorfulTheme::default()).with_prompt("Enter footer").allow_empty(true).interact_text().unwrap_or_default());
    let commit = Commit::new(
        commit_type,
        &description,
        body.as_deref(),
        footer.as_deref(),
    );
    let commit_message = commit.to_string();
    let output = Command::new("git").arg("commit").arg("-m").arg(commit_message).output().expect("Failed to add commit message");
    status_printer(&output);
}