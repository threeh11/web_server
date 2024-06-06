use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};
use web_server::ThreadPool;
use std::env;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // Аналог
    // use std::thread;
    // let pool = rayon::ThreadPoolBuilder::new()
    //     .num_threads(4)
    //     .build()
    //     .unwrap();
    let pools = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pools.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let contents = String::from("Ура ты попал на сайт");
    let length = contents.len();

    let response =
        format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}