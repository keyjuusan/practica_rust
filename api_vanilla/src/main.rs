use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
    time,
};

fn main() {
    let listener = TcpListener::bind("localhost:3333").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(||
          manejar_request(stream)
        );
    }
}

fn manejar_request(mut stream: TcpStream) {
    thread::sleep(time::Duration::from_secs(10));
    stream.write_all("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Hola desde mi servidor web con Rust</h1>".as_bytes()).unwrap();
}
