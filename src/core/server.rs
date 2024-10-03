use std::collections::HashMap;
use std::convert::Infallible;
use std::{fs};
use uuid::{ContextV7, Timestamp, Uuid};
use std::net::{SocketAddr};
use std::path::Path;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response};
use tokio::task::JoinHandle;
use crate::config::jexus_config::Server;

pub struct ServerD {
    pub uuid: Uuid,
    pub port: u16,
    pub socket_addr: SocketAddr,
    pub root_dir: &'static Path,
    pub tasks_connection: HashMap<Uuid, JoinHandle<()>>,
}

impl ServerD {
    pub fn new(server: &Server) -> Self {
        let port = server.listen as u16;
        let uuid: Uuid = Uuid::new_v7(Timestamp::from_unix(ContextV7::new(), 1497624119, 1234));
        Self {
            uuid,
            port,
            socket_addr: SocketAddr::from(([127, 0, 0, 1], port)), //?????
            root_dir: Path::new(&server.root),
            tasks_connection: HashMap::new(),
        }
    }

    pub async fn handle_connection(request: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
        let user_agent = request.headers().get("User-Agent").map(|value| value.to_str().unwrap_or("Unknown")).unwrap_or("Unknown");
        // info!("Request: {} {} - User-Agent: {}", request.method(), request.uri(), user_agent);
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
}