use dialoguer::{{console::Style, theme::ColorfulTheme, FuzzySelect, Input }};
use std::{process::{Command,exit}, collections::HashMap};
use crate::{license::{self, LicenseContent}, branches};
use crate::commits;


fn get_name() -> String {
    let name: String = match license::get_git_user_name() {
        Some(mut name) => {
            // removing trailing newline (cross platform way)
            if name.ends_with("\n") {
                name.pop();

                if name.ends_with("\r") {
                    name.pop();
                }
            }

            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter Author Name")
                .default(name)
                .interact_text()
                .unwrap();

            name
        }
        None => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Name")
                .interact_text()
                .unwrap();

            input
        }
    };

    name
}

pub fn prompt(){
    let items = vec!["initialize git repository","add files","commit","push","add license","clear cache","exit"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("What do you choose?").items(&items).interact().unwrap();
    println!("{}",items[selection]);

    if items[selection] == "initialize git repository"{
        initialize();
    }
    if items[selection] == "add files"{
        commits::add_files();
    }
    if items[selection] == "exit"{
        exit(0);
    }
    if items[selection] == "commit"{
        commits::commit_function();
    }
    if items[selection] == "push"{
        push();
    }
    if items[selection] == "clear cache"{
        clear_cache();
    }
    if items[selection] == "add license"{
        generate_license();
    }
}


fn initialize(){
    let output = Command::new("git").arg("init").output().expect("Failed to initalize repository");
    if String::from_utf8_lossy(&output.stdout) != "" {
         println!("Status: {}",String::from_utf8_lossy(&output.stdout));
    }
    else {
        println!("Error: {}",String::from_utf8_lossy(&output.stderr));
    }
}

fn push(){
    let branch_list: Vec<String> = branches::get_branches();
    let branch_selection = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Select branch").items(&branch_list).interact().unwrap();
    let branch = branch_list[branch_selection].clone();
    let remote_list = branches::get_remotes();
    let remote_selection = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Select remote").items(&remote_list).interact().unwrap();
    let alias = remote_list[remote_selection].clone();
    let output = Command::new("git").arg("push").arg("-u").arg(alias).arg(branch).output().expect("Failed to push to respective repository");
    println!("Status: {}",String::from_utf8_lossy(&output.stdout));
    println!("Status: {}",String::from_utf8_lossy(&output.stderr));
}

fn clear_cache(){
    let  output = Command::new("git").arg("rm").arg("-r").arg("--cached").arg(".").output().expect("Failed to clear cache");
    println!("Status: {}",String::from_utf8_lossy(&output.stdout));
    println!("Status: {}",String::from_utf8_lossy(&output.stderr));
}

fn generate_license(){
    let licenses = match license::fetch_licenses(){
        Ok(licenses) => licenses,
        Err(err) => {
            println!("Error fetching licenses: {}", err);
            return;
        }
    };
    let license_map: HashMap<String, String> = licenses
        .iter()
        .map(|license| (license.name.clone(), license.key.clone()))
        .collect();

    let license_names: Vec<&String> = licenses.iter().map(|license| &license.name).collect();
    let license_selection = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Choose your license").items(&license_names).interact().unwrap();
    let selected_license = license_names[license_selection];
    let selected_license_key = license_map.get(selected_license).cloned().unwrap_or_default();
    let license_content = match license::fetch_license_content(&selected_license_key){
        Ok(license_content) => license_content,
        Err(err) => {
            println!("Error fetching license content: {}",err);
            return ;
        }
    };
    let name = get_name();
    license::write_license_file(&license_content.body);
    
}

