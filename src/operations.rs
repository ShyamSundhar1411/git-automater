use dialoguer::{{console::Style, theme::ColorfulTheme, FuzzySelect }};
use std::{process::{Command,exit}, collections::HashMap};
use crate::{license, branches, commits, helpers, gitignore};
pub fn prompt(){
    let items = vec!["Initialize Git Repository","Add Files","Commit","Push","Add License","Add .gitignore","Branch Manager","Clear Cache","Exit"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("What do you choose?").items(&items).interact().unwrap();
    println!("{}",items[selection]);

    if selection == 0{
        initialize();
    }
    if selection == 1{
        commits::add_files();
    }
    
    if selection == 2{
        commits::commit_function();
    }
    if selection == 3{
        push();
    }
    if selection == 4{
        generate_license();
    }
    if selection == 5{
        gitignore::generate_gitignore();
    }

    if selection == 6{
        branches::branch_manager();
    }

    if selection == 7{
        clear_cache();
    }
    
    if selection == 8{
        exit_prompt();
    }
}


fn initialize(){
    let output = Command::new("git").arg("init").output().expect("Failed to initalize repository");
    helpers::status_printer(&output);
}

fn push(){
    let branch_list: Vec<String> = branches::get_branches();
    let branch_selection = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Select branch").items(&branch_list).interact().unwrap();
    let branch = branch_list[branch_selection].clone();
    let remote_list = branches::get_remotes();
    let remote_selection = FuzzySelect::with_theme(&ColorfulTheme::default()).with_prompt("Select remote").items(&remote_list).interact().unwrap();
    let alias = remote_list[remote_selection].clone();
    let output = Command::new("git").arg("push").arg("-u").arg(alias).arg(branch).output().expect("Failed to push to respective repository");
    helpers::status_printer(&output);
}

fn clear_cache(){
    let  output = Command::new("git").arg("rm").arg("-r").arg("--cached").arg(".").output().expect("Failed to clear cache");
    helpers::status_printer(&output);
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
    let name = helpers::get_name();
    let year: String = helpers::get_year();
    
    let _output = match license::write_license_file(&license_content.body, &name, &year){
        Ok(_)=> println!("{}",Style::new().for_stderr().green().italic().apply_to("License created successfully")),
        Err(err)=>{
            let error_message = format!("Error creating license: {}", err);
            print!("{}",Style::new().for_stderr().red().italic().apply_to(&error_message));
            return ;
        }
    };
    
    
}



fn exit_prompt(){
    println!("{}",Style::new().for_stderr().green().apply_to("Thanks for using me. Have a great day"));
    exit(0);
}
