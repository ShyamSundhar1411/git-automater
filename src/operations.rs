use inquire::error::InquireError;
use console::Style;
use std::{process::{exit, Command},collections::HashMap};
use indexmap::IndexMap;
use crate::{branches, commits, gitignore, helpers::{self, display_options}, license};
pub fn prompt(){
    let actions: IndexMap<&str,fn()> = IndexMap::from([
        ("Initialize Git Repository",initialize as fn()),
        ("Add Files",commits::add_files as fn()),
        ("Commit", commits::commit_function as fn()),
        ("Push", push as fn()),
        ("Add License", generate_license as fn()),
        ("Add .gitignore", gitignore::generate_gitignore as fn()),
        ("Branch Manager", branches::branch_manager as fn()),
        ("Clear Cache", clear_cache as fn()),
        ("Exit",exit_prompt as fn())
    ]);
    let prompt_actions: Vec<String> = actions.keys().cloned().map(|s| s.to_string()).collect();
    let prompt_selection: Result<String, InquireError> = display_options("Select action", prompt_actions);
    match prompt_selection{
        Ok(selection) => {
            
            if let Some(action) = actions.get(selection.as_str()){
                action();
            }
        }
        Err(_) => eprintln!("Something went wrong"),
    }
}


fn initialize(){
    let output = Command::new("git").arg("init").output().expect("Failed to initalize repository");
    helpers::status_printer(&output);
}

fn push(){
    let branch_list: Vec<String> = branches::get_branches();
    let branch_selection = display_options("Select branch", branch_list);
    let branch = match branch_selection{
        Ok(branch) => branch.to_string(),
        Err(_) => {
            println!("Something went wrong");
            return;
        },
    };
    let remote_list = branches::get_remotes();
    let remote_selection = display_options("Select remote", remote_list);
    let alias = match remote_selection{
        Ok(remote) => remote.to_string(),
        Err(_) => {
            println!("Something went wrong");
            return;
        },
    };

    let output = Command::new("git").arg("push").arg("-u").arg(&alias).arg(&branch).output().expect("Failed to push to respective repository");
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

    let license_names: Vec<String> = licenses.iter().map(|license| license.name.to_string()).collect();
    let selected_license = match display_options("Select License",license_names){
        Ok(license) => license,
        Err(_) => {
            println!("Something went wrong");
            return;
        }
    };
    let selected_license_key = license_map.get(&selected_license).cloned().unwrap_or_default();
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
            eprintln!("{}",Style::new().for_stderr().red().italic().apply_to(&error_message));
        }
    };
    
    
}



fn exit_prompt(){
    println!("{}",Style::new().for_stderr().green().apply_to("Thanks for using me. Have a great day"));
    exit(0);
}
