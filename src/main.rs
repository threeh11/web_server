mod config;
mod server;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

#[tokio::main]
// #[tokio::main] - макрос, создает внутренний runtime под капотом
//
// #[tokio::main]
// async fn main() {
//     println!("hello");
// }

// Аналог

//fn main() {
//     let mut rt = tokio::runtime::Runtime::new().unwrap();
//
//     rt.block_on(async {
//         println!("hello");
//     })
// }
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let config = Config::new("src\\config\\config.yaml")?;

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(mut stream: tokio::net::TcpStream) {
    let contents = String::from("Ура ты попал на сайт");
    let length = contents.len();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).await.unwrap();
}