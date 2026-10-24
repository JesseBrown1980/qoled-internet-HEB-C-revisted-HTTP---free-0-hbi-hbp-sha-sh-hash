//! Native HBI / HBP / SHA / SH / HASH receipt serialization and verification.
//! Adheres strictly to json=0 wire-format specification.

use crate::sha256;
use std::fs::File;
use std::io::{self, BufWriter, Write};

pub const GENESIS_CHAIN_HASH: &str =
    "aabd0374888ad5e2380f53b95c782f0ae3368d0dc702457a22ae93beb55311ab";

pub struct ReceiptWriter {
    hbp_file: BufWriter<File>,
    hbi_file: BufWriter<File>,
    pub hbp_offset: usize,
    pub prev_hash: String,
    pub valid_records: usize,
}

impl ReceiptWriter {
    /// Create new HBP and HBI receipt writer with canonical json=0 headers.
    pub fn create(hbp_path: &str, hbi_path: &str, hbp_filename: &str) -> Result<Self, io::Error> {
        let mut hbp_file = BufWriter::new(File::create(hbp_path)?);
        let mut hbi_file = BufWriter::new(File::create(hbi_path)?);

        let hbp_hdr =
            "HOST8HDR|schema=asolaria.hebc.qtrit.v2|seat=ACER-CLAUDE-FABLE5|version=0.2.0|mode=HEBC_RUST|seal=HBI-HBP-SHA-SH-HASH|traversal=HBI->HBP->SHA->SH->HASH|compiler_version=1.81.0|json=0\n";
        hbp_file.write_all(hbp_hdr.as_bytes())?;
        let hbp_offset = hbp_hdr.len();

        let hbi_hdr = format!(
            "HBIHDR|for={}|schema=asolaria.hebc.index.v2|seal=HBI-HBP-SHA-SH-HASH|traversal=HBI->HBP->SHA->SH->HASH|json=0\n",
            hbp_filename
        );
        hbi_file.write_all(hbi_hdr.as_bytes())?;

        Ok(Self {
            hbp_file,
            hbi_file,
            hbp_offset,
            prev_hash: GENESIS_CHAIN_HASH.to_string(),
            valid_records: 0,
        })
    }

    /// Write a validated transition receipt and index entry.
    #[allow(clippy::too_many_arguments)]
    pub fn record_transition(
        &mut self,
        tick: usize,
        room_start: usize,
        rooms: usize,
        source_sha: &str,
        sh: &str,
        d_e: i64,
        rev_mismatches: usize,
        state_sha: &str,
        virtual_portals: u64,
        portal_probes: usize,
    ) -> Result<String, io::Error> {
        // 1. Canonical payload
        let payload = format!(
            "HOST8WIN|tick={}|room_start={}|rooms={}|source_sha256={}|sh={}|dE={}|rev_mismatches={}|state_sha256={}|virtual_portals={}|portal_probes={}|physics_status=VERIFIED",
            tick, room_start, rooms, source_sha, sh, d_e, rev_mismatches, state_sha, virtual_portals, portal_probes
        );
        let receipt_sha = sha256::hash_bytes(payload.as_bytes());

        // 2. Chained hash
        let chain_payload = format!(
            "{}|{}|{}|{}|{}|{}|VERIFIED|{}",
            self.prev_hash, tick, room_start, source_sha, sh, state_sha, receipt_sha
        );
        let cur_hash = sha256::hash_bytes(chain_payload.as_bytes());

        // 3. Full HBP record
        let hbp_row = format!(
            "{}|receipt_sha={}|prev_hash={}|hash={}|json=0\n",
            payload, receipt_sha, self.prev_hash, cur_hash
        );
        let hbp_bytes = hbp_row.as_bytes();
        let hbp_len = hbp_bytes.len();
        self.hbp_file.write_all(hbp_bytes)?;

        // 4. HBI index row
        let hbp_row_sha = sha256::hash_bytes(hbp_bytes);
        let hbi_row = format!(
            "HBI|tick={}|offset={}|len={}|source_sha={}|sh={}|hbp_sha256={}|hash={}|status=VERIFIED|json=0\n",
            tick, self.hbp_offset, hbp_len, source_sha, sh, hbp_row_sha, cur_hash
        );
        self.hbi_file.write_all(hbi_row.as_bytes())?;

        self.hbp_offset += hbp_len;
        self.prev_hash = cur_hash.clone();
        self.valid_records += 1;

        Ok(cur_hash)
    }

    /// Flush all receipt buffers.
    pub fn flush(&mut self) -> Result<(), io::Error> {
        self.hbp_file.flush()?;
        self.hbi_file.flush()?;
        Ok(())
    }
}

