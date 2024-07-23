mod config;
mod server;

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
use crate::config::server_config::{Config, Server};
use std::collections::HashMap;
use std::{env, fs};
use std::path::Path;
use uuid::{ContextV7, Timestamp, Uuid};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new("src/config/config.yaml")?;

    let servers_conf: Vec<Server> = config.http.servers;
    let mut tasks = HashMap::new();

    for server_conf in &servers_conf {
        let uuid: Uuid = Uuid::new_v7(Timestamp::from_unix(ContextV7::new(), 1497624119, 1234));
        let port = String::from(&server_conf.listen).parse()?;
        let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener: TcpListener = TcpListener::bind(&addr).await?;
        let root: String = String::from(&server_conf.root);

        let task = tokio::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap();
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
        });

        tasks.insert(uuid, task);
        println!("Запущен сервер с UUID: {}", uuid);
    }

    println!("Запущено {} серверов", tasks.len());

    // Ожидаем завершения всех задач
    for (_, task) in tasks {
        task.await?;
    }

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