pub mod http_proxy_module {
    use crate::jxs::http::index::HttpModuleModule;
    use hyper::{Body, Request, Response, StatusCode};
    use hyper::body::Bytes;
    use std::convert::Infallible;
    use std::path::Path;
    use std::fs;
    use std::future::Future;
    use std::pin::Pin;
    use http_body_util::Full;


    pub struct ProxyModule {
        proxy_pass: String, // todo на какой нибудь удобо читаемый тип
    }

    impl HttpModuleModule for ProxyModule {
        async fn handler(&self, request: Request<Body>) -> Pin<Box<dyn Future<Output = Result<Response<Full<Bytes>>, Infallible>> + Send>> {
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
    
    impl ProxyModule {
        pub fn new() -> Self {
            Self{
                proxy_pass: "".to_string(),
            }
        }

        pub fn set_proxy_pass(&mut self, proxy_pass: String) -> &mut ProxyModule {
            self.proxy_pass = proxy_pass;
            self
        }

        pub fn build(&self) -> Box<Self> {
            Box::new(Self{
                proxy_pass: self.proxy_pass.clone(),
            })
        }
    }

}