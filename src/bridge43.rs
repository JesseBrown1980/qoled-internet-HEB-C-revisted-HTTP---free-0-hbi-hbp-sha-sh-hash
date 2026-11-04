//! Sparse 3^16 ternary address namespace for the three verified Taylor-Green
//! -> Clay/Clasy 81-register projection leaves. A virtual address is NOT an
//! independent state variable. No new NTT, 43M-element array or PDE proof.
//! Source: Clay three-step GitHub Actions receipts from 2026-11-04.
#![forbid(unsafe_code)]

use crate::sha256::hash_bytes;
use std::path::Path;
use std::sync::OnceLock;

pub const CAPACITY: u64 = 43_046_721; // 3^16
pub const TIME_STRIDE: u64 = 14_348_907; // 3^15
pub const ARM_STRIDE: u64 = 4_782_969; // 3^14
pub const REGISTERS: usize = 81; // 3^4, native Clay projection width
pub const POPULATED: usize = 3 * 3 * REGISTERS; // only 729 occupied positions
pub const P: i64 = 1_000_999;
pub const SOURCE_CLAY_COMMIT: &str = "36b304006adfecff13df6ed1098c9ed30433bc3b";
pub const LEAF_HASHES: [&str; 3] = [
    "ee5036f22e94990de1992e4649302d603aa5a2891222b9333a512959c95b40be",
    "67c1ceb79d45761b68835910e0a05c76564785b77b487bff3fe6d5a49901b11e",
    "c638926523a7b45e44e1396320a4f3a640c17633716e65824351a8a543819c9f",
];
pub const INDEX_HASH: &str = "7c4b65c4c36e57d5447b5ece0312d1e2e440d2b09cae192ccbafa38ae0501f51";
pub const ARM_NAMES: [&str; 3] = ["TGV", "SCR", "NULL"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Address {
    pub step: u8,     // 1, 2, 3
    pub arm: u8,      // 0=TGV, 1=SCR, 2=NULL
    pub register: u8, // 0..80
}

/// Embed the 9 model/time pages in a 16-trit address.
/// Top trit=step-1, next trit=arm, bottom four=register.
/// Middle ten trits must be zero. Every valid occupied slot is unique.
#[must_use]
pub const fn encode(slot: Address) -> Option<u64> {
    if slot.step < 1 || slot.step > 3 || slot.arm >= 3 || slot.register >= 81 {
        return None;
    }
    Some(
        (slot.step as u64 - 1) * TIME_STRIDE
            + (slot.arm as u64) * ARM_STRIDE
            + slot.register as u64,
    )
}

/// Return None for unpopulated or out-of-range virtual addresses.
/// No fabricated value is returned for the other 43,045,992 addresses.
#[must_use]
pub const fn decode(id: u64) -> Option<Address> {
    if id >= CAPACITY {
        return None;
    }
    let step = id / TIME_STRIDE;
    let arm = (id % TIME_STRIDE) / ARM_STRIDE;
    let register = id % ARM_STRIDE;
    if step > 2 || arm > 2 || register >= REGISTERS as u64 {
        return None;
    }
    Some(Address {
        step: step as u8 + 1,
        arm: arm as u8,
        register: register as u8,
    })
}

#[must_use]
pub fn to_trits16(mut id: u64) -> Option<[u8; 16]> {
    if id >= CAPACITY {
        return None;
    }
    let mut result = [0u8; 16];
    for trit in &mut result {
        *trit = (id % 3) as u8;
        id /= 3;
    }
    Some(result)
}

#[must_use]
pub fn from_trits16(trits: &[u8; 16]) -> Option<u64> {
    let mut id = 0u64;
    let mut power = 1u64;
    for &digit in trits {
        if digit >= 3 {
            return None;
        }
        id += u64::from(digit) * power;
        power *= 3;
    }
    if id < CAPACITY {
        Some(id)
    } else {
        None
    }
}

#[derive(Clone, Debug)]
pub struct RegisterRow {
    pub step: u8,
    pub arm: u8,
    pub signed: [i64; REGISTERS],
    pub field: [u64; REGISTERS],
}

pub struct ReceiptStore {
    pub rows: Vec<RegisterRow>,
}

fn field<'a>(line: &'a str, label: &str) -> Result<&'a str, String> {
    let needle = format!("{label}=");
    let mut value = None;
    for chunk in line.split('|') {
        if let Some(v) = chunk.strip_prefix(&needle) {
            if value.replace(v).is_some() {
                return Err(format!("duplicate field {label}"));
            }
        }
    }
    value.ok_or_else(|| format!("missing field {label}"))
}

