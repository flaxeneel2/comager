#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]


use std::ops::Deref;
use std::sync::Mutex;
use bollard::{API_DEFAULT_VERSION, Docker};
use bollard::container::ListContainersOptions;
use bollard::errors::Error;
use futures::stream::StreamExt;

use bollard::image::{CreateImageOptions, ListImagesOptions};
use bollard::models::{ContainerSummary, ImageSummary};
use serde_json::{json, Value};
use tauri::{AppHandle, Manager, State};

struct Connection(Mutex<Docker>);

struct ConnectionDetails {
    connection_type: Mutex<i8>
}


fn main() {
    tauri::Builder::default()
        .manage(Connection(Mutex::new(create_docker_connection())))
        .invoke_handler(tauri::generate_handler![get_docker_containers, get_docker_images, install_docker_image_from_repo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}

fn create_docker_connection() -> Docker {
    Docker::connect_with_http("http://localhost:16384", 4, API_DEFAULT_VERSION).unwrap()
}

fn update_docker_connection_details() {

}

#[tauri::command]
async fn get_docker_images(conn_state: State<'_, Connection>) -> Result<Vec<ImageSummary>, Value> {
    let docker: Docker = conn_state.0.lock().unwrap().deref().clone();
    let images_unchecked = docker.list_images(Some(ListImagesOptions::<String> {
        all: true,
        ..Default::default()
    })).await;
    match images_unchecked {
        Ok(images) => Ok(images),
        Err(err) => Err(check_docker_errors(err)),
    }
}

#[tauri::command]
async fn get_docker_containers(conn_state: State<'_, Connection>) -> Result<Vec<ContainerSummary>, Value> {
    let docker: Docker = conn_state.0.lock().unwrap().deref().clone();
    let containers_unchecked = docker.list_containers(Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    })).await;
    match containers_unchecked {
        Ok(containers) => Ok(containers),
        Err(err) => Err(check_docker_errors(err)),
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

#[tauri::command]
async fn install_docker_image_from_repo(conn_state: State<'_, Connection>, image_name: String, app_handle: AppHandle) ->  Result<bool, Value> {
    println!("Called!");
    let docker: Docker = conn_state.0.lock().unwrap().deref().clone();
    let create_image_opts = CreateImageOptions {
        from_image: image_name,
        ..Default::default()
    };
    let mut stream_unchecked = docker.create_image(Some(create_image_opts), None, None);
    while let Some(item) = stream_unchecked.next().await {
        match item {
            Ok(data) => {
                app_handle.app_handle().emit_all("hi", data).expect("TODO: panic message");
            },
            Err(err) => {
                app_handle.emit_all("image-download-error", check_docker_errors(err)).expect("ERRORED WHILE ERRORING");
                break
            }
        }
    }
    Ok(true)
}