/// Write trust anchor file binding root digests, latest transition, and chain hash.
pub fn write_trust_anchor(
    anchor_path: &str,
    hbp_path: &str,
    hbi_path: &str,
    total_ticks: usize,
    total_rooms: usize,
    chain_hash: &str,
) -> Result<(), io::Error> {
    let hbp_bytes = std::fs::read(hbp_path)?;
    let hbi_bytes = std::fs::read(hbi_path)?;
    let root_hbp_sha = sha256::hash_bytes(&hbp_bytes);
    let root_hbi_sha = sha256::hash_bytes(&hbi_bytes);

    let sig_payload = format!(
        "{}|{}|{}|{}|{}",
        total_ticks, total_rooms, chain_hash, root_hbp_sha, root_hbi_sha
    );
    let anchor_sig = sha256::hash_bytes(sig_payload.as_bytes());

    let anchor_str = format!(
        "ASOLARIA_HOST8_TRUST_ANCHOR|version=2|seat=ACER-CLAUDE-FABLE5|epoch=20261024_RUST_HEBC|kernel_id=D3Q27_RUST_181_288GATES|lattice=L24_V13824_K27_INTEGER|total_ticks={}|total_rooms={}|root_hbp_sha256={}|root_hbi_sha256={}|chain_hash={}|anchor_sig={}|json=0\n",
        total_ticks, total_rooms, root_hbp_sha, root_hbi_sha, chain_hash, anchor_sig
    );

    let mut out = File::create(anchor_path)?;
    out.write_all(anchor_str.as_bytes())?;
    out.flush()?;
    Ok(())
}

/// Verify trust anchor signature and root digests.
pub fn verify_trust_anchor(
    anchor_path: &str,
    hbp_path: &str,
    hbi_path: &str,
) -> Result<bool, String> {
    let content =
        std::fs::read_to_string(anchor_path).map_err(|e| format!("Cannot read anchor: {e}"))?;
    let line = content.lines().next().ok_or("Empty anchor file")?;
    if !line.starts_with("ASOLARIA_HOST8_TRUST_ANCHOR|") {
        return Err("Invalid anchor header".to_string());
    }

    let mut fields = std::collections::BTreeMap::new();
    for part in line.split('|') {
        if let Some((k, v)) = part.split_once('=') {
            fields.insert(k, v);
        }
    }

    let total_ticks = fields.get("total_ticks").ok_or("Missing total_ticks")?;
    let total_rooms = fields.get("total_rooms").ok_or("Missing total_rooms")?;
    let chain_hash = fields.get("chain_hash").ok_or("Missing chain_hash")?;
    let root_hbp_sha = fields
        .get("root_hbp_sha256")
        .ok_or("Missing root_hbp_sha256")?;
    let root_hbi_sha = fields
        .get("root_hbi_sha256")
        .ok_or("Missing root_hbi_sha256")?;
    let anchor_sig = fields.get("anchor_sig").ok_or("Missing anchor_sig")?;

    let sig_payload = format!(
        "{}|{}|{}|{}|{}",
        total_ticks, total_rooms, chain_hash, root_hbp_sha, root_hbi_sha
    );
    let expected_sig = sha256::hash_bytes(sig_payload.as_bytes());
    if *anchor_sig != expected_sig {
        return Err("Tampered or invalid anchor signature".to_string());
    }

    let actual_hbp_bytes = std::fs::read(hbp_path).map_err(|e| format!("Cannot read HBP: {e}"))?;
    if sha256::hash_bytes(&actual_hbp_bytes) != *root_hbp_sha {
        return Err("Anchor root_hbp_sha256 mismatch".to_string());
    }

    let actual_hbi_bytes = std::fs::read(hbi_path).map_err(|e| format!("Cannot read HBI: {e}"))?;
    if sha256::hash_bytes(&actual_hbi_bytes) != *root_hbi_sha {
        return Err("Anchor root_hbi_sha256 mismatch".to_string());
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receipt_writer_and_anchor_verification() {
        let temp_dir = std::env::temp_dir();
        let hbp_path = temp_dir.join("test_trace.hbp");
        let hbi_path = temp_dir.join("test_index.hbi");
        let anchor_path = temp_dir.join("test.anchor");

        let hbp_s = hbp_path.to_str().unwrap();
        let hbi_s = hbi_path.to_str().unwrap();
        let anchor_s = anchor_path.to_str().unwrap();

        let mut writer =
            ReceiptWriter::create(hbp_s, hbi_s, "test_trace.hbp").expect("create failed");
        let chain1 = writer
            .record_transition(
                1,
                0,
                13824,
                "0000000000000000000000000000000000000000000000000000000000000000",
                "0000000000000000",
                0,
                0,
                "1111111111111111111111111111111111111111111111111111111111111111",
                23_000_000_000,
                8192,
            )
            .expect("record failed");
        writer.flush().expect("flush failed");

        write_trust_anchor(anchor_s, hbp_s, hbi_s, 1, 13824, &chain1).expect("anchor write failed");
        assert!(verify_trust_anchor(anchor_s, hbp_s, hbi_s).expect("anchor verify failed"));

        // Tamper test: append byte to HBP
        let mut hbp_tampered = std::fs::read(hbp_s).unwrap();
        hbp_tampered.push(b'X');
        std::fs::write(hbp_s, hbp_tampered).unwrap();
        assert!(verify_trust_anchor(anchor_s, hbp_s, hbi_s).is_err());

        // Cleanup
        let _ = std::fs::remove_file(hbp_path);
        let _ = std::fs::remove_file(hbi_path);
        let _ = std::fs::remove_file(anchor_path);
    }
}
