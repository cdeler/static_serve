use std::net::{IpAddr, SocketAddr};
use std::net::{TcpListener, TcpStream};

use http::header::HeaderValue;
use http::{Request, Response, StatusCode};
use std::io::Read;
use http::Method;

use httparse;

fn handle_request(request_str: String, socket: TcpStream) {
    let mut headers = [httparse::EMPTY_HEADER; 1];

    let mut request = httparse::Request::new(&mut headers);

    match request.parse(request_str.as_bytes()) {
        Ok(_) => {
            let path = request.path.unwrap();
            println!("{}", path);

            let mut response_headers = [httparse::EMPTY_HEADER; 1];
            let response = httparse::Response::new(&mut response_headers);
        }
        Err(err) => eprintln!("Cannot parse the request due to error {}", err)
    };
}

pub fn run_server(host: String, port: u16, _dirname: String) {
    let ip_addr: IpAddr = host.parse().expect("Cannot parse ip address");

    let socket = SocketAddr::new(ip_addr, port);

    let listener = TcpListener::bind(&socket).expect("Cannot bind the socket");

    listener
        .incoming()
        .for_each(move |sock| {
            let mut socket = sock.unwrap();
            println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());
            let mut data: String = String::new();


            match socket.read_to_string(&mut data) {
                Ok(_) => handle_request(data, socket),
                Err(err) => {
                    eprintln!("Failed to read data from socket due to error {}", err);
                }
            };
        });
}
