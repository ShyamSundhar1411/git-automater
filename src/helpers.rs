use dialoguer::{{console::Style, theme::ColorfulTheme, FuzzySelect, Input }};
use std::process::{Command,exit};

pub fn prompt(){
    let items = vec!["initialize git repository","add files","commit","push","add license","add readme.MD","exit"];
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
}

fn push(){
    let branch: String = Input::new().with_prompt("Branch Name").default("main".to_string()).interact_text().unwrap();
    let alias: String = Input::new().with_prompt("Repo Alias").default("origin".to_string()).interact_text().unwrap();
    let output = Command::new("git").arg("push").arg("-u").arg(alias).arg(branch).output().expect("Failed to push to respective repository");
    println!("Status: {}",String::from_utf8_lossy(&output.stdout));
}