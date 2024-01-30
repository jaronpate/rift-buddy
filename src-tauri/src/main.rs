// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::sync::Mutex;
use sysinfo::ProcessExt;
use tauri::Manager;

#[derive(Debug, Serialize, Clone)]
pub struct RiotLockFile {
    pub port: u32,
    pub password: String,
    pub address: String,
    pub protocol: String,
    pub username: String,
}

struct AppState {
    client: reqwest::Client,
    credentials: Mutex<Option<RiotLockFile>>,
}

fn get_pages_file(app_handle: tauri::AppHandle) -> std::path::PathBuf {
    // Get app data directory
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("data");
    // Create data directory if it doesn't exist
    if !app_dir.exists() {
        std::fs::create_dir(app_dir.clone()).unwrap();
    }
    // Find or create pages file
    let pages_file = app_dir.join("pages.json");
    // Print pages file path
    // println!("Pages file: {}", pages_file.display());
    if !pages_file.exists() {
        std::fs::write(pages_file.clone(), "[]").unwrap();
    }
    // Return pages file
    return pages_file;
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn initialize(app_handle: tauri::AppHandle) {
    println!("Closing splashscreen");
    // Close splashscreen
    app_handle.get_window("splash").unwrap().close().unwrap();
    print!("Showing main window");
    // Show main window
    app_handle.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
fn get_saved_pages(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    // Get pages file
    let pages_file = get_pages_file(app_handle.clone());
    // Read pages file or return error
    let pages = match std::fs::read_to_string(pages_file.clone()) {
        Ok(pages) => pages,
        Err(_) => return Err("Can't read pages file".into()),
    };
    // Parse and return pages or return error
    let pages: serde_json::Value = match serde_json::from_str(&pages) {
        Ok(pages) => pages,
        Err(_) => return Err("Can't parse pages file".into()),
    };

    return Ok(pages);
}

#[tauri::command]
fn get_client_info(state: tauri::State<AppState>) -> Result<RiotLockFile, String> {
    use sysinfo::{ProcessRefreshKind, RefreshKind, System, SystemExt};

    let system = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );

    let mut processes = system.processes_by_name("LeagueClient.exe");

    // Get first process or return error
    let process =
        match processes.nth(0) {
            Some(process) => process,
            None => return Err(
                "Can't find LeagueClient.exe process. Make sure that the League Client is running."
                    .into(),
            ),
        };

    // Get process path
    let path = process.exe().parent().unwrap().to_path_buf();

    // Get lockfile path
    let path = path.join("lockfile");

    if path.exists() {
        // Read lockfile
        let lockfile = std::fs::read_to_string(path.clone()).unwrap();
        // Split lockfile into parts
        let parts: Vec<&str> = lockfile.split(":").collect();
        // Create RiotLockFile struct
        let lockfile = RiotLockFile {
            port: parts[2].parse::<u32>().unwrap(),
            password: parts[3].to_string(),
            address: "127.0.0.1".to_string(),
            protocol: parts[4].to_string(),
            username: "riot".to_string(),
        };

        // Save lockfile to state
        *state.credentials.lock().unwrap() = Some(lockfile.clone());

        // Print lockfile
        // println!("Lockfile: {:?}", lockfile);

        // Return lockfile
        return Ok(lockfile);
    } else {
        return Err("Can't find lockfile. Make sure that the League Client is running.".into());
    }
}

#[tauri::command]
fn save_page(
    app_handle: tauri::AppHandle,
    id: String,
    page: serde_json::Value,
) -> Vec<serde_json::Value> {
    // Get pages file
    let pages_file = get_pages_file(app_handle.clone());
    // Read pages file
    let pages = std::fs::read_to_string(pages_file.clone()).unwrap();
    // Parse pages file
    let mut pages: Vec<serde_json::Value> = serde_json::from_str(&pages).unwrap();
    // Push array with id and page to pages
    pages.push(serde_json::json!([id, page]));
    // Serialize pages
    let out = serde_json::to_string(&pages).unwrap();
    // Write pages to file
    std::fs::write(pages_file.clone(), out).unwrap();
    // Return pages
    return pages;
}

#[tauri::command]
fn edit_page(
    app_handle: tauri::AppHandle,
    id: String,
    page: serde_json::Value,
) -> Vec<serde_json::Value> {
    // Get pages file
    let pages_file = get_pages_file(app_handle.clone());
    // Read pages file
    let pages = std::fs::read_to_string(pages_file.clone()).unwrap();
    // Parse pages file
    let mut pages: Vec<serde_json::Value> = serde_json::from_str(&pages).unwrap();
    // Modify page preserving array order
    for i in 0..pages.len() {
        if pages[i][0].as_str().unwrap() == id {
            pages[i][1] = page.clone();
        }
    }
    // Serialize pages
    let out = serde_json::to_string(&pages).unwrap();
    // Write pages to file
    std::fs::write(pages_file.clone(), out).unwrap();
    // Return pages
    return pages;
}

#[tauri::command]
fn delete_page(app_handle: tauri::AppHandle, id: String) -> Vec<serde_json::Value> {
    // Get pages file
    let pages_file = get_pages_file(app_handle.clone());
    // Read pages file
    let pages = std::fs::read_to_string(pages_file.clone()).unwrap();
    // Parse pages file
    let mut pages: Vec<serde_json::Value> = serde_json::from_str(&pages).unwrap();
    // Filter pages
    pages = pages
        .into_iter()
        .filter(|page| page[0].as_str().unwrap() != id)
        .collect();
    // Serialize pages
    let out = serde_json::to_string(&pages).unwrap();
    // Write pages to file
    std::fs::write(pages_file.clone(), out).unwrap();
    // Return pages
    return pages;
}

#[tauri::command]
async fn lcu(
    state: tauri::State<'_, AppState>,
    endpoint: String,
    method: String,
    data: Option<serde_json::Value>,
) -> Result<serde_json::Value, String> {
    let credentials = state.credentials.lock().unwrap().clone().unwrap();
    let method = match method.to_string().to_uppercase().as_ref() {
        "GET" => reqwest::Method::GET,
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        _ => reqwest::Method::GET,
    };
    let body = match data {
        Some(data) => Some(data),
        None => Some(serde_json::Value::Null),
    };

    // println!("{:?}", body);

    // println!("https://{}:{}{}", credentials.address, credentials.port, endpoint);

    let body = state
        .client
        .request(
            method.clone(),
            format!(
                "https://{}:{}{}",
                credentials.address, credentials.port, endpoint
            ),
        )
        .basic_auth(credentials.username, credentials.password.into())
        .json(&body)
        .send()
        .await
        .expect("Failed to send request");

    // println!("{:?}", body);

    let parsed = match method == reqwest::Method::GET || method == reqwest::Method::POST {
        true => body
            .json::<serde_json::Value>()
            .await
            .expect("Failed to parse response"),
        false => serde_json::Value::Null,
    };

    Ok(parsed.into())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            client: reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
            credentials: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            initialize,
            get_saved_pages,
            get_client_info,
            save_page,
            edit_page,
            delete_page,
            lcu
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