fn parse_csv_i64<const N: usize>(src: &str) -> Result<[i64; N], String> {
    let v: Vec<i64> = src
        .split(',')
        .map(|x| {
            x.parse::<i64>()
                .map_err(|_| "bad signed integer".to_string())
        })
        .collect::<Result<_, _>>()?;
    v.try_into()
        .map_err(|_| format!("expected {N} signed values"))
}
fn parse_csv_u64<const N: usize>(src: &str) -> Result<[u64; N], String> {
    let v: Vec<u64> = src
        .split(',')
        .map(|x| x.parse::<u64>().map_err(|_| "bad residue".to_string()))
        .collect::<Result<_, _>>()?;
    v.try_into().map_err(|_| format!("expected {N} residues"))
}

impl ReceiptStore {
    pub fn get(&self, slot: Address) -> Option<(i64, u64)> {
        let row = self
            .rows
            .iter()
            .find(|r| r.step == slot.step && r.arm == slot.arm)?;
        let i = usize::from(slot.register);
        Some((row.signed[i], row.field[i]))
    }
}

/// Read only three complete, published, byte-pinned leaf receipts.
/// No network access, data generation, or substitute fixtures on failure.
pub fn load(dir: &Path) -> Result<ReceiptStore, String> {
    let index =
        std::fs::read(dir.join("LEAVES.hbi")).map_err(|e| format!("missing LEAVES.hbi: {e}"))?;
    if hash_bytes(&index) != INDEX_HASH {
        return Err("HBI index SHA256 mismatch".into());
    }
    let index = String::from_utf8(index).map_err(|_| "invalid index UTF8")?;
    let index_lines: Vec<&str> = index.lines().collect();
    if index_lines.len() != 4 || !index_lines[0].starts_with("BRIDGE_HBI|") {
        return Err("unexpected index row count".into());
    }
    let mut rows = Vec::with_capacity(9);
    for step in 1..=3usize {
        let name = format!("LEAF-{step}.hbp");
        let leaf = std::fs::read(dir.join(&name)).map_err(|e| format!("missing {name}: {e}"))?;
        if hash_bytes(&leaf) != LEAF_HASHES[step - 1] {
            return Err(format!("{name} SHA256 mismatch"));
        }
        let ix = index_lines[step];
        let parent = if step == 1 {
            "GENESIS"
        } else {
            LEAF_HASHES[step - 2]
        };
        if field(ix, "file")? != name
            || field(ix, "sha256")? != LEAF_HASHES[step - 1]
            || field(ix, "previous_leaf_sha256")? != parent
            || field(ix, "rows")? != "3"
            || field(ix, "t")? != step.to_string()
        {
            return Err(format!("HBI row {step} inconsistent"));
        }
        let leaf_text =
            String::from_utf8(leaf).map_err(|_| format!("invalid leaf UTF8 at {step}"))?;
        let lines: Vec<&str> = leaf_text.lines().collect();
        if lines.len() != 5
            || !lines[0].starts_with("LEAF_HEADER|")
            || !lines[4].starts_with("LEAF_FOOTER|")
            || field(lines[0], "t")? != step.to_string()
            || field(lines[0], "previous_leaf_sha256")? != parent
            || field(lines[4], "t")? != step.to_string()
            || field(lines[4], "status")? != "PASS"
        {
            return Err(format!("invalid leaf structure at {step}"));
        }
        for (arm, &label) in ARM_NAMES.iter().enumerate() {
            let record = lines[arm + 1];
            if !record.starts_with("LEAF|")
                || field(record, "t")? != step.to_string()
                || field(record, "arm")? != label
            {
                return Err(format!("unexpected arm at {step}/{arm}"));
            }
            let signed = parse_csv_i64::<REGISTERS>(field(record, "signed81")?)?;
            let residues = parse_csv_u64::<REGISTERS>(field(record, "u81")?)?;
            if signed
                .iter()
                .zip(&residues)
                .any(|(&j, &u)| j.abs() > 9_216 || u >= P as u64 || u != j.rem_euclid(P) as u64)
            {
                return Err(format!("residue mapping mismatch {step}/{arm}"));
            }
            let global = parse_csv_i64::<3>(field(record, "global")?)?;
            let q: u64 = field(record, "Q")?.parse().map_err(|_| "bad Q field")?;
            if q != signed.iter().sum::<i64>().rem_euclid(P) as u64
                || q != global.iter().sum::<i64>().rem_euclid(P) as u64
                || field(record, "mass")? != "373248"
                || field(record, "E2")? != "746496"
            {
                return Err(format!("total/energy mismatch {step}/{arm}"));
            }
            if label == "NULL"
                && (signed.iter().any(|&j| j != 0)
                    || field(record, "moved_trits")? != "0"
                    || field(record, "local_defect")? != "0")
            {
                return Err(format!("NULL moved at {step}"));
            }
            rows.push(RegisterRow {
                step: step as u8,
                arm: arm as u8,
                signed,
                field: residues,
            });
        }
    }
    if rows.len() * REGISTERS != POPULATED {
        return Err("occupied address census mismatch".into());
    }
    Ok(ReceiptStore { rows })
}

