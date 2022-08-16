#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use league_client_connector::LeagueClientConnector;
use league_client_connector::RiotLockFile;
use tauri::{
  Manager,
};

use reqwest::Client;
use std::fs::File;
use std::io::Read;

struct AppState {
  client: Client,
  credentials: RiotLockFile,
}

#[tauri::command]
fn close_splash(window: tauri::Window) {
  println!("Closing splashscreen");
  // Close splashscreen
  if let Some(splash) = window.get_window("splash") {
    splash.close().expect("Failed to close splashscreen");
  }
  // Show main window
  if let Some(win) = window.get_window("main") {
    win.show().expect("Failed to show main window");
  }
}

#[tauri::command]
fn get_credentials() -> Result<RiotLockFile, String> {
  let lockfile = LeagueClientConnector::parse_lockfile();
  match lockfile {
    Ok(lockfile) => Ok(lockfile.into()),
    Err(_) => Err("League Client not found.".into())
  }
}

#[tauri::command]
async fn lcu(state: tauri::State<'_, AppState>, endpoint: String) -> Result<serde_json::Value, String> {
  println!("https://{}:{}{}", state.credentials.address, state.credentials.port, endpoint);
  let body: serde_json::Value = state.client.get(format!("https://{}:{}{}", state.credentials.address, state.credentials.port, endpoint))
  .basic_auth("riot", Some(state.credentials.password.clone()))
  .send()
  .await
  .expect("Failed to send request")
  .json::<serde_json::Value>()
  .await
  .expect("Failed to parse response");

  Ok(body.into())
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let cert_path = app.path_resolver()
        .resolve_resource("./riotgames.pem")
        .expect("Unable to find cert");
      Ok({
        // Self signed LCU Cert
        let mut buf = Vec::new();
        File::open(cert_path).expect("Unable to open")
          .read_to_end(&mut buf).unwrap();
        let cert = reqwest::Certificate::from_pem(&buf).unwrap();
        // Build reqwest client
        let client = Client::builder()
          .add_root_certificate(cert)
          .build()
          .unwrap();
        let credentials = get_credentials().expect("League Client not found.");
        app.manage(AppState {
          client,
          credentials
        });
      })

    })
    .invoke_handler(tauri::generate_handler![close_splash, get_credentials, lcu])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
