#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use bollard::{API_DEFAULT_VERSION, Docker};

struct Connection(Mutex<Option<Docker>>);

fn main() {
    tauri::Builder::default()
        .manage(Connection(Mutex::new(create_docker_connection())))
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
