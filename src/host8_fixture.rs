//! Read-only, deterministic Host8 300GB Sector 000 Window 0 generator.
//! This reconstructs 13,824 canonical 8-byte room stubs from the published
//! Clasy X3 / Host8 integer identity codec; no disk or 300GB allocation.
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic, clippy::float_cmp, clippy::cast_precision_loss)]
pub const ROOMS: usize = 13_824;
pub const SEED: u64 = 0x518b_bb47_2a9d_b786;
pub const H27: i128 = 3_812_798_742_493;
pub const TOTAL_ROOMS: i128 = 37_500_000_000;
const M1: u64 = 0xbf58_476d_1ce4_e5b9;
const M2: u64 = 0x94d0_49bb_1331_11eb;

/// Canonical 64-bit bijective mixer, using modulo-2^64 integer arithmetic.
pub fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(M1);
    z = (z ^ (z >> 27)).wrapping_mul(M2);
    z ^ (z >> 31)
}

/// The global n-th canonical Host8 room stub, not the n-th already decoded trit.
pub fn room_word(n: u64) -> Result<u64, &'static str> {
    if i128::from(n) >= TOTAL_ROOMS { return Err("global room outside 300GB plane"); }
    let signed_word = (i128::from(n) - TOTAL_ROOMS / 2 + H27) as u64;
    Ok(mix(signed_word ^ SEED))
}

pub fn sector000_window000_bytes() -> Vec<u8> {
    let mut bytes = Vec::with_capacity(ROOMS * 8);
    for n in 0..ROOMS as u64 {
        bytes.extend_from_slice(&room_word(n).expect("bounded canonical room index").to_le_bytes());
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{unpack_room_stubs, N};
    use crate::sha256::hash_bytes;

    #[test]
    fn canonical_block_matches_published_physical_input_sha() {
        let raw = sector000_window000_bytes();
        assert_eq!(raw.len(), 110_592);
        assert_eq!(
            hash_bytes(&raw),
            "2dfe7d757ecb012475806b86fcdefc602680d042db836ffbcb2bfe924f2a18ee"
        );
        let mut state = vec![0; N];
        let words = raw.chunks_exact(8).map(|b| u64::from_le_bytes(
            b.try_into().expect("8 byte word")
        )).collect::<Vec<_>>();
        unpack_room_stubs(&words, &mut state);
        assert_eq!(
            hash_bytes(&state),
            "846f2b159a1efcf3b3b7c1d1a89ffb1ba839093b180d905e93c1ea377331077f"
        );
    }
    #[test]
    fn canonical_room_boundaries_are_fail_closed() {
        assert!(room_word(0).is_ok());
        assert!(room_word(TOTAL_ROOMS as u64 - 1).is_ok());
        assert!(room_word(TOTAL_ROOMS as u64).is_err());
    }
}
