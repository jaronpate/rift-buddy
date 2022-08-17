#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use league_client_connector::LeagueClientConnector;
use league_client_connector::RiotLockFile;
use reqwest::Method;
use tauri::{
  Manager,
};
use std::sync:: Mutex;
use reqwest::Client;
use std::fs::File;
use std::io::Read;

struct AppState {
  client: Client,
  credentials: Mutex<Option<RiotLockFile>>
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
fn get_credentials(state: tauri::State<AppState>) -> Result<String, String> {
  let lockfile = LeagueClientConnector::parse_lockfile();
  match lockfile {
    Ok(lockfile) => {
      *state.credentials.lock().unwrap() = Some(lockfile.into());
      Ok("Credentials loaded".into())
    },
    Err(_) => Err("League Client not found.".into())
  }
}

#[tauri::command]
async fn lcu(state: tauri::State<'_, AppState>, endpoint: String, method: String, data: Option<serde_json::Value>) -> Result<serde_json::Value, String> {
  let credentials = state.credentials.lock().unwrap().as_ref().expect("Credentials not found").clone();
  let method = match method.to_string().to_uppercase().as_ref() {
    "GET" => Method::GET,
    "POST" => Method::POST,
    "PUT" => Method::PUT,
    "DELETE" => Method::DELETE,
    _ => Method::GET
  };
  let body = match data {
    Some(data) => Some(data),
    None => Some(serde_json::Value::Null)
  };

  println!("{:?}", body);

  // println!("https://{}:{}{}", credentials.address, credentials.port, endpoint);
  
  let body = state.client.request(method.clone(), format!("https://{}:{}{}", credentials.address, credentials.port, endpoint))
  .basic_auth("riot", Some(credentials.password.clone()))
  .json(&body)
  .send()
  .await
  .expect("Failed to send request");

  // println!("{:?}", body);

  let parsed = match method == Method::GET || method == Method::POST {
    true => body.json::<serde_json::Value>().await.expect("Failed to parse response"),
    false => serde_json::Value::Null
  };

  Ok(parsed.into())
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
        app.manage(AppState {
          client,
          credentials: Mutex::new(None)
        });
      })

    })
    .invoke_handler(tauri::generate_handler![close_splash, get_credentials, lcu])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
