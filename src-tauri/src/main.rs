// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fmt;
use std::fs::{self};
use std::fs::{File, OpenOptions};
use std::io::{self};
use std::io::{Read, Write};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[derive(Serialize, Deserialize, Debug)]

struct Project {
    id: u32,
    name: String,
    description: Option<String>,
    status: Option<String>,
    created_at: String,
    updated_at: String,
    logo: Option<String>,
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn store_projects(
    projects: &Vec<Project>,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(&projects)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Important: clear the file content before writing
        .open(file_path)?;

    file.write_all(json.as_bytes())?;
    Ok(())
}

fn read_projects(file_path: &str) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
    let file = File::open(file_path);

    let projects = match file {
        Ok(mut file) => {
            let mut json = String::new();
            file.read_to_string(&mut json)?;
            serde_json::from_str(&json)?
        }
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
            let projects = Vec::new();
            let json = serde_json::to_string(&projects)?;
            fs::create_dir_all(get_home_dir() + "/.projects")?;
            let mut file = File::create(file_path)?;
            file.write_all(json.as_bytes())?;
            projects
        }
        Err(ref err) if err.kind() == io::ErrorKind::UnexpectedEof => {
            println!("here");
            let projects = Vec::new();
            projects
            // ! handle if object is empty
        }
        Err(err) => return Err(Box::new(err)),
    };

    Ok(projects)
}

fn get_home_dir() -> String {
    return env::var("HOME").expect("Failed to get the user's home directory.");
}
fn get_file_path() -> String {
    let home_dir = get_home_dir();
    let file_path = format!("{}/.projects/data.json", home_dir);
    file_path
}

#[tauri::command]

fn get_project(id: u32) -> Option<Project> {
    let projects = get_projects();

    for project in projects {
        if project.id == id {
            return Some(project);
        }
    }

    None // Return None if the project with the given id is not found
}

#[tauri::command]
async fn delete_project(id: u32) {
    let mut projects = get_projects();
    projects.retain(|project| project.id != id);
    match store_projects(&projects, &get_file_path()) {
        Ok(_) => println!("Project with ID {} deleted.", id),
        Err(e) => eprintln!("Failed to store projects after deletion: {}", e),
    }
}

#[tauri::command]
fn get_projects() -> Vec<Project> {
    let file_path = get_file_path();
    match read_projects(&file_path) {
        Ok(projects) => projects,
        Err(e) => {
            eprintln!("Error reading projects: {}", e);
            vec![] // return an empty vector in case of error
        }
    }
}

#[tauri::command]
fn add_project(mut project: Project) -> bool {
    let mut projects = get_projects();
    if projects.len() == 0 {
        project.id = 1;
    } else {
        project.id = projects[projects.len() - 1].id + 1;
    }
    projects.push(project);
    store_projects(&projects, &get_file_path()).unwrap();
    return true;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_project,
            get_projects,
            add_project,
            delete_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
