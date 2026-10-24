//! Complete 244-state physical-input replay. Exact Rust-only SHA comparison
//! against pinned Host8 C++ trajectory for Sector 000 Window 0.
//! No Windows D: volume, JSON, Python, JavaScript or network required.
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic, clippy::float_cmp, clippy::cast_precision_loss)]

use hebc::host8_fixture::sector000_window000_bytes;
use hebc::sha256::hash_bytes;
use hebc::{
    compute_integer_invariants, make_collisions, make_stream_destinations, make_velocities,
    step_universe_backward, step_universe_forward, unpack_room_stubs, K, N, V,
};
const REFERENCE: &str = include_str!("../evidence/physical_sector000_w0000_243_ticks.csv");

#[test]
fn physical_host8_to_native_hebc_244_states_bit_exact() {
    let raw = sector000_window000_bytes();
    assert_eq!(raw.len(), V * 8);
    assert_eq!(
        hash_bytes(&raw),
        "2dfe7d757ecb012475806b86fcdefc602680d042db836ffbcb2bfe924f2a18ee"
    );
    let stubs: Vec<u64> = raw.chunks_exact(8)
        .map(|b| u64::from_le_bytes(b.try_into().expect("8 bytes")))
        .collect();
    let mut state = vec![0u8; N];
    unpack_room_stubs(&stubs, &mut state);

    let lines: Vec<&str> = REFERENCE.lines().collect();
    assert_eq!(lines.len(), 245);
    assert_eq!(lines[0], "tick,state_sha256");
    let expected: Vec<&str> = lines[1..].iter().map(|line| {
        line.split_once(',').expect("CSV row").1
    }).collect();

    let velocities = make_velocities();
    let gates = make_collisions(&velocities);
    assert_eq!(gates.len(), 288);
    let destinations = make_stream_destinations(&velocities);
    let reference_invariants = compute_integer_invariants(&state, &velocities);
    let initial = state.clone();
    let mut scratch = vec![0u8; N];

    assert_eq!(
        hash_bytes(&state), expected[0],
        "tick 0 input differs from pinned Host8 C++ state"
    );
    for tick in 0..243 {
        step_universe_forward(&mut state, &mut scratch, &gates, &destinations, tick);
        assert_eq!(
            hash_bytes(&state), expected[tick + 1],
            "SHA-256 trajectory mismatch at tick {}", tick + 1
        );
        assert_eq!(
            compute_integer_invariants(&state, &velocities), reference_invariants,
            "integer invariants changed at tick {}", tick + 1
        );
        assert!(state.iter().all(|&q| q <= 2), "invalid Q-trit");
    }
    assert_eq!(
        hash_bytes(&state),
        "d33eaf8d8d156377957a42f8536e667b46ce6f22b3cb0b0256147ea74d246426"
    );
    for tick in (0..243).rev() {
        step_universe_backward(&mut state, &mut scratch, &gates, &destinations, tick);
    }
    assert_eq!(state, initial, "243-step Loschmidt reversal failed");
    assert_eq!(state.len(), V * K);
}

#[test]
fn source_csv_has_unique_consecutive_ticks_and_valid_digests() {
    assert_eq!(REFERENCE.lines().count(), 245);
    for (index, line) in REFERENCE.lines().skip(1).enumerate() {
        let (tick, digest) = line.split_once(',').expect("CSV pair");
        assert_eq!(tick.parse::<usize>().expect("tick number"), index);
        assert_eq!(digest.len(), 64);
        assert!(digest.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()));
    }
}
