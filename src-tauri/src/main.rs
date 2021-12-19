#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use league_client_connector::LeagueClientConnector;
use league_client_connector::RiotLockFile;
use tauri::{
  api::{
    path::{resolve_path, BaseDirectory}
  },
  Manager,
};
use std::fs::File;
use std::io::Read;

// Create the command:
#[tauri::command]
fn close_splash(window: tauri::Window) {
  // Close splashscreen
  if let Some(splash) = window.get_window("splash") {
    splash.close().unwrap();
  }
  // Show main window
  window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
fn get_credentials() -> RiotLockFile {
  let lockfile = LeagueClientConnector::parse_lockfile().unwrap();
  lockfile.into()
}

#[tauri::command]
async fn lcu(endpoint: String, port: serde_json::Number, password: String) -> serde_json::Value {
  // Self signed LCU Cert
  let mut buf = Vec::new();
  let context = tauri::generate_context!("./tauri.conf.json");
  let cert_path = resolve_path(
    context.config(),
    context.package_info(),
    "assets/riotgames.pem",
    Some(BaseDirectory::Resource),
  ).unwrap();
  File::open(cert_path.to_string_lossy().to_string()).expect("Unable to open")
    .read_to_end(&mut buf).unwrap();
  let cert = reqwest::Certificate::from_pem(&buf).unwrap();
  // Build reqwest client
  let client = reqwest::Client::builder()
    .add_root_certificate(cert)
    .build()
    .unwrap();

  let body: serde_json::Value = client.get(format!("https://127.0.0.1:{}{}", port, endpoint))
    .basic_auth("riot", Some(password))
    .send()
    .await
    .unwrap()
    .json::<serde_json::Value>()
    .await
    .unwrap();

  body.into()

  // println!("{:#?}", body.to_string());
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![close_splash, get_credentials, lcu])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
