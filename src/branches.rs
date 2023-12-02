use std::process::Command;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, console::Style,Input};
use crate::helpers;
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

pub fn create_branch(){
    let branch_name: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Enter Branch Name").interact_text().unwrap();
    let output = Command::new("git").arg("branch").arg(&branch_name).output().expect("Failed to create branch");
    helpers::status_printer(&output);
    println!("{}",Style::new().for_stdout().green().apply_to("Branch Created"));

}

pub fn delete_branch(){
    let branch_list = get_branches();
    let branch_option = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Select Branch to Delete").items(&branch_list).interact().unwrap();
    let selected_branch = &branch_list[branch_option];
    let output = Command::new("git").arg("branch").arg("-D").arg(&selected_branch).output().expect("Failed to delete branch");
    helpers::status_printer(&output);
}
pub fn view_branches(){
    let output  = get_branches();
        if output.is_empty(){
            let message = "Error Fetching Branches";
            println!("{}",Style::new().for_stderr().red().apply_to(&message));
        }
        for branch in output{
            println!("{}",Style::new().for_stdout().green().apply_to(&branch));
        }
}
pub fn checkout_branch(){
    let branch_list = get_branches();
    let branch_option = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Select Branch to Checkout").items(&branch_list).interact().unwrap();
    let selected_branch = &branch_list[branch_option];
    let output = Command::new("git").arg("checkout").arg(&selected_branch).output().expect("Failed to checkout branch");
    helpers::status_printer(&output);
}

pub fn merge_branch(){
    checkout_branch();
    let branch_list = get_branches();
    let branch_options = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Select Branch to Merge").items(&branch_list).interact().unwrap();
    let selected_branch = &branch_list[branch_options];
    let output = Command::new("git").arg("merge").arg(&selected_branch).output().expect("Faield to merge branch");
    helpers::status_printer(&output);
}
pub fn branch_manager(){
    let branch_prompts = ["Checkout", "Create Branch", "Delete Branch", "Merge Branch", "Pull Branch","View Branches"];
    let branch_prompt_selection = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Select Branch Operation").items(&branch_prompts).interact().unwrap();

    if branch_prompt_selection == 0{
        checkout_branch();
    }
    if branch_prompt_selection == 1{
        create_branch();
    }
    if branch_prompt_selection == 2{
        delete_branch();
    }
    if branch_prompt_selection == 3 {
        merge_branch();
    }
    if branch_prompt_selection == 5{
        view_branches();
    }
}