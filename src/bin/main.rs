use server::get_local_ip;
use server::ThreadPool;
use std::io::BufReader;
use std::time::Duration;
use std::{
    fs,
    thread,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn main() {
    let local_ip = get_local_ip().unwrap().to_string();
    let listener = TcpListener::bind(format!("{}:7878", local_ip)).unwrap();
    println!("Сервер запущен по адресу: {}:7878",local_ip);
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Сервер остановлен.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" =>{
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
}
