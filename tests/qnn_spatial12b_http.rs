//! Loopback HTTP contract for QNN spatial12b read-only address routes.
#![forbid(unsafe_code)]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn request(path: &str) -> String {
    let server = TcpListener::bind("127.0.0.1:0").expect("loopback listener");
    let addr = server.local_addr().expect("local address");
    let worker = thread::spawn(move || {
        let (stream, _) = server.accept().expect("accept");
        hebc::server::handle_client(stream, &[], 0, "unused", 0, 0)
            .expect("native HTTP0 request handler");
    });
    let mut client = TcpStream::connect(addr).expect("connect");
    client
        .write_all(
            format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
                .as_bytes(),
        )
        .expect("send request");
    let mut reply = String::new();
    client.read_to_string(&mut reply).expect("receive response");
    worker.join().expect("handle request");
    reply
}

#[test]
fn qnn_http_contract_is_read_only() {
    let reply = request("/qnn/spatial12b/contract");
    assert!(reply.starts_with("HTTP/1.1 200 OK"));
    assert!(reply.contains("|capacity=12000000000|"));
    assert!(reply.contains("|weights=NOT_LOADED|"));
    assert!(reply.contains("|physics_engine=UNCHANGED|"));
}

#[test]
fn qnn_http_lookup_matches_hosted_qnn_address() {
    let reply = request("/qnn/spatial12b/address/11999999999");
    assert!(reply.starts_with("HTTP/1.1 200 OK"));
    assert!(reply.contains("|x=796|y=875|z=423|channel=11|"));
    assert!(reply.contains("|ternary22=1010222022010011100102|"));
    assert!(reply.contains("|state=UNAVAILABLE|"));
}

#[test]
fn qnn_http_rejects_bad_or_nonexistent_addresses() {
    let invalid = request("/qnn/spatial12b/address/not-a-number");
    assert!(invalid.starts_with("HTTP/1.1 400 Bad Request"));
    assert!(invalid.contains("|status=BAD_ADDRESS|"));
    let missing = request("/qnn/spatial12b/address/12000000000");
    assert!(missing.starts_with("HTTP/1.1 400 Bad Request"));
    assert!(missing.contains("|status=OUT_OF_BOUNDS|"));
}
