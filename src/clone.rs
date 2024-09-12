use std::{path::Path, process::{Command, Output}};
use indicatif::{ProgressBar, ProgressStyle};
use crate::helpers;
use inquire::Text;



pub fn clone_repository() {
  
    let repo_url = Text::new("Enter the repository URL:").prompt().unwrap();
    
   
    let path = Text::new("Enter the path to clone the repository:")
        .with_default("./")
        .prompt().unwrap();
    
    let path = Path::new(&path);

 
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/white}] {pos:>7}/{len:7} ({eta})").unwrap());
    

    if !path.exists() {
        eprintln!("Error: The path already exists.");
    }
    let output: Output = if path.to_str() == Some("./") {
        Command::new("git")
            .arg("clone")
            .arg(&repo_url)
            .output().expect("Failed to execute git command")
    } else {
        Command::new("git")
            .arg("clone")
            .arg(&repo_url)
            .arg(&path.display().to_string())
            .output().expect("Failed to execute git command")
    };
    
    for _ in 0..100 {
        pb.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    pb.finish_with_message("Cloning completed!");
    helpers::status_printer(&output); 
    
    
}
