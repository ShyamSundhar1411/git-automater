use std::process::Command;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, console::Style,Input};
use crate::helpers;

pub fn get_remotes(show_verbose: bool) -> Vec<String>{
    let mut cmd = Command::new("git");
    cmd.arg("remote");

    if show_verbose {
        cmd.arg("-v");
    }
    let remotes = cmd.output().expect("Failed to get remotes");
    let remote_output = std::str::from_utf8(&remotes.stdout).expect("Failed to parse remote output");
    let remote_list: Vec<String> = remote_output.lines().map(String::from).collect();
    remote_list
}

pub fn get_remote_url(remote_name: &str) -> String{
    let mut cmd = Command::new("git");
    cmd.arg("remote").arg("get-url").arg(remote_name);
    let remote_url = cmd.output().expect("Failed to get remote url");
    let remote_url = std::str::from_utf8(&remote_url.stdout).expect("Failed to parse remote url");
    remote_url.trim().to_string()
}
pub fn view_remotes(){
    let remote_list: Vec<String> = get_remotes(true);
    if remote_list.is_empty(){
        let message = "No Remotes found";
        println!("{}",Style::new().for_stdout().green().apply_to(&message));

    }
    for remote in remote_list{
        println!("{}",Style::new().for_stdout().blue().apply_to(&remote));
    }
}

pub fn add_remote(){
    let help_text = "An alias is a short name for the remote, like 'origin' or 'upstream'";
    
    let prompt_text = format!(
        "Enter Remote Alias [{}]:\n  (Hint: {})",
        "origin", help_text
    );
    let remote_alias: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt_text)
        .default("origin".into())
        .interact_text()
        .unwrap();
    let remote_url = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Remote URL")
        .validate_with(|input: &String| helpers::validate_remote_url(input))
        .interact_text()
        .unwrap();
    let output = Command::new("git").arg("remote").arg("add").arg(&remote_alias).arg(&remote_url).output().expect("Failed to add remote");
    helpers::status_printer(&output);
    println!("{}",Style::new().for_stdout().green().apply_to("Remote Created"));
}
pub fn update_remote(){
    let remote_list: Vec<String> = get_remotes(false);
    let remote_list_with_url: Vec<String> = remote_list.iter().map(|remote| {
        let remote_url = get_remote_url(remote);
        format!("{} {}", remote, remote_url)
    }).collect();
    let remote_prompt = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Remote to Update")
        .default(0)
        .items(&remote_list_with_url)
        .interact()
        .unwrap();
    let remote_name = remote_list[remote_prompt].split_whitespace().next().unwrap();
    let remote_url = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter Remote URL")
        .validate_with(|input: &String| helpers::validate_remote_url(input))
        .interact_text()
        .unwrap();
    let output = Command::new("git").arg("remote").arg("set-url").arg(remote_name).arg(&remote_url).output().expect("Failed to update remote");
    helpers::status_printer(&output);
    println!("{}", Style::new().for_stdout().green().apply_to("Remote Updated"));
}

pub fn delete_remote(){
    let remote_list: Vec<String> = get_remotes(false);
    let remote_list_with_url: Vec<String> = remote_list.iter().map(|remote| {
        let remote_url = get_remote_url(remote);
        format!("{} {}", remote, remote_url)
    }).collect();
    let remote_prompt = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Remote to Delete")
        .default(0)
        .items(&remote_list_with_url)
        .interact()
        .unwrap();
    let remote_name = remote_list[remote_prompt].split_whitespace().next().unwrap();
    let output = Command::new("git").arg("remote").arg("remove").arg(remote_name).output().expect("Failed to delete remote");
    helpers::status_printer(&output);
    println!("{}", Style::new().for_stdout().green().apply_to("Remote Deleted"));
}
pub fn remote_manager(){
    let remote_prompts = ["View Remotes","Add Remote","Update Remote", "Delete Remote"];
    let remote_prompt_selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Remote Operation")
        .default(0)
        .items(&remote_prompts)
        .interact()
        .unwrap();
    if remote_prompt_selection == 0{
        view_remotes();
    }
    if remote_prompt_selection == 1{
        add_remote();
    }
    if remote_prompt_selection == 2{
        update_remote();
    }
    if remote_prompt_selection == 3{
        delete_remote();
    }
}