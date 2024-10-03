use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode, Client};
use hyper::header::CONTENT_TYPE;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::fs;

async fn handle_html_request(path: &str) -> Result<Response<Body>, Infallible> {
    let file_path: String = format!("public{}", path);
    if let Ok(file) = fs::read(&file_path).await {
        let response = Response::builder()
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from(file))
            .unwrap();
        return Ok(response);
    }

    // Если файл не найден
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap();

    Ok(response)
}

async fn handle_java_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let client = Client::new();
    let uri = format!("http://127.0.0.1:8180{}", req.uri().path());
    let java_req = Request::builder()
        .method(req.method())
        .uri(uri)
        .body(req.into_body())
        .unwrap();

    match client.request(java_req).await {
        Ok(response) => Ok(response),
        Err(e) => {
            eprintln!("Ошибка при проксировании запроса на Java Spring Boot: {}", e);
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap())
        }
    }
}

async fn handle_request(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path: &str = request.uri().path();

    // Обработка статических файлов
    if path.ends_with(".html") {
        return handle_html_request(path).await;
    }

    // Проксирование запросов на Java Spring Boot
    handle_java_request(request).await
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}