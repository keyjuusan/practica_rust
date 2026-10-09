use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use backend_vanilla::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(||{
          manejar_conexion(stream)
        })

        // 2
        // thread::spawn(|| manejar_conexion(stream));
        // 1
        // manejar_conexion(stream)
    }
    println!("Shutting down.");
}

fn manejar_conexion(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    let [method, endpoint, _http_version] = match parts.as_slice() {
        [m, e, h] => [m, e, h],
        _ => panic!("La línea HTTP no tiene exactamente 3 partes"),
    };

    let (status_line, path_file) = manejar_api(method, endpoint);
    // let (status_line, path_file) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "./src/index.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "./src/404.html")
    // };

    let contents = fs::read_to_string(path_file).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\nContent-Length: {length}\n\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    // println!("Peticion http: {http_request:#?}")
}

fn manejar_api<'a>(method: &'a str, endpoint: &'a str) -> (&'a str, &'a str) {
    if method == "GET" {
        match endpoint {
            "/" => ("HTTP/1.1 200 OK", "./src/index.html"),
            "/sleep" => {
                // simulando respuesta lenta
                thread::sleep(Duration::from_secs(10));

                ("HTTP/1.1 200 OK", "./src/index.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "./src/404.html"),
        }
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./src/404.html")
    }
}
