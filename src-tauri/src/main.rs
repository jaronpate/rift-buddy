#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use league_client_connector::LeagueClientConnector;
// use league_client_connector::RiotLockFile;
use reqwest::Method;
use tauri::{
  Manager,
};
use std::path::PathBuf;
use std::sync:: Mutex;
use reqwest::Client;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct RiotLockFile {
  pub port: u32,
  pub password: String,
  pub address: String
}

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
  let file_path = get_client_path().or_else(|_| {
    // println!("League client not found");
    return Err("League client not found".to_string());
  })?;

  let mut buf = String::new();
    File::open(file_path).expect("Unable to open")
      .read_to_string(&mut buf).unwrap();
  let keys:Vec<&str> = buf.split(":").collect();
  let lockfile = RiotLockFile {
    port: keys[2].parse::<u32>().unwrap(),
    password: keys[3].to_string(),
    address: String::from("127.0.0.1")
  };
  // println!("{:?}", lockfile);

  *state.credentials.lock().unwrap() = Some(lockfile.into());
  Ok("Client found".into())
}

#[tauri::command]
async fn lcu(state: tauri::State<'_, AppState>, endpoint: String, method: String, data: Option<serde_json::Value>) -> Result<serde_json::Value, String> {
  let password = state.credentials.lock().unwrap().as_ref().unwrap().password.clone();
  let port = state.credentials.lock().unwrap().as_ref().unwrap().port.clone();
  let address = state.credentials.lock().unwrap().as_ref().unwrap().address.clone();
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

  // println!("{:?}", body);

  // println!("https://{}:{}{}", credentials.address, credentials.port, endpoint);
  
  let body = state.client.request(method.clone(), format!("https://{}:{}{}", address, port, endpoint))
  .basic_auth("riot", Some(password.clone()))
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
    .manage(AppState {
      client:  Client::builder().danger_accept_invalid_certs(true).build().unwrap(),
      credentials: Mutex::new(None)
    })
    // .setup(|app| {
    //   let cert_path = app.path_resolver()
    //     .resolve_resource("./riotgames.pem")
    //     .expect("Unable to find cert");
    //   Ok({
    //     // Self signed LCU Cert
    //     let mut buf = Vec::new();
    //     File::open(cert_path).expect("Unable to open")
    //       .read_to_end(&mut buf).unwrap();
    //     let cert = reqwest::Certificate::from_pem(&buf).unwrap();
    //     // Build reqwest client
    //     let client = Client::builder()
    //       .danger_accept_invalid_certs(true)
    //       .add_root_certificate(cert)
    //       .build()
    //       .unwrap();
    //     // Setup app state
    //     app.manage(AppState {
    //       client,
    //       credentials: Mutex::new(None)
    //     });
    //   })

    // })
    .invoke_handler(tauri::generate_handler![close_splash, get_credentials, lcu])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn get_client_path() -> Result<PathBuf, String> {
  use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

  let system = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));

  let mut processes = system.processes_by_name("LeagueClient.exe");

  if let Some(p) = processes.next() {
      if let Some(path) = p.exe().parent() {
          let mut path = path.to_path_buf();
          path.push("lockfile");
          return Ok(path);
      }
  }

  Err("Can't find lockfile. Make sure that the League Client is running.".into())
}