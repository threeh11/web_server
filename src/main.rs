mod config;

use bytes::Bytes;
use std::convert::Infallible;
use http_body_util::Full;

use std::net::{SocketAddr};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener};
use hyper::{Request, Response, StatusCode};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::{TokioIo};
use crate::config::server_config::{Config, Server, WorkerProcesses};
use std::collections::HashMap;
use std::{fs};
use std::path::Path;
use uuid::{ContextV7, Timestamp, Uuid};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new("src/config/config.yaml")?;

    let servers_conf: Vec<Server> = config.http.servers;
    let worker_processes: WorkerProcesses = config.main.worker_processes;
    let worker_threads: usize;
    let num_cpus: usize = num_cpus::get();
    match worker_processes {
        WorkerProcesses::Auto => {
            worker_threads = num_cpus;
        }
        WorkerProcesses::Number(worker_processes_count ) => {
            if (worker_processes_count as usize > num_cpus + 10) {
                panic!("worker_processes - set value, exceeding the number of cores by 10");
            }
            worker_threads = worker_processes_count as usize;
        }
    }
    let mut tasks = HashMap::new();

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(worker_threads)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            for server_conf in &servers_conf {
                let uuid: Uuid = Uuid::new_v7(Timestamp::from_unix(ContextV7::new(), 1497624119, 1234));
                let port: u16 = server_conf.listen as u16;
                let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));
                let listener: TcpListener = TcpListener::bind(&addr).await?;
                let root: String = String::from(&server_conf.root);

                let task = tokio::spawn(async move {
                    loop {
                        match listener.accept().await {
                            Ok((stream, _)) => {
                                let io = TokioIo::new(stream);

                                tokio::task::spawn(async move {
                                    if let Err(err) = http1::Builder::new()
                                        .serve_connection(io, service_fn(handle_connection))
                                        .await
                                    {
                                        println!("Error serving connection: {:?}", err);
                                    }
                                });
                            }
                            Err(e) => {
                                println!("Failed to accept connection: {}", e);
                            }
                        }
                    }
                });

                tasks.insert(uuid, task);
                println!("Запущен сервер с UUID: {}", uuid);
            }

            println!("Запущено {} серверов", tasks.len());

            // Ожидаем завершения всех задач
            for (_, task) in tasks {
                if let Err(e) = task.await {
                    println!("Task failed: {:?}", e);
                }
            }

            Ok::<(), Box<dyn std::error::Error>>(())
        })?;

    Ok(())
}

async fn handle_connection(request: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    let base_path = Path::new("/home/threeh/test");
    let request_path = Path::new(request.uri().path());

    let request_path_str = request_path.to_str().expect("Invalid Unicode in path");

    let request_path_str = if request_path_str.starts_with('/') {
        &request_path_str[1..]
    } else {
        request_path_str
    };

    let full_path = base_path.join(Path::new(request_path_str));

    match fs::read(full_path) {
        Ok(contents) => Ok(Response::new(Full::from(Bytes::from(contents)))),
        Err(_) => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::from("File not found"))
            .unwrap()),
    }
}