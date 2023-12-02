use std::process::Command;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, console::Style};
pub fn get_branches() -> Vec<String>{
    let branches = Command::new("git").arg("branch").arg("--format").arg("%(refname:short)").output().expect("Failed to fetch branches");
    let branch_output = std::str::from_utf8(&branches.stdout).expect("Failed to parse branch output");
    let branch_list: Vec<String> = branch_output.lines().map(String::from).collect();
    branch_list
}

pub fn get_remotes() -> Vec<String>{
    let remote = Command::new("git").arg("remote").output().expect("Failed to fetch remote");
    let remote_output = std::str::from_utf8(&remote.stdout).expect("Failed to parse remote output");
    let remote_list: Vec<String> = remote_output.lines().map(String::from).collect();
    remote_list
}

pub fn branch_manager(){
    let branch_prompts = ["Checkout", "Create Branch", "Delete Branch", "Merge Branch", "Push Branch", "Pull Branch","View Branches","Switch Branch"];
    let branch_prompt_selection = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Select Branch Operation").items(&branch_prompts).interact().unwrap();
    if branch_prompt_selection == 6{
        let output  = get_branches();
        if output.is_empty(){
            let message = "Error Fetching Branches";
            println!("{}",Style::new().for_stderr().red().apply_to(&message));
        }
        for branch in output{
            println!("{}",Style::new().for_stdout().green().apply_to(&branch));
        }
    }
}