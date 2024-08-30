use std::process::Command;
use console::Style;
use indexmap::IndexMap;
use inquire::{InquireError,Text};
use crate::helpers::{display_options,status_printer};
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

pub fn checkout_branch(){
    let selected_branch = match display_options("Select Branch to Checkout",get_branches()){
        Ok(branch) => branch,
        Err(_) => {
            eprint!("Something went wrong");
            return ;
        }

    }; 
   
    let output = Command::new("git").arg("checkout").arg(&selected_branch).output().expect("Failed to checkout branch");
    status_printer(&output);
}

pub fn create_branch(){
    let branch_name: String = Text::new("Enter Branch Name:").prompt().unwrap();
    let output = Command::new("git").arg("branch").arg(&branch_name).output().expect("Failed to create branch");
    status_printer(&output);
    println!("{}",Style::new().for_stdout().green().apply_to("Branch Created"));

}

pub fn delete_branch(){
    let selected_branch = match display_options("Select Branch to Delete",get_branches()){
        Ok(branch) => branch,
        Err(_) => {
            eprint!("Something went wrong");
            return ;
        }

    };
    let output = Command::new("git").arg("branch").arg("-D").arg(&selected_branch).output().expect("Failed to delete branch");
    status_printer(&output);
}

pub fn merge_branch(){
    checkout_branch();
    let selected_branch = match display_options("Select Branch to Merge",get_branches()){
        Ok(branch) => branch,
        Err(_) => {
            eprint!("Something went wrong");
            return ;
        }

    };
    let output = Command::new("git").arg("merge").arg(&selected_branch).output().expect("Faield to merge branch");
    status_printer(&output);
}

pub fn pull_branch(){
    let selected_remote = match display_options("Select Remote Repo", get_remotes()){
        Ok(remote) => remote,
        Err(_) => {
            eprint!("Something went wrong");
            return ;
        }

    }; 

    let output = Command::new("git").arg("pull").arg(&selected_remote).output().expect("Failed to pull changes");
    status_printer(&output);
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

pub fn branch_manager(){
    let branch_prompts: IndexMap<&str,fn()> = IndexMap::from([
        ("Checkout", checkout_branch as fn()),
        ("Create Branch", create_branch as fn()),
        ("Delete Branch", delete_branch as fn()),
        ("Merge Branch", merge_branch as fn()),
        ("Pull Branch", pull_branch as fn()),
        ("View Branches", view_branches as fn())
    ]);
    let branch_actions: Vec<String> = branch_prompts.keys().map(|&s| s.to_string()).collect();
    let branch_prompt_selection: Result<String,InquireError> = display_options("Select Branch Action", branch_actions);
    match branch_prompt_selection{
        Ok(selection) => {
            
            if let Some(action) = branch_prompts.get(selection.as_str()){
                action();
            }
        }
        Err(_) => eprintln!("Error")
    }

}