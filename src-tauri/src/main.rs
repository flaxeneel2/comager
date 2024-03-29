#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::ops::Deref;
use std::path::Path;

use std::sync::{Mutex};
use std::sync::mpsc::channel;
use bollard::{API_DEFAULT_VERSION, Docker};
use bollard::container::{AttachContainerOptions, Config, CreateContainerOptions, InspectContainerOptions, ListContainersOptions, RemoveContainerOptions, RestartContainerOptions, StartContainerOptions, StopContainerOptions};
use bollard::errors::Error;
use bollard::image::{CreateImageOptions, ListImagesOptions, RemoveImageOptions};
use bollard::models::{ContainerCreateResponse, ContainerInspectResponse, ContainerSummary, HostConfig, ImageDeleteResponseItem, ImageSummary, SystemInfo};

use tauri::{AppHandle, Manager, State};
use serde_json::{json, Value};
use tauri::api::dialog::blocking::FileDialogBuilder;
use futures::stream::StreamExt;
use tokio::io::AsyncWriteExt;


/// The Connection state. This will be used to store an existing connection.
struct Connection(Mutex<Option<Docker>>);

/// The main function. All the commands must be registered here
fn main() {
    tauri::Builder::default()
        .manage(Connection(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            open_file_selection_and_get_file_path,
            create_docker_socket_connection,
            set_container_up_for_live_stdio,
            create_docker_http_connection,
            create_docker_ssl_connection,
            get_docker_container_details,
            add_docker_image_by_name,
            restart_docker_container,
            delete_docker_container,
            create_docker_container,
            start_docker_container,
            get_docker_daemon_info,
            stop_docker_container,
            get_docker_containers,
            delete_docker_image,
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

/// Get the information about the docker daemon and the host
///
/// # Arguments
/// * `conn` - The global connection state.
///
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

/// Add a new image by name.
/// It can be from any repository, whether official or unofficial, as long as it returns a valid image.
///
/// # Arguments
/// * `conn` - The connection state.
/// * `app_handle` - The app handle for tauri
/// * `image_name` - The name of the image to be deleted.
/// * `unique_id` - The unique ID to broadcast to
///
#[tauri::command]
async fn add_docker_image_by_name(conn: State<'_, Connection>, app_handle: AppHandle, image_name: &str, unique_id: &str) -> Result<(), Value> {
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        Err(json!({"error":"No docker connection!"}))
    } else {
        let docker = docker_option.unwrap();
        let mut stream = docker.create_image(
            Some(CreateImageOptions {
                from_image: image_name,
                ..Default::default()
            }),
            None,
            None
        );
        while let Some(item) = stream.next().await {
            match item {
                Ok(data) => {
                    app_handle.app_handle().emit_all(unique_id, data).expect("TODO: panic message");
                },
                Err(err) => {
                    return Err(check_docker_errors(err));
                }
            }
        }
        Ok(())
    }
}

/// Delete a given image
///
/// # Arguments
/// * `conn` - The connection state.
/// * `image_name` - The name of the image to be deleted.
/// * `force` - Whether the image should be force deleted or not.
///
#[tauri::command]
async fn delete_docker_image(conn: State<'_, Connection>, image_name: &str, force: bool) -> Result<Vec<ImageDeleteResponseItem>, Value> {
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        Err(json!({"error":"No docker connection!"}))
    } else {
        let delete_response = docker_option.unwrap().remove_image(
            image_name,
            Some(RemoveImageOptions {
                force,
                ..Default::default()
            }),
            None
        ).await;
        match delete_response {
            Ok(image_delete_responses) => {
                Ok(image_delete_responses)
            },
            Err(err) =>
            Err(check_docker_errors(err))
        }
    }
}

/// Get the images currently installed on the docker daemon
///
/// # Arguments
/// * `conn` - The global connection state
///
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

/// Get the containers currently installed on the docker daemon
///
/// # Arguments
/// * `conn` - The global connection state
///
#[tauri::command]
async fn get_docker_containers(conn: State<'_, Connection>) -> Result<Vec<ContainerSummary>, Value> {
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        return Err(json!({"error":"No docker connection!"}))
    }
    let docker = docker_option.unwrap();
    match docker.list_containers(Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    })).await {
        Ok(containers) => { Ok(containers) }
        Err(e) => { Err(check_docker_errors(e)) }
    }
}

/// Get the container details from daemon
///
/// # Arguments
/// * `conn` - The global connection state
///
#[tauri::command]
async fn get_docker_container_details(conn: State<'_, Connection>, container_id: &str) -> Result<ContainerInspectResponse, Value> {
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        return Err(json!({"error":"No docker connection!"}))
    }
    let docker = docker_option.unwrap();
    match docker.inspect_container(container_id, Some(InspectContainerOptions {
        size: false
    })).await {
        Ok(container_info) => { Ok(container_info) }
        Err(e) => { Err(check_docker_errors(e)) }
    }
}

/// Delete a given container
///
/// # Arguments
/// * `conn` - The connection state.
/// * `container_id` - The id of the container to be deleted.
/// * `force` - Whether the container should be force deleted or not. (If container is running, it is killed and then deleted)
/// * `volumes` - Whether the volumes linked to the container should be deleted too
/// * `links` - Whether links should be deleted too.
///
#[tauri::command]
async fn delete_docker_container(conn: State<'_, Connection>,
                                 container_id: &str,
                                 force: bool,
                                 volumes: bool,
                                 links: bool
) -> Result<(), Value> {
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        Err(json!({"error":"No docker connection!"}))
    } else {
        let delete_response = docker_option.unwrap().remove_container(
            container_id,
            Some(RemoveContainerOptions {
                force,
                v: volumes,
                link: links,
            })
        ).await;
        match delete_response {
            Ok(_) => {
                Ok(())
            },
            Err(err) =>
                Err(check_docker_errors(err))
        }
    }
}

