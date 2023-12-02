use dialoguer::{{ theme::ColorfulTheme, FuzzySelect, Input }};
use std::{process::Command,collections::HashMap};
use crate::helpers;
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
    let file_name: String = Input::new().with_prompt("File Path").default(".".to_string()).interact_text().unwrap();
    if file_name != "."{
        let output = Command::new("git").arg("add").arg(file_name).output().expect("failed to add files");
        helpers::status_printer(&output); 
    }
    else{
        let output = Command::new("git").arg("add").arg(".").output().expect("failed to add files");   
        helpers::status_printer(&output); 
    }  
    
}

pub fn commit_function(){
    let conventional_commit_types = vec!["feat","fix","docs","style","refactor","perf","test","build","ci","chore","revert"];
    let emoji_mapping: HashMap<&str, &str> = [
        ("feat", "\u{2728}"),   // Sparkles
        ("fix", "\u{1F41B}"),    // Bug
        ("docs", "\u{1F4DD}"),   // Document
        ("style", "\u{1F484}"),  // Lipstick
        ("refactor", "\u{1F527}"), // Triangular ruler
        ("perf", "\u{26A1}"),     // High voltage
        ("test", "\u{1F6A8}"),    // Police car light
        ("build", "\u{1F527}"),   // Wrench
        ("ci", "\u{1F680}"),      // Rocket
        ("chore", "\u{1F528}"),   // Hammer
        ("revert", "\u{23EA}"),   // Double arrow left
    ]
    .iter()
    .cloned()
    .collect();
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

// Display the custom formatted list with emojis using FuzzySelect
    let type_select = FuzzySelect::new()
    .with_prompt("Select a conventional commit type")
    .items(&formatted_options)
    .interact()
    .expect("Failed to read selection");
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
    helpers::status_printer(&output); 
}