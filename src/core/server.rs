use std::convert::Infallible;
use std::fs;
use std::path::Path;
use tokio::net::TcpListener;
use std::net::{SocketAddr};
use bytes::Bytes;
use http_body_util::Full;
use hyper::{body, Request, Response, StatusCode};
use uuid::{ContextV7, Timestamp, Uuid};
use crate::core::location::LocationInstance;

// #[derive(Eq, Hash, PartialEq)]
#[derive(Clone)]
pub struct VirtualHost<'a> {
    pub uuid: Uuid,
    // pub server_name: &'a str,
    pub port: u16,
    pub socket_addr: SocketAddr,
    pub root_dir: &'a Path,
    pub locations: Box<Vec<LocationInstance>>
}

impl VirtualHost<'_> {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            uuid: Self::generate_uuid(),
            port: 0,
            socket_addr: "0.0.0.0:0".parse().unwrap(),
            root_dir: Path::new("").into(),
            locations: Box::new(Vec::new()),
        })
    }

    pub fn set_port(&mut self, port: u16) -> &mut Self {
        // todo запилить норм проверку
        // if capabilities::is_root() {
        //     self.port = port;
        // }
        self.port = port;
        self
    }


    pub fn set_socket_addr(&mut self, socket_addr: SocketAddr) -> &mut Self {
        // тут бы тоже проверочку
        self.socket_addr = socket_addr;
        self
    }

    pub async fn get_tcp_listener(&mut self) -> TcpListener {
        TcpListener::bind(self.socket_addr).await.unwrap()
    }

    pub fn set_locations(&mut self, locations: Vec<LocationInstance>) -> &mut Self {
        self.locations = Box::new(locations);
        self
    }

    pub fn build(&self) -> Box<Self> {
        Box::new(Self{
            uuid: self.uuid,
            port: self.port,
            socket_addr: self.socket_addr,
            root_dir: self.root_dir,
            locations: self.locations.clone()
        })
    }

    fn generate_uuid() -> Uuid {
        Uuid::new_v7(Timestamp::from_unix(ContextV7::new(), 1497624119, 1234))
    }

    pub async fn handler(&self, request: Request<impl body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
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