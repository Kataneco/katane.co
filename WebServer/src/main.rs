use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let mut request_args = request_line.split_whitespace();
    let (request_type, request_path, request_version) = (request_args.next().unwrap_or_default(), request_args.next().unwrap_or_default(), request_args.next().unwrap_or_default());

    if request_type != "GET" && request_version != "HTTP/1.1" { return; }

    //TODO Remove .. in filepaths
    let file = fs::read_to_string("../WebPages".to_owned() + if request_path == "/" { "/index.html" } else { request_path });
    let response = if file.is_ok() {
        let contents = file.unwrap();
        let length = contents.len();
        format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}")
    } else {
        let contents = fs::read_to_string("../WebPages/404.html").unwrap();
        let length = contents.len();
        format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: {length}\r\n\r\n{contents}")
    };

    stream.write_all(response.as_bytes()).unwrap();
}