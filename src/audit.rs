//! Fail-closed native HBI/HBP/SHA/SH/HASH receipt-chain verifier.
//! Rechecks canonical record digests, byte offsets and every chain transition.
//! These are integrity checks, not third-party digital signatures.
#![forbid(unsafe_code)]
#![deny(
    clippy::float_arithmetic,
    clippy::float_cmp,
    clippy::cast_precision_loss
)]

use crate::receipts::GENESIS_CHAIN_HASH;
use crate::sha256::hash_bytes;
use crate::{PROBES_PER_STEP, V, VIRTUAL_PORTALS};
use std::collections::BTreeMap;
use std::path::Path;

fn fields(line: &str, prefix: &str) -> Result<BTreeMap<&str, &str>, String> {
    if !line.starts_with(prefix) {
        return Err(format!("wrong record type: {prefix}"));
    }
    let mut out = BTreeMap::new();
    for pair in line.split('|').skip(1) {
        let (k, v) = pair.split_once('=').ok_or("noncanonical field")?;
        if k.is_empty() || v.is_empty() || out.insert(k, v).is_some() {
            return Err("empty or duplicate field".into());
        }
    }
    Ok(out)
}

fn get<'a>(m: &'a BTreeMap<&str, &str>, k: &str) -> Result<&'a str, String> {
    m.get(k).copied().ok_or_else(|| format!("missing {k}"))
}
fn number(m: &BTreeMap<&str, &str>, k: &str) -> Result<usize, String> {
    get(m, k)?
        .parse::<usize>()
        .map_err(|_| format!("noninteger {k}"))
}
fn digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|x| x.is_ascii_hexdigit() && !x.is_ascii_uppercase())
}

/// Verify a complete receipt bundle without requiring filesystem access.
/// This checks integrity of the recorded results, not correctness of physics
/// (the independent 244-state replay covers the latter).
pub fn verify_bundle(hbp: &[u8], hbi: &[u8], anchor: &str) -> Result<usize, String> {
    if hbp.is_empty()
        || hbi.is_empty()
        || anchor.is_empty()
        || !hbp.ends_with(b"\n")
        || !hbi.ends_with(b"\n")
        || !anchor.ends_with('\n')
    {
        return Err("missing or unterminated evidence".into());
    }
    let a_line = anchor.strip_suffix('\n').ok_or("anchor newline")?;
    if a_line.contains('\n') || a_line.contains('\r') {
        return Err("multiple anchor lines".into());
    }
    let a = fields(a_line, "ASOLARIA_HOST8_TRUST_ANCHOR|")?;
    if get(&a, "json")? != "0" || get(&a, "version")? != "2" {
        return Err("unsupported anchor schema".into());
    }
    let ticks = number(&a, "total_ticks")?;
    if ticks == 0 || ticks > 1_000_000 || number(&a, "total_rooms")? != ticks * V {
        return Err("anchor transition totals invalid".into());
    }
    if get(&a, "root_hbp_sha256")? != hash_bytes(hbp)
        || get(&a, "root_hbi_sha256")? != hash_bytes(hbi)
    {
        return Err("root SHA mismatch".into());
    }
    let root_chain = get(&a, "chain_hash")?;
    if !digest(root_chain) {
        return Err("invalid chain digest".into());
    }
    let payload = format!(
        "{}|{}|{}|{}|{}",
        ticks,
        ticks * V,
        root_chain,
        get(&a, "root_hbp_sha256")?,
        get(&a, "root_hbi_sha256")?
    );
    if get(&a, "anchor_sig")? != hash_bytes(payload.as_bytes()) {
        return Err("anchor checksum mismatch".into());
    }

    let hbp_rows: Vec<&[u8]> = hbp.split_inclusive(|b| *b == b'\n').collect();
    let hbi_rows: Vec<&[u8]> = hbi.split_inclusive(|b| *b == b'\n').collect();
    if hbp_rows.len() != ticks + 1 || hbi_rows.len() != ticks + 1 {
        return Err("record count differs from anchor".into());
    }
    let header_bp = std::str::from_utf8(hbp_rows[0]).map_err(|_| "bad HBP header utf8")?;
    let header_bi = std::str::from_utf8(hbi_rows[0]).map_err(|_| "bad HBI header utf8")?;
    if !header_bp.starts_with("HOST8HDR|")
        || !header_bi.starts_with("HBIHDR|")
        || !header_bp.ends_with("|json=0\n")
        || !header_bi.ends_with("|json=0\n")
    {
        return Err("invalid index/receipt headers".into());
    }

    let mut prev = GENESIS_CHAIN_HASH.to_string();
    let mut offset = hbp_rows[0].len();
    let mut first_source: Option<&str> = None;
    for (pos, (raw, raw_index)) in hbp_rows[1..].iter().zip(&hbi_rows[1..]).enumerate() {
        if !raw.ends_with(b"\n") || !raw_index.ends_with(b"\n") {
            return Err("unterminated row".into());
        }
        let line = std::str::from_utf8(raw)
            .map_err(|_| "HBP invalid utf8")?
            .strip_suffix('\n')
            .ok_or("HBP missing newline")?;
        let index = std::str::from_utf8(raw_index)
            .map_err(|_| "HBI invalid utf8")?
            .strip_suffix('\n')
            .ok_or("HBI missing newline")?;
        if line.contains('\r') || index.contains('\r') {
            return Err("noncanonical CRLF".into());
        }
        let bp = fields(line, "HOST8WIN|")?;
        let bi = fields(index, "HBI|")?;
        let tick = pos + 1;
        if number(&bp, "tick")? != tick
            || number(&bi, "tick")? != tick
            || number(&bi, "offset")? != offset
            || number(&bi, "len")? != raw.len()
        {
            return Err(format!("HBI offset or tick mismatch at {tick}"));
        }
        if number(&bp, "rooms")? != V
            || number(&bp, "room_start")? != 0
            || number(&bp, "portal_probes")? != tick * PROBES_PER_STEP
            || get(&bp, "virtual_portals")? != VIRTUAL_PORTALS.to_string()
        {
            return Err(format!("invalid receipt geometry at tick {tick}"));
        }
        if get(&bp, "dE")? != "0"
            || get(&bp, "rev_mismatches")? != "0"
            || get(&bp, "physics_status")? != "VERIFIED"
            || get(&bp, "json")? != "0"
            || get(&bi, "json")? != "0"
            || get(&bi, "status")? != "VERIFIED"
        {
            return Err(format!("receipt status invalid at tick {tick}"));
        }

        let src = get(&bp, "source_sha256")?;
        let sh = get(&bp, "sh")?;
        let state = get(&bp, "state_sha256")?;
        if !digest(src)
            || !digest(state)
            || sh.len() != 16
            || sh != &src[..16]
            || get(&bi, "source_sha")? != src
            || get(&bi, "sh")? != sh
        {
            return Err(format!("source/SH invalid at tick {tick}"));
        }
        if let Some(source) = first_source {
            if source != src {
                return Err("source changed within run".into());
            }
        } else {
            first_source = Some(src);
        }

        let (original_payload, _) = line
            .split_once("|receipt_sha=")
            .ok_or("missing receipt digest delimiter")?;
        let calc_receipt = hash_bytes(original_payload.as_bytes());
        if get(&bp, "receipt_sha")? != calc_receipt {
            return Err(format!("receipt SHA invalid at tick {tick}"));
        }
        if get(&bp, "prev_hash")? != prev || get(&bi, "hbp_sha256")? != hash_bytes(raw) {
            return Err(format!(
                "HBP/HBI digest or chain predecessor invalid at tick {tick}"
            ));
        }
        let next_payload = format!(
            "{}|{}|{}|{}|{}|{}|VERIFIED|{}",
            prev, tick, 0, src, sh, state, calc_receipt
        );
        let next = hash_bytes(next_payload.as_bytes());
        if get(&bp, "hash")? != next || get(&bi, "hash")? != next {
            return Err(format!("HBI/HBP chain mismatch at tick {tick}"));
        }
        prev = next;
        offset += raw.len();
    }
    if offset != hbp.len() || get(&a, "chain_hash")? != prev {
        return Err("anchor last chain/offset mismatch".into());
    }
    Ok(ticks)
}

