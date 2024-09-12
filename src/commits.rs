use inquire::Text;

use std::{process::Command,collections::HashMap};
use crate::helpers::{self, display_options};
pub struct Commit{
    description: String,
    commit_type: String,
    body: Option<String>,
    footer: Option<String>,
    file_name: Option<String>
}
impl Commit{
    pub fn new(commit_type: &str, description: &str,body: Option<&str>, footer: Option<&str>, file_name: Option<&str>) -> Self{
        Commit{
            commit_type: commit_type.to_string(),
            description: description.to_string(),
            body: body.map(String::from),
            footer: footer.map(String::from),
            file_name: file_name.map(String::from),
        }
    }
    pub fn to_string(&self) -> String{
        let mut commit_message = String::new();
        commit_message.push_str(&format!("{}",self.commit_type));
        if let Some(ref file_name) = &self.file_name {
            if !file_name.is_empty() {
                commit_message.push_str("(");
                commit_message.push_str(file_name);
                commit_message.push_str(")");
            }
        };
        commit_message.push_str(&format!(": {}",self.description));
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
    let file_name: String = Text::new("File path").with_default(".").prompt().unwrap();

    
    let output = if file_name != "." {
        Command::new("git")
            .arg("add")
            .arg(&file_name)
            .output()
            .expect("failed to add files")
    } else {
        Command::new("git")
            .arg("add")
            .arg(".")
            .output()
            .expect("failed to add files")
    };
    helpers::status_printer(&output); 
    
    
}

pub fn commit_function(){
    let conventional_commit_types = vec![
        "feat", "fix", "docs", "style", "refactor", "perf", "test", 
        "build", "ci", "chore", "revert"
    ];

    let emoji_mapping: HashMap<&str, &str> = [
        ("feat", "\u{2728}"),     // Sparkles
        ("fix", "\u{1F41B}"),     // Bug
        ("docs", "\u{1F4DD}"),    // Document
        ("style", "\u{1F484}"),   // Lipstick
        ("refactor", "\u{1F527}"),// Triangular ruler
        ("perf", "\u{26A1}"),     // High voltage
        ("test", "\u{1F6A8}"),    // Police car light
        ("build", "\u{1F527}"),   // Wrench
        ("ci", "\u{1F680}"),      // Rocket
        ("chore", "\u{1F528}"),   // Hammer
        ("revert", "\u{23EA}"),   // Double arrow left
    ].iter().cloned().collect();

    let formatted_options: Vec<String> = conventional_commit_types
        .iter()
        .map(|commit_type| {
            if let Some(emoji) = emoji_mapping.get(commit_type) {
                format!("{} ({})", commit_type, emoji)
            } else {
                commit_type.to_string()
            }
        })
        .collect();

    let commit_type = match display_options("Select a commit type", formatted_options) {
        Ok(commit_type) => commit_type,
        Err(_) => return,
    };

    let file_name: Option<String> = Text::new("Enter file name or class name (default will be blank)")
        .with_default("")
        .prompt()
        .ok();


    let description = loop{
        let result = Text::new("Enter a short description").prompt();
        match result{
            Ok(input) => {
                if input.trim().is_empty(){
                    println!("Description cannot be empty");
                    continue;
                }
                break input;
            }
            Err(err) => {
                eprintln!("An error occurred: {:?}", err);
                std::process::exit(1); 
            }
        }
    };


    let body: Option<String> = Text::new("Enter brief description (optional)")
        .with_default("")
        .prompt()
        .ok();
    let footer: Option<String> = Text::new("Enter footer (optional)")
        .with_default("")
        .prompt()
        .ok();

    let commit = Commit::new(
        commit_type.as_str(),
        &description,
        body.as_deref(),
        footer.as_deref(),
        file_name.as_deref(),
    );

    let commit_message = commit.to_string();

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
        .expect("Failed to add commit message");

    
   

  

    helpers::status_printer(&output);
}
