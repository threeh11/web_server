use std::path::Path;
use tokio::net::TcpListener;
use std::net::{SocketAddr};
use uuid::{ContextV7, Timestamp, Uuid};

pub struct ServerInstance<'a> {
    pub uuid: Uuid,
    pub port: u16,
    pub socket_addr: SocketAddr,
    pub root_dir: &'a Path,
}

impl ServerInstance {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            uuid: Self::generate_uuid(),
            port: 0,
            socket_addr: "0.0.0.0:0".parse().unwrap(),
            root_dir: Path::new("").into(),
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

    pub fn set_root_dir(&mut self, root_dir: String) -> &mut Self {
        self.root_dir = Path::new(root_dir.as_str()).into();
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

    pub fn build(&self) -> Box<Self> {
        Box::new(Self{
            uuid: self.uuid,
            port: self.port,
            socket_addr: self.socket_addr,
            root_dir: self.root_dir,
        })
    }

    fn generate_uuid() -> Uuid {
        Uuid::new_v7(Timestamp::from_unix(ContextV7::new(), 1497624119, 1234))
    }

}

// pub struct ServerHanler {
//     pub uuid: Uuid,
//     pub port: u16,
//     pub socket_addr: SocketAddr,
//     pub root_dir: &'static Path,
//     pub tasks_connection: HashMap<Uuid, JoinHandle<()>>,
// }
//
// impl ServerHanler {
//     pub fn new(server: &Server) -> Self {
//         let port = server.listen as u16;
//         let uuid: Uuid = Uuid::new_v7(Timestamp::from_unix(ContextV7::new(), 1497624119, 1234));
//         Self {
//             uuid,
//             port,
//             socket_addr: SocketAddr::from(([127, 0, 0, 1], port)), //?????
//             root_dir: Path::new(&server.root),
//             tasks_connection: HashMap::new(),
//         }
//     }
//
//     pub async fn handle_connection(request: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
//         let user_agent = request.headers().get("User-Agent").map(|value| value.to_str().unwrap_or("Unknown")).unwrap_or("Unknown");
//         // info!("Request: {} {} - User-Agent: {}", request.method(), request.uri(), user_agent);
//         let base_path = Path::new("/home/threeh/test");
//         let request_path = Path::new(request.uri().path());
//
//         let request_path_str = request_path.to_str().expect("Invalid Unicode in path");
//
//         let request_path_str = if request_path_str.starts_with('/') {
//             &request_path_str[1..]
//         } else {
//             request_path_str
//         };
//
//         let full_path = base_path.join(Path::new(request_path_str));
//
//         match fs::read(full_path) {
//             Ok(contents) => Ok(Response::new(Full::from(Bytes::from(contents)))),
//             Err(_) => Ok(Response::builder()
//                 .status(StatusCode::NOT_FOUND)
//                 .body(Full::from("File not found"))
//                 .unwrap()),
//         }
//     }
//
//     pub async fn proxy(request: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
//
//     }
// }