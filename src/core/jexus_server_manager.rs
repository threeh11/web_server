use hyper::service::service_fn;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::{TcpListener};
use std::string::String;
use crate::config::jexus_config::{Server};
use crate::core::server::{Server, ServerHanler};

pub struct JexusServerManager<'a> {
    pub servers_by_config: &'a Vec<Server>,
}

impl JexusServerManager<'_> {
    pub fn new(servers: &Vec<Server>) -> Self {
        Self {
            servers_by_config: servers,
        }
    }

    pub async fn setup_servers(&mut self) -> Result<(), String> {
        for server_conf in self.servers_by_config {
            let mut server = ServerHanler::new(&server_conf);
            let listener: TcpListener = TcpListener::bind(&server.socket_addr).await.unwrap();

            let task = tokio::spawn(async move {
                loop {
                    match listener.accept().await {
                        Ok((stream, _)) => {
                            let io = TokioIo::new(stream);

                            tokio::task::spawn(async move {
                                if let Err(err) = http1::Builder::new()
                                    .serve_connection(io, service_fn(Server::handle_connection))
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

            server.tasks_connection.insert(server.uuid, task);
            println!("Запущен сервер с UUID: {}", server.uuid);
        }

        // println!("Запущено {} серверов", server.tasks_connection.len());

        // Ожидаем завершения всех задач
        for (_, task) in self.tasks {
            if let Err(e) = task.await {
                println!("Task failed: {:?}", e);
            }
        }

        Ok::<(), Box<dyn std::error::Error>>(())
    }

}