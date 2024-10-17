use std::collections::HashMap;
use std::error::Error;
use hyper::service::service_fn;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::{TcpListener};
use std::sync::Arc;
use crate::core::server::VirtualHost;

pub struct JexusServerManager {
    pub servers_by_config: Vec<Box<VirtualHost<'static>>>,
}

impl JexusServerManager {
    pub fn new(servers: Vec<Box<VirtualHost<'static>>>) -> Self {
        Self {
            servers_by_config: servers,
        }
    }

    pub async fn setup_servers(&mut self) -> Result<(), Box<dyn Error>> {
        let mut tasks = HashMap::new();

        let servers_arc = Arc::new(self.servers_by_config.clone());

        for server_conf in &*servers_arc {
            let server_conf_arc = Arc::new(server_conf.clone());
            let listener: TcpListener = TcpListener::bind(server_conf.socket_addr).await?;

            let task = tokio::spawn(async move {
                loop {
                    match listener.accept().await {
                        Ok((stream, _)) => {
                            let io = TokioIo::new(stream);

                            let server_conf = Arc::clone(&server_conf_arc);

                            tokio::spawn(async move {
                                if let Err(err) = http1::Builder::new()
                                    .serve_connection(io, service_fn(move |req| {
                                        let server_conf = Arc::clone(&server_conf);
                                        async move { server_conf.handler(req).await }
                                    }))
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

            tasks.insert(server_conf.uuid, task);
            println!("Запущен сервер с UUID: {}", server_conf.uuid);
        }

        println!("Запущено {} серверов", tasks.len());

        // Ожидаем завершения всех задач
        for (_, task) in tasks {
            if let Err(e) = task.await {
                println!("Task failed: {:?}", e);
            }
        }

        Ok(())
    }
}