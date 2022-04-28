#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use league_client_connector::LeagueClientConnector;
use league_client_connector::RiotLockFile;
use tungstenite::{connect, Message};
use url::Url;
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
fn error(window: tauri::Window) {
  println!("ERROR");
  // Create splashscreen
  if let Some(splash) = window.get_window("splash") {
    splash.show().unwrap();
  }
  // Hide main window
  window.get_window("main").unwrap().hide().unwrap();
}

#[tauri::command]
fn get_credentials() -> Result<RiotLockFile, String> {
  let lockfile = LeagueClientConnector::parse_lockfile();
  match lockfile {
    Ok(lockfile) => Ok(lockfile.into()),
    Err(_) => Err("error".into())
  }
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
}

#[tauri::command]
async fn post(endpoint: String, port: serde_json::Number, password: String, data: serde_json::Value) -> serde_json::Value {
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

  println!("{}", data);

  let body = client.put(format!("https://127.0.0.1:{}{}", port, endpoint))
    .basic_auth("riot", Some(password))
    .json(&data)
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

  println!("{}", body);

  body.into()
}

#[tauri::command]
async fn ws(port: serde_json::Number, password: String) {

  let formaturl = format!("wss://riot:{}@127.0.0.1:{}", password, port).to_string();
  println!("{}", formaturl);

  let (mut socket, response) =
        connect(Url::parse(&formaturl).unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket.write_message(Message::Text("[5, \"OnJsonApiEvent\"]".into())).unwrap();
    println!("TEST");
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![close_splash, get_credentials, lcu, ws, error, post])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
