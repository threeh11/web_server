use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::header::CONTENT_TYPE;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::fs;
use fastcgi_client::{Client, Params};
use std::net::TcpStream;
use std::io::Cursor;

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

async fn handle_php_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let file_path: String = format!("public{}", "/index.php");

    let stream: TcpStream = TcpStream::connect("127.0.0.1:9000").unwrap();
    let mut client = Client::new(stream, false);

    let params: Params<'_> = Params::new()
        .set_request_method("GET")
        .set_script_name("/index.php")
        .set_script_filename(file_path.as_str())
        .set_request_uri("/index.php")
        .set_document_uri("/index.php")
        .set_remote_addr("127.0.0.1")
        .set_remote_port("9000")
        .set_server_addr("127.0.0.1")
        .set_server_port("80")
        .set_server_name("localhost")
        .set_content_type("")
        .set_content_length("0");

    // Чтение тела запроса в байты
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let mut body_cursor = Cursor::new(body_bytes);

    // Отправка запроса через FastCGI
    let response = client.do_request(&params, &mut body_cursor).unwrap();

    let response_body = response.get_stdout().unwrap();
    let response: Response<Body> = Response::builder()
        .header(CONTENT_TYPE, "text/html")
        .body(Body::from(response_body))
        .unwrap();

    Ok(response)
}

async fn handle_request(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path: &str = request.uri().path();

    // Обработка статических файлов
    if path.ends_with(".html") {
        return handle_html_request(path).await;
    }

    // Обработка PHP-скриптов через FastCGI
    if path.ends_with(".php") {
        return handle_php_request(request).await;
    }

    // Если файл не найден
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap();

    Ok(response)
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