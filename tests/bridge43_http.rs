//! Direct HTTP-0 socket regression of the sparse 3^16 address contract.
#![forbid(unsafe_code)]
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn get(path: &str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback only");
    let addr = listener.local_addr().expect("bound address");
    let task = thread::spawn(move || {
        let (stream, _) = listener.accept().expect("one incoming request");
        let state = vec![1u8; hebc::N];
        hebc::server::handle_client(stream, &state, 0, "test-only", 0, 0).expect("HTTP-0 handler");
    });
    let mut conn = TcpStream::connect(addr).expect("connect loopback");
    let request = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\n\r\n");
    conn.write_all(request.as_bytes()).expect("write request");
    conn.shutdown(Shutdown::Write).expect("finish request");
    let mut response = String::new();
    conn.read_to_string(&mut response).expect("read response");
    task.join().expect("handler terminated");
    response
}

#[test]
fn portal_contract_is_exact_power_of_three_and_sparse() {
    let body = get("/bridge/43m/contract");
    assert!(body.starts_with("HTTP/1.1 200 OK"));
    assert!(body.contains("capacity=43046721"));
    assert!(body.contains("power=3^16"));
    assert!(body.contains("occupied=729"));
    assert!(body.contains("VIRTUAL_ADDRESSES_NOT_INDEPENDENT_FLUID_STATES"));
}

#[test]
fn unpopulated_and_invalid_addresses_are_not_fabricated() {
    let empty = get("/bridge/43m/address/81");
    assert!(empty.starts_with("HTTP/1.1 404 Not Found"));
    assert!(empty.contains("UNPOPULATED"));
    let upper = get("/bridge/43m/address/43046720");
    assert!(upper.starts_with("HTTP/1.1 404 Not Found"));
    let too_large = get("/bridge/43m/address/43046721");
    assert!(too_large.starts_with("HTTP/1.1 400 Bad Request"));
    let invalid = get("/bridge/43m/address/nan");
    assert!(invalid.starts_with("HTTP/1.1 400 Bad Request"));
}

#[test]
fn populated_values_require_valid_receipts() {
    let installed = std::env::var_os("HEB_C_CLAY_LEAVES_DIR").is_some();
    let response = get("/bridge/43m/address/0");
    if installed {
        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(response.contains("signed=357|residue=357"));
    } else {
        assert!(response.starts_with("HTTP/1.1 503 Service Unavailable"));
        assert!(response.contains("RECEIPTS_NOT_VERIFIED"));
    }
}
