use dialoguer::{{console::Style, theme::ColorfulTheme, FuzzySelect, Input }};
use std::process::{Command,exit};

pub fn prompt(){
    let items = vec!["initialize git repository","add files","commit","push","add license","add readme.MD","clear cache","exit"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("What do you choose?").items(&items).interact().unwrap();
    println!("{}",items[selection]);
    if items[selection] == "add files"{
        add_files();
    }
    if items[selection] == "exit"{
        exit(0);
    }
    if items[selection] == "commit"{
        commit();
    }
    if items[selection] == "push"{
        push();
    }
    if items[selection] == "clear cache"{
        clear_cache();
    }
}
fn add_files(){
    let file_name: String = Input::new().with_prompt("File Name:").default(".".to_string()).interact_text().unwrap();
    if file_name != "."{
        let output = Command::new("git").arg("add").arg(file_name).output().expect("failed to add files");
        println!("Status: {}",String::from_utf8_lossy(&output.stdout));
    }
    else{
        let output = Command::new("git").arg("add").arg(".").output().expect("failed to add files");   
        println!("Status: {}",String::from_utf8_lossy(&output.stdout));
    }   
}

fn commit(){
    let commit_message: String = Input::new().with_prompt("Commit Message").interact_text().unwrap();
    let output = Command::new("git").arg("commit").arg("-m").arg(commit_message).output().expect("Failed to add commit message");
    println!("Status: {}",String::from_utf8_lossy(&output.stdout));
    println!("Status: {}",String::from_utf8_lossy(&output.stderr));
}

fn push(){
    let branches = Command::new("git").arg("branch").arg("--format").arg("%(refname:short)").output().expect("Failed to fetch branches");
    let branch_output = std::str::from_utf8(&branches.stdout).expect("Failed to parse branch output");
    let branch_list: Vec<&str> = branch_output.lines().collect();
    let branch_selection = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Select branch").items(&branch_list).interact().unwrap();

    let remote = Command::new("git").arg("remote").output().expect("Failed to fetch remote");
    let remote_output = std::str::from_utf8(&remote.stdout).expect("Failed to parse remote output");
    let remote_list: Vec<&str> = remote_output.lines().collect();
    let remote_selection = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Select remote").items(&remote_list).interact().unwrap();
    let branch = branch_list[branch_selection];
    let alias = remote_list[remote_selection];
    let output = Command::new("git").arg("push").arg("-u").arg(alias).arg(branch).output().expect("Failed to push to respective repository");
    println!("Status: {}",String::from_utf8_lossy(&output.stdout));
    println!("Status: {}",String::from_utf8_lossy(&output.stderr));
}

fn clear_cache(){
    let  output = Command::new("git").arg("rm").arg("-r").arg("--cached").arg(".").output().expect("Failed to clear cache");
    println!("Status: {}",String::from_utf8_lossy(&output.stdout));
    println!("Status: {}",String::from_utf8_lossy(&output.stderr));
}