use hyper::Request;


pub struct Handlers {
    user_agent: String,
    url: Path // @todo url type
}

impl Handlers {
    pub fn new(request: Request<impl hyper::body::Body>) -> Self {
        let user_agent = request.headers()
            .get("User-Agent")
            .map(|value| value.to_str().unwrap_or("Unknown")).unwrap_or("Unknown");
        let url = Self::build_url(request.uri().path());

        Self {
            user_agent,
            url,
        }
    }

    fn build_url(uri: &Path) -> Path {
        // info!("Request: {} {} - User-Agent: {}", request.method(), request.uri(), user_agent);
        let base_path = Path::new("/home/threeh/test");

        let request_path_str = uri.to_str().expect("Invalid Unicode in path");

        let request_path_str = if request_path_str.starts_with('/') {
            &request_path_str[1..]
        } else {
            request_path_str
        };

        base_path.join(Path::new(request_path_str))
    }

    pub fn handle_connection(&self) {
        match fs::read(self.url) {
            Ok(contents) => Ok(Response::new(Full::from(Bytes::from(contents)))),
            Err(_) => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::from("File not found"))
                .unwrap()),
        }
    }
}
