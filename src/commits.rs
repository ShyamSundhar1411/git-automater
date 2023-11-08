use dialoguer::{{console::Style, theme::ColorfulTheme, FuzzySelect, Input }};
use std::process::{Command,exit};

pub fn add_files(){
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

pub fn commit(){
    let commit_message: String = Input::new().with_prompt("Commit Message").interact_text().unwrap();
    let output = Command::new("git").arg("commit").arg("-m").arg(commit_message).output().expect("Failed to add commit message");
    println!("Status: {}",String::from_utf8_lossy(&output.stdout));
    println!("Status: {}",String::from_utf8_lossy(&output.stderr));
}