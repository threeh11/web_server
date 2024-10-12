use std::path::Path;
use tokio::net::TcpListener;
use std::net::{SocketAddr};
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

}