pub fn verify_files(anchor: &Path, hbp: &Path, hbi: &Path) -> Result<usize, String> {
    let a = std::fs::read_to_string(anchor).map_err(|e| format!("anchor: {e}"))?;
    let b = std::fs::read(hbp).map_err(|e| format!("HBP: {e}"))?;
    let i = std::fs::read(hbi).map_err(|e| format!("HBI: {e}"))?;
    verify_bundle(&b, &i, &a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::receipts::{write_trust_anchor, ReceiptWriter};
    use std::fs;

    #[test]
    fn published_synthetic_receipts_have_full_chain_integrity() {
        let hbp = include_bytes!("../evidence/heb_c_rust_243.hbp");
        let hbi = include_bytes!("../evidence/heb_c_rust_243.hbi");
        let anchor = include_str!("../evidence/heb_c_rust_243.anchor");
        assert_eq!(verify_bundle(hbp, hbi, anchor), Ok(243));
    }

    #[test]
    fn tampering_fails_even_when_root_digests_are_rewritten() {
        let hbp = include_bytes!("../evidence/heb_c_rust_243.hbp");
        let hbi = include_bytes!("../evidence/heb_c_rust_243.hbi");
        let anchor = include_str!("../evidence/heb_c_rust_243.anchor");
        assert!(verify_bundle(hbp, hbi, anchor).is_ok());
        let mut changed = hbp.to_vec();
        let start = changed
            .windows(3)
            .position(|b| b == b"dE=")
            .expect("receipt field");
        changed[start + 3] = b'9';
        assert!(verify_bundle(&changed, hbi, anchor).is_err());

        let mut rewritten = anchor.to_owned();
        let old = hash_bytes(hbp);
        let new_hash = hash_bytes(&changed);
        rewritten = rewritten.replace(&old, &new_hash);
        // Even after adjusting the top-level root, the chained data must fail.
        assert!(verify_bundle(&changed, hbi, &rewritten).is_err());
    }

    #[test]
    fn writer_roundtrip_passes_the_strict_verifier() {
        let base = std::env::temp_dir().join(format!("hebc-strict-{}", std::process::id()));
        fs::create_dir_all(&base).expect("create temp dir");
        let hbp = base.join("test.hbp");
        let hbi = base.join("test.hbi");
        let anchor = base.join("test.anchor");
        let mut w = ReceiptWriter::create(hbp.to_str().unwrap(), hbi.to_str().unwrap(), "test.hbp")
            .unwrap();
        let source = "b".repeat(64);
        let state = "c".repeat(64);
        let sh = &source[..16];
        let chain = w
            .record_transition(
                1,
                0,
                V,
                &source,
                sh,
                0,
                0,
                &state,
                VIRTUAL_PORTALS,
                PROBES_PER_STEP,
            )
            .unwrap();
        w.flush().unwrap();
        write_trust_anchor(
            anchor.to_str().unwrap(),
            hbp.to_str().unwrap(),
            hbi.to_str().unwrap(),
            1,
            V,
            &chain,
        )
        .unwrap();
        assert_eq!(verify_files(&anchor, &hbp, &hbi), Ok(1));
        let _ = fs::remove_dir_all(&base);
    }
}