/// Create a new docker container
///
/// # Arguments
/// * `conn` - The connection state.
/// * `name` - The name of the container
/// * `image` - The image to create the container from
/// * `cpu_percentage_limit` - CPU percentage limit.
/// * `memory_limit` - Memory the container is allowed to use
///
#[tauri::command]
async fn create_docker_container(conn: State<'_, Connection>,
                                 name: &str,
                                 image: &str,
                                 cpu_percentage_limit: i64,
                                 memory_limit: i64
) -> Result<ContainerCreateResponse, Value> {
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        Err(json!({"error":"No docker connection!"}))
    } else {
        let docker = docker_option.unwrap();
        let host_config: Option<HostConfig> = Some(HostConfig {
            nano_cpus: Some(cpu_percentage_limit*10000000),
            memory: Some(memory_limit),
            ..Default::default()
        });
        let config = Config {
            host_config,
            image: Some(image),

            ..Default::default()
        };
        let create_container_options = CreateContainerOptions {
            name,
            ..Default::default()
        };
        match docker.create_container(Some(create_container_options), config).await {
            Ok(container_created) => {
                Ok(container_created)
            },
            Err(e) => {
                Err(check_docker_errors(e))
            }
        }
    }
}

///
/// Register a container for stdio.
///
/// # Arguments
/// * `conn` - The connection state
/// * `app_handle` - The handle for the app
/// * `container_id` - The container ID to register
/// * `unique_id` - The ID for the listeners
#[tauri::command]
async fn set_container_up_for_live_stdio(conn: State<'_, Connection>,
                                         app_handle: AppHandle,
                                         container_id: &str,
                                         unique_id: &str
) -> Result<(), Value> {
    println!("{}", "called!");
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        Err(json!({"error":"No docker connection!"}))
    } else {
        let docker = docker_option.unwrap();
        let mut should_quit = false;
        let attach_point = docker.attach_container(container_id, Some(AttachContainerOptions::<String>{
            stdin: Some(true),
            stdout: Some(true),
            stderr: Some(true),
            stream: Some(true),
            logs: Some(true),
            detach_keys: Some("ctrl-c".to_string()),
        })).await;
        match attach_point {
            Ok(res) => {
                let mut output = res.output;
                let mut input = res.input;

                let (tx, rx) = channel();
                let ctx = Mutex::new(tx);
                let listener = app_handle.listen_global(unique_id, move |event| {
                    println!("{}", "hi");
                    ctx.lock().unwrap().deref().send(event.clone().payload().unwrap().to_string()).unwrap();
                });
                for msg in rx.iter() {
                    if msg.starts_with("END_TUNNEL|") {
                        println!("{}", "quitting tunnel");
                        should_quit = true
                    } else {
                        let mut send = msg.split("|").nth(1).unwrap().as_bytes();
                        let _ = input.write_all_buf(&mut send);
                    }
                }

                while let Some(data) = output.next().await {
                    if should_quit {
                        app_handle.unlisten(listener);
                        break
                    }
                    println!("{}", "there was some data from output");
                    if data.is_ok() {
                        let data = data.unwrap();
                        println!("{}", data);
                        app_handle.emit_all(unique_id, data.to_string()).expect("TODO: panic message");
                    }
                }
            },
            Err(err) => {
                return Err(check_docker_errors(err))
            }
        }
        Ok(())
    }
}


///
/// Start a container
///
/// # Arguments
/// * `conn` - The internal connection state
/// * `container_id` - The ID of the container to start
#[tauri::command]
async fn start_docker_container(conn: State<'_, Connection>, container_id: &str) -> Result<(), Value> {
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        return Err(json!({"error":"No docker connection!"}))
    }
    let docker = docker_option.unwrap();
    let req = docker.start_container(container_id, Some(StartContainerOptions {
        detach_keys: "ctrl-^"
    })).await;
    match req {
        Ok(()) => {
            Ok(())
        },
        Err(err) => {
            Err(check_docker_errors(err))
        }
    }
}

///
/// Stop a container
///
/// # Arguments
/// * `conn` - The internal connection state
/// * `container_id` - The ID of the container to stop
#[tauri::command]
async fn stop_docker_container(conn: State<'_, Connection>, container_id: &str) -> Result<(), Value> {
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        return Err(json!({"error":"No docker connection!"}))
    }
    let docker = docker_option.unwrap();
    match docker.stop_container(container_id, Some(StopContainerOptions {
        t: 10
    })).await {
        Ok(()) => {
            Ok(())
        },
        Err(err) => {
            Err(check_docker_errors(err))
        }
    }
}

///
/// Retart a container
///
/// # Arguments
/// * `conn` - The internal connection state
/// * `container_id` - The ID of the container to restart
#[tauri::command]
async fn restart_docker_container(conn: State<'_, Connection>, container_id: &str) -> Result<(), Value> {
    let docker_option: Option<Docker> = conn.0.lock().unwrap().deref().clone();
    if docker_option.is_none() {
        return Err(json!({"error":"No docker connection!"}))
    }
    let docker = docker_option.unwrap();
    match docker.restart_container(container_id, Some(RestartContainerOptions {
        t: 10
    })).await {
        Ok(()) => {
            Ok(())
        },
        Err(err) => {
            Err(check_docker_errors(err))
        }
    }
}
/// Check bollard errors and convert then to JSON so they can be sent to the frontend
///
/// # Arguments
/// * `err` - The error to check
///
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