mod config;
mod server;

use bytes::Bytes;
use std::convert::Infallible;
use http_body_util::Full;

use std::net::{SocketAddr};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener};
use hyper::{Request, Response};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::{TokioIo};
use crate::config::server_config::{Config, Server};
use std::collections::HashMap;
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
    Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}