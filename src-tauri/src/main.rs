// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::{self, copy};
use std::fs::{File, OpenOptions};
use std::io::{self};
use std::io::{Read, Write};
use std::path::Path;
use std::path::PathBuf;

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

/**
 * Stores the projects vector in the data.json file
 * @ return Result<(), Box<dyn std::error::Error>> (should return Ok or an error)
 */
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

/**
 * Reads the data.json file and returns a vector of Project structs
 * @ return Result<Vec<Project>, Box<dyn std::error::Error>> (should return a vector of Project structs or an error)
 */
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

/**
 * Returns the path to the user's home directory
 * @ return String (should return the path to the user's home directory)
 */
fn get_home_dir() -> String {
    return env::var("HOME").expect("Failed to get the user's home directory.");
}

/**
 * Returns the path to the data.json file
 * @ return String (should return the path to the data.json file)
 */
fn get_file_path() -> String {
    let home_dir = get_home_dir();
    let file_path = format!("{}/.projects/data.json", home_dir);
    file_path
}

/**
 * Extracts the filename from a path and returns it as a String
 * @ return Result<String, Box<dyn Error>> (should return a String as file.extension or an error)
 */
fn extract_filename(path: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(path);
    let filename_osstr = path.file_name().ok_or("No file name present in path")?;
    let filename_str = filename_osstr
        .to_str()
        .ok_or("Failed to convert filename to string")?;
    Ok(filename_str.to_string())
}

/**
 * Copies the logo to the .projects folder
 * @ return Result<PathBuf, Box<dyn Error>> (should return the path to the logo or an error)
 */
 */
fn copy_logo(project: &Project) -> Result<PathBuf, Box<dyn Error>> {
    let logo_path = match &project.logo {
        Some(path) => path,
        None => return Err("Logo path is None".into()),
    };

    let filename = extract_filename(logo_path)?;

    let destination_path = format!("{}/.projects/{}", get_home_dir(), filename);

    let source_path = Path::new(logo_path);
    let dest_path = Path::new(&destination_path);

    copy(source_path, &dest_path)?;

    Ok(dest_path.to_path_buf())
}

#[tauri::command]
/**
 * Returns the project with the given id
 * @ return Option<Project> (should return the project with the given id or None if the project is not found)
 */
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
/**
 * Deletes the project with the given id
 */
async fn delete_project(id: u32) {
    let mut projects = get_projects();
    projects.retain(|project| project.id != id);
    match store_projects(&projects, &get_file_path()) {
        Ok(_) => println!("Project with ID {} deleted.", id),
        Err(e) => eprintln!("Failed to store projects after deletion: {}", e),
    }
}

#[tauri::command]
/**
 * Returns a vector of all projects
 * @ return Vec<Project> (should return a vector of all projects)
 */
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

/**
 * Adds a project to the data.json file
 * @ return Result<bool, String> (should return true if the project was added successfully or an error)
 */
#[tauri::command]
fn add_project(mut project: Project) -> Result<bool, String> {
    let mut projects = get_projects();
    if projects.len() == 0 {
        project.id = 1;
    } else {
        project.id = projects[projects.len() - 1].id + 1;
    }
    match copy_logo(&project) {
        Ok(path) => project.logo = Some(path.to_str().unwrap().to_string()),
        Err(e) => eprintln!("Error copying logo: {}", e),
    }
    projects.push(project);
    match store_projects(&projects, &get_file_path()) {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Error storing projects: {}", e)),
    }
}

/**
 * Runs the Tauri application
 */
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