/// Runtime receipts are optional, but missing evidence never becomes synthetic data.
/// Read at most once per server process; set HEB_C_CLAY_LEAVES_DIR before startup.
static STORE: OnceLock<Result<ReceiptStore, String>> = OnceLock::new();
pub fn runtime_store() -> &'static Result<ReceiptStore, String> {
    STORE.get_or_init(|| {
        let dir = std::env::var("HEB_C_CLAY_LEAVES_DIR")
            .map_err(|_| "HEB_C_CLAY_LEAVES_DIR not set; no receipts loaded".to_string())?;
        load(Path::new(&dir))
    })
}

#[must_use]
pub fn contract() -> String {
    format!(
        "HTTP0_BRIDGE43|schema=TERNARY-16-SPARSE-V1|capacity={CAPACITY}|power=3^16|page_count=531441|registers_per_page=81|occupied={POPULATED}|steps=3|arms=3|source_clay_commit={SOURCE_CLAY_COMMIT}|identity=SHA256_PINNED_LEAVES|scope=VIRTUAL_ADDRESSES_NOT_INDEPENDENT_FLUID_STATES|json=0\n"
    )
}

#[must_use]
pub fn lookup(id: u64, data: &ReceiptStore) -> Option<String> {
    let slot = decode(id)?;
    let (signed, residue) = data.get(slot)?;
    let trits = to_trits16(id)?;
    let trits_high_to_low: String = trits.iter().rev().map(|d| char::from(b'0' + *d)).collect();
    Some(format!(
        "HTTP0_BRIDGE43_SLOT|id={id}|trits16={trits_high_to_low}|t={}|arm={}|register={}|signed={signed}|residue={residue}|source_leaf_sha256={}|json=0\n",
        slot.step, ARM_NAMES[usize::from(slot.arm)], slot.register, LEAF_HASHES[usize::from(slot.step - 1)]
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn capacity_and_page_math() {
        assert_eq!(3u64.pow(16), CAPACITY);
        assert_eq!(3u64.pow(15), TIME_STRIDE);
        assert_eq!(3u64.pow(14), ARM_STRIDE);
        assert_eq!(CAPACITY / REGISTERS as u64, 531_441);
        assert_eq!(POPULATED, 729);
        assert_eq!(CAPACITY - POPULATED as u64, 43_045_992);
        assert!(CAPACITY >= 23_000_000);
    }
    #[test]
    fn all_729_addresses_unique_and_roundtrip_in_16_trits() {
        let mut unique = BTreeSet::new();
        for step in 1..=3 {
            for arm in 0..3 {
                for register in 0..81 {
                    let slot = Address {
                        step,
                        arm,
                        register,
                    };
                    let id = encode(slot).expect("valid slot");
                    assert!(unique.insert(id));
                    assert_eq!(decode(id), Some(slot));
                    let digits = to_trits16(id).expect("valid ID");
                    assert_eq!(from_trits16(&digits), Some(id));
                    assert_eq!(digits[15], step - 1);
                    assert_eq!(digits[14], arm);
                    assert!(digits[4..14].iter().all(|&v| v == 0));
                }
            }
        }
        assert_eq!(unique.len(), POPULATED);
    }
    #[test]
    fn refuse_absent_or_out_of_range_slots() {
        assert!(decode(CAPACITY).is_none());
        assert!(decode(CAPACITY - 1).is_none());
        assert!(decode(81).is_none());
        assert!(encode(Address {
            step: 0,
            arm: 0,
            register: 0
        })
        .is_none());
        assert!(encode(Address {
            step: 1,
            arm: 3,
            register: 0
        })
        .is_none());
        assert!(encode(Address {
            step: 1,
            arm: 0,
            register: 81
        })
        .is_none());
        assert_eq!(from_trits16(&[3; 16]), None);
        let max = to_trits16(CAPACITY - 1).unwrap();
        assert_eq!(from_trits16(&max), Some(CAPACITY - 1));
        assert!(load(Path::new("/__hebc_clay_receipt_intentionally_missing__")).is_err());
    }
}
