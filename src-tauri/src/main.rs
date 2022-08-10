#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::ops::Deref;
use std::sync::Mutex;
use bollard::{API_DEFAULT_VERSION, Docker};
use bollard::errors::Error;
use bollard::models::SystemInfo;
use tauri::State;
use serde_json::{json, Value};

struct Connection(Mutex<Option<Docker>>);

fn main() {
    tauri::Builder::default()
        .manage(Connection(Mutex::new(create_docker_connection())))
        .invoke_handler(tauri::generate_handler![get_docker_daemon_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_docker_connection() -> Option<Docker> {
    println!("{}", "Connecting....");
    let connection = Docker::connect_with_http("http://192.168.1.69:16384", 4, API_DEFAULT_VERSION);
    let connection = match connection {
        Ok(conn) => {
            println!("{}", "Connected successfully!");
            Some(conn)
        },
        Err(err) => {
            println!("{}", "Connection unsuccessful!");
            println!("{:?}", err);
            None
        }
    };
    connection
}

#[tauri::command]
async fn get_docker_daemon_info(conn: State<'_, Connection>) -> Result<SystemInfo, Value> {
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        Err(json!({"error":"No docker connection!"}))
    } else {
        let info = docker_option.unwrap().info().await;
        match info {
            Ok(sys_info) => {
                Ok(sys_info)
            },
            Err(error) => {
                Err(check_docker_errors(error))
            }
        }
    }
}

#[tauri::command]
async fn get_docker_daemon_info(conn: State<'_, Connection>) -> Result<Images, Value> {
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        Err(json!({"error":"No docker connection!"}))
    } else {
        let info = docker_option.unwrap().info().await;
        match info {
            Ok(sys_info) => {
                Ok(sys_info)
            },
            Err(error) => {
                Err(check_docker_errors(error))
            }
        }
    }
}

fn check_docker_errors(err: Error) -> Value {
    return match err {
        Error::DockerResponseServerError { status_code, message } => json!({"error": "DOCKER_RESPONSE_SERVER_ERROR", "status_code": status_code, "error_msg": message}),
        Error::RequestTimeoutError => json!({"error": "DOCKER_REQUEST_TIMEOUT"}),
        Error::IOError { err } => json!({"error": "IO_ERROR", "error_msg": err.to_string()}),
        Error::HttpClientError { err } => json!({"error": "HTTP_CLIENT_ERROR", "error_msg": err.to_string()}),
        Error::HyperResponseError { err } => json!({"error": "HYPER_RESPONSE_ERROR", "error_msg": err.to_string()}),
        _ => {
            println!("{:?}", err);
            json!({"error": "UNKNOWN_DOCKER_ERROR"})
        }
    }
}