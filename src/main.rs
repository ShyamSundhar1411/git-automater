mod operations;
mod helpers;
mod license;
mod commits;
mod branches;
mod remote;
mod gitignore;
fn main() {
    loop{
        operations::prompt();
    }
}
