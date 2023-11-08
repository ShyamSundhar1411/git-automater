use std::process::Command;

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