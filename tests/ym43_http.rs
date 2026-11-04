//! Socket-level verification of the Yang-Mills 3^16 HTTP-0 receipt domain.
//! The test never executes a Yang-Mills solver or creates fake receipts.
#![forbid(unsafe_code)]

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn get(path: &str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local only");
    let addr = listener.local_addr().expect("local socket");
    let task = thread::spawn(move || {
        let (stream, _) = listener.accept().expect("incoming socket");
        let state = vec![1u8; hebc::N];
        hebc::server::handle_client(stream, &state, 0, "source-index-only", 0, 0)
            .expect("HTTP-0 request");
    });
    let mut connection = TcpStream::connect(addr).expect("connect local");
    connection
        .write_all(format!("GET {path} HTTP/1.1\r\nHost: localhost\r\n\r\n").as_bytes())
        .expect("send GET");
    connection.shutdown(Shutdown::Write).expect("close request");
    let mut response = String::new();
    connection.read_to_string(&mut response).expect("read body");
    task.join().expect("handle request");
    response
}

#[test]
fn catalog_tracks_both_models_without_conflation() {
    let c = get("/bridge/43m/catalog");
    assert!(c.starts_with("HTTP/1.1 200 OK"));
    assert!(c.contains("capacity=43046721"));
    assert!(c.contains("clay_evidence_slots=729"));
    assert!(c.contains("yang_mills_evidence_slots=3"));
    assert!(c.contains("occupied_total=732"));
    assert!(c.contains("ym_gap_verdict=OPEN"));
    assert!(c.contains("virtual_not_physical_states=1"));
    let ym = get("/ym/43m/contract");
    assert!(ym.starts_with("HTTP/1.1 200 OK"));
    assert!(ym.contains("gap_verdict=OPEN"));
}

#[test]
fn source_pinned_proof_records_and_global_dispatch() {
    let ready = std::env::var_os("HEB_C_YM43_ROOT").is_some();
    for (id, class) in [(81, "PERRON"), (82, "FULLREBUILD"), (83, "ALLPAIRS")] {
        let scoped = get(&format!("/ym/43m/address/{id}"));
        let global = get(&format!("/bridge/43m/lookup/{id}"));
        if ready {
            assert!(scoped.starts_with("HTTP/1.1 200 OK"));
            assert_eq!(scoped, global);
            assert!(scoped.contains(&format!("class={class}")));
            assert!(scoped.contains("SOURCE_PIN_AND_RECORDED_FIELDS"));
            if class == "PERRON" {
                assert!(scoped.contains("source_gap_verdict=OPEN"));
            }
        } else {
            assert!(scoped.starts_with("HTTP/1.1 503 Service Unavailable"));
            assert!(global.starts_with("HTTP/1.1 503 Service Unavailable"));
        }
    }
}

#[test]
fn invalid_or_unpopulated_address_cannot_supply_yang_mills_certificate() {
    for id in ["84", "14348988", "43046720"] {
        let empty = get(&format!("/ym/43m/address/{id}"));
        assert!(empty.starts_with("HTTP/1.1 404 Not Found"));
    }
    for id in ["43046721", "not-a-number"] {
        let bad = get(&format!("/ym/43m/address/{id}"));
        assert!(bad.starts_with("HTTP/1.1 400 Bad Request"));
    }
    // The original Clay-specific route still excludes the new domain.
    let original = get("/bridge/43m/address/81");
    assert!(original.starts_with("HTTP/1.1 404 Not Found"));
}
