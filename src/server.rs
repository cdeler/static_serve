use std::net::{IpAddr, SocketAddr};
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

pub fn run_server(host: String, port: u16) {
    let ip_addr: IpAddr = host.parse().expect("Cannot parse ip address");

    let socket = SocketAddr::new(ip_addr, port);

    let listener = TcpListener::bind(&socket).expect("Cannot bind the socket");

    let server = listener
        .incoming()
        .for_each(|socket| {
            println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            // Process socket here.

            Ok(())
        }).map_err(|err| {
            eprintln!("Cannot accept the socket {:?}", err);
        });

    println!("Runnin server on {}:{}", host, port);

    ::tokio::run(server);
}
