use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    username: String,
    projectname: String,
}

fn main() {
    let args = Cli::parse();
    let username = args.username;
    let projectname = args.projectname;

    let home_dir = std::env::var("HOME").expect("Could not get HOME environment variable");
    let base_dir = PathBuf::from(format!("{}/dev", home_dir));
    let user_dir = base_dir.join(&username);
    let project_dir = user_dir.join(&projectname);

    // Step 2: Check if the username directory exists
    if !user_dir.exists() {
        print!("Directory for user '{}' does not exist. Create it? (y/n): ", username);
        io::stdout().flush().unwrap();
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        if response.trim().eq_ignore_ascii_case("y") {
            fs::create_dir_all(&user_dir).expect("Failed to create user directory");
        } else {
            println!("Exiting.");
            return;
        }
    }

    // Step 3: Change into username directory
    std::env::set_current_dir(&user_dir).expect("Failed to change directory to user");

    // Step 4: Check if the project directory exists
    if !project_dir.exists() {
        print!("Project '{}' does not exist. Clone it from GitHub? (y/n): ", projectname);
        io::stdout().flush().unwrap();
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        if response.trim().eq_ignore_ascii_case("y") {
            let repo_url = format!("{}/{}", username, projectname);
            let output = Command::new("gh")
                .arg("repo")
                .arg("clone")
                .arg(&repo_url)
                .status()
                .expect("Failed to execute 'gh repo clone'");

            if !output.success() {
                println!("Failed to clone the repository.");
                return;
            }
        } else {
            println!("Exiting.");
            return;
        }
    }

    // Step 5: Change into the project directory
    std::env::set_current_dir(&project_dir).expect("Failed to change directory to project");

    // Step 6: Check for existing Zellij session
    let session_name = format!("{} {}", username, projectname);
    let output = Command::new("zellij")
        .arg("list-sessions")
        .output()
        .expect("Failed to list Zellij sessions");

    let session_list = String::from_utf8_lossy(&output.stdout);
    if session_list.contains(&session_name) {
        Command::new("zellij")
            .arg("attach")
            .arg(&session_name)
            .status()
            .expect("Failed to attach to Zellij session");
    } else {
        Command::new("zellij")
            .arg("-s")
            .arg(&session_name)
            .status()
            .expect("Failed to create Zellij session");
    }
}

