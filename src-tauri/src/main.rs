#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::ops::Deref;
use std::path::Path;
use std::sync::Mutex;
use bollard::{API_DEFAULT_VERSION, Docker};
use bollard::errors::Error;
use bollard::image::ListImagesOptions;
use bollard::models::{ImageSummary, SystemInfo};
use tauri::State;
use serde_json::{json, Value};
use tauri::api::dialog::blocking::FileDialogBuilder;

struct Connection(Mutex<Option<Docker>>);

fn main() {
    tauri::Builder::default()
        .manage(Connection(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            open_file_selection_and_get_file_path,
            create_docker_socket_connection,
            create_docker_http_connection,
            create_docker_ssl_connection,
            get_docker_daemon_info,
            get_docker_images
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Create a new HTTP connection to a docker daemon
///
/// # Arguments
/// * `conn` - The connection state
/// * `addr` - The docker daemon address
/// * `timeout` - The time (in seconds) that
#[tauri::command]
async fn create_docker_http_connection(conn: State<'_, Connection>, addr: String, timeout: i32) -> Result<String, Value> {
    let connection = Docker::connect_with_http(
        addr.as_str(),
        timeout as u64,
        API_DEFAULT_VERSION
    );
    check_docker_connection_details(connection, conn, addr).await
}

/// Create a new Socket connection to a docker daemon
///
/// # Arguments
/// * `conn` - The connection state
/// * `addr` - The docker daemon address
/// * `timeout` - The time (in seconds) that
#[tauri::command]
async fn create_docker_socket_connection(conn: State<'_, Connection>, socket_path: String, timeout: i32) -> Result<String, Value> {
    let connection = Docker::connect_with_socket(
        socket_path.as_str(),
        timeout as u64,
        API_DEFAULT_VERSION
    );
    check_docker_connection_details(connection, conn, socket_path).await
}

/// Create a new SSL connection to a docker daemon
///
/// # Arguments
/// * `conn` - The connection state
/// * `addr` - The docker daemon address
/// * `ssl_key` - Path to key
/// * `ssl_cert` - Path to certificate
/// * `ssl_ca` - Path to CA
/// * `timeout` - The time (in seconds) that
///
#[tauri::command]
async fn create_docker_ssl_connection(
    conn: State<'_, Connection>,
    addr: String,
    ssl_key: String,
    ssl_cert: String,
    ssl_ca: String,
    timeout: i32
) -> Result<String, Value> {
    let connection = Docker::connect_with_ssl(
        addr.as_str(),
        Path::new(ssl_key.as_str()),
        Path::new(ssl_cert.as_str()),
        Path::new(ssl_ca.as_str()),
        timeout as u64,
        API_DEFAULT_VERSION
    );
    check_docker_connection_details(connection, conn, addr).await
}


/// Check a given connection, and if valid, update connection state
///
/// # Arguments
/// * `connection` - The connection to test
/// * `conn` - The global state for connection
/// * `addr` - The address/path that is being connected to. This is just for the messages
async fn check_docker_connection_details(connection: Result<Docker, Error>, conn: State<'_, Connection>, addr: String) -> Result<String, Value> {
    let connection = match connection {
        Ok(conn) => {
            println!("Arguments accepted. Trying to connect to {}", addr);
            conn
        },
        Err(err) => {
            println!("Failed while trying to create a new connection! Error: {:?}", err);
            return Err(check_docker_errors(err))
        }
    };
    let ping = connection.ping().await;
    match ping {
        Ok(reply) => {
            println!("Successfully connected to {} and received a reply: {}", addr, reply);
            *conn.0.lock().unwrap() = Some(connection);
            Ok(reply)
        },
        Err(err) => {
            println!("Error when connecting to {}. Error: {:?}", addr, err);
            Err(check_docker_errors(err))
        }
    }
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
async fn get_docker_images(conn: State<'_, Connection>) -> Result<Vec<ImageSummary>, Value> {
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        return Err(json!({"error":"No docker connection!"}))
    }
    let docker = docker_option.unwrap();
    match docker.list_images(Some(ListImagesOptions::<String> {
        all: true,
        ..Default::default()
    })).await {
        Ok(images) => { Ok(images) },
        Err(e) => { Err(check_docker_errors(e)) }
    }
}

/// Make the user select a file and return its path
///
/// # Arguments
/// * `filter_name` - Name of the filter shown in file explorer
/// * `extensions` - Array of accepted extensions
#[tauri::command]
async fn open_file_selection_and_get_file_path(filter_name: String, extensions: Vec<&str>) -> Result<String, String> {
    let allowed_extensions = extensions.as_slice();
    let file = FileDialogBuilder::new()
        .add_filter(filter_name, allowed_extensions)
        .pick_file();
    if file.is_none() {
        Err("".to_string())
    } else {
        let file_path = file.unwrap().into_os_string().into_string().unwrap();
        Ok(file_path)
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