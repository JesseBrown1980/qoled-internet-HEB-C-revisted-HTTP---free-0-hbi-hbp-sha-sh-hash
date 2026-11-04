//! Fail-closed three-certificate Yang-Mills evidence index on HTTP-0's
//! already verified 3^16 virtual address namespace. No new eigensolve.
//! The fifth trit (3^4) marks YM domain=1; original Clay leaves require
//! all ten middle trits=0, so these are provably disjoint.
#![forbid(unsafe_code)]

use crate::sha256::hash_bytes;
use std::path::Path;
use std::sync::OnceLock;

pub const CAPACITY: u64 = crate::bridge43::CAPACITY;
pub const DOMAIN_OFFSET: u64 = 81; // nonzero digit in fifth ternary position
pub const OCCUPIED: usize = 3;
pub const COMBINED_OCCUPIED: usize = crate::bridge43::POPULATED + OCCUPIED;
pub const SOURCE_COMMIT: &str = "5517268b511d5f28e307b21fc53eede3200ed7b9";
pub const REPOSITORY: &str =
    "JesseBrown1980/Finite-universe-Yang-mills-existence-and-mass-gap-using-HEB-C";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Source {
    pub kind: &'static str,
    pub path: &'static str,
    pub sha256: &'static str,
    pub git_blob_sha1: &'static str,
    pub bytes: usize,
}

pub const SOURCES: [Source; OCCUPIED] = [
    Source {
        kind: "PERRON",
        path: "native/ym/results/3x4_stream/perron/PERRON-RESULT.hbp",
        sha256: "de19b354127c59ba3c2328fd310b5ff6a1103829c8d1d86c58b433aeec115a18",
        git_blob_sha1: "cbd428014a4d440732a5c0a541ed57a9041becee",
        bytes: 44_107,
    },
    Source {
        kind: "FULLREBUILD",
        path: "native/ym/results/3x4_stream/perron/FULLREBUILD-RUN.hbp",
        sha256: "cccada48ce34abeb12d77c8c4a0917dd4d9acb489aac3a9fe5d77b29f47a0a07",
        git_blob_sha1: "d6e264cdf3a30929edc552bf85077f4aa08b799c",
        bytes: 1_230,
    },
    Source {
        kind: "ALLPAIRS",
        path: "native/ym/results/3x4_stream/perron/allpairs/ALLPAIRS-RUN.hbp",
        sha256: "8af3e4e76672abc92bf0d17113680cda387a353de082916a5e2921756ca95ce0",
        git_blob_sha1: "6cf863c8ac0d4943fb5077c77fa4fc02b3e0acc2",
        bytes: 3_815,
    },
];

#[derive(Clone)]
pub struct VerifiedEvidence {
    pub source: Source,
    pub address: u64,
    pub statement: String,
}

pub struct Store {
    pub records: Vec<VerifiedEvidence>,
}

#[must_use]
pub const fn encode(kind_index: u8) -> Option<u64> {
    if kind_index >= OCCUPIED as u8 {
        None
    } else {
        Some(DOMAIN_OFFSET + kind_index as u64)
    }
}

#[must_use]
pub const fn decode(id: u64) -> Option<u8> {
    if id >= DOMAIN_OFFSET && id < DOMAIN_OFFSET + OCCUPIED as u64 {
        Some((id - DOMAIN_OFFSET) as u8)
    } else {
        None
    }
}

/// An unambiguous key/value in an already selected and committed HBP record.
fn field<'a>(line: &'a str, label: &str) -> Result<&'a str, String> {
    let prefix = format!("{label}=");
    let mut match_value = None;
    for part in line.split('|') {
        if let Some(value) = part.strip_prefix(&prefix) {
            if match_value.replace(value).is_some() {
                return Err(format!("duplicate key {label}"));
            }
        }
    }
    match_value.ok_or_else(|| format!("missing field {label}"))
}

fn row<'a>(lines: &'a str, kind: &str, run: Option<&str>) -> Result<&'a str, String> {
    let mut matching = lines.lines().filter(|line| {
        line.starts_with(kind)
            && line.as_bytes().get(kind.len()) == Some(&b'|')
            && match run {
                Some(expected) => field(line, "run").ok() == Some(expected),
                None => true,
            }
    });
    let result = matching
        .next()
        .ok_or_else(|| format!("missing {kind} row"))?;
    if matching.next().is_some() {
        return Err(format!("ambiguous {kind} row"));
    }
    if field(result, "json")? != "0" {
        return Err("unexpected JSON flag".into());
    }
    Ok(result)
}

fn expect(line: &str, key: &str, value: &str) -> Result<(), String> {
    if field(line, key)? != value {
        return Err(format!("recorded {key} differs from registered {value}"));
    }
    Ok(())
}

/// These statements are SOURCE RECORD STATUS only. This reader is *not* a
/// rerun of 45.5M Yang-Mills rows or 1.63B weighted pair comparisons.
fn select_statement(kind: &str, data: &str) -> Result<String, String> {
    match kind {
        "PERRON" => {
            let x = row(data, "PERRON_RESULT", None)?;
            expect(x, "best_t_cert_milli", "94070673")?;
            expect(x, "lambda1_upper_milli", "94070936")?;
            expect(x, "gap_verdict", "OPEN")?;
            expect(
                x,
                "all_rows_rebuild_second_seat",
                "MATCHED_45508080_of_45508080",
            )?;
            Ok("claim=t_cert_94070673_units_1e-6_JE|lambda1_upper_94070936_units_1e-6_JE|source_gap_verdict=OPEN|verification=SOURCE_PIN_AND_RECORDED_FIELDS".to_string())
        }
        "FULLREBUILD" => {
            let x = row(data, "PREDICTION_RESULT", Some("fullrebuild_3x4"))?;
            for (k, v) in [
                ("rows_rebuilt", "45508080"),
                ("rows_read", "45508080"),
                ("canonical_key_mismatch", "0"),
                ("weight_mismatch", "0"),
                ("M_mismatch", "0"),
                ("entries_mismatch", "0"),
                ("merkle", "PASS"),
                ("prediction_met", "1"),
                ("exit", "0"),
                ("sectors_match_manifest", "34/34"),
            ] {
                expect(x, k, v)?;
            }
            Ok("claim=45508080_rows_rebuilt|source_merkle=PASS|source_mismatch=0|verification=SOURCE_PIN_AND_RECORDED_FIELDS".to_string())
        }
        "ALLPAIRS" => {
            let x = row(data, "PREDICTION_RESULT", Some("allpairs_3x4"))?;
            for (k, v) in [
                ("n", "45508080"),
                ("rows_read", "45508080"),
                ("pairs", "1632625631"),
                ("passes", "4"),
                ("value_mismatch", "0"),
                ("lower_without_upper", "0"),
                ("upper_without_lower", "0"),
                ("row_sum_nonzero", "0"),
                ("merkle_each_pass", "PASS"),
                ("verdict", "PASS"),
                ("prediction_met", "1"),
                ("exit", "0"),
            ] {
                expect(x, k, v)?;
            }
            Ok("claim=1632625631_weighted_pairs_reported|source_symmetry=PASS|verification=SOURCE_PIN_AND_RECORDED_FIELDS".to_string())
        }
        _ => Err("unknown Yang-Mills certificate class".into()),
    }
}

/// Accept exact source bytes from a pinned checkout, not remotely fetched
/// content and not a substitute generated when files are missing.
pub fn load(repo_root: &Path) -> Result<Store, String> {
    let mut records = Vec::with_capacity(OCCUPIED);
    for (i, source) in SOURCES.iter().copied().enumerate() {
        let input = std::fs::read(repo_root.join(source.path))
            .map_err(|e| format!("missing {}: {e}", source.path))?;
        if input.len() != source.bytes || hash_bytes(&input) != source.sha256 {
            return Err(format!(
                "source SHA256 or length mismatch at {}",
                source.path
            ));
        }
        let text =
            std::str::from_utf8(&input).map_err(|_| format!("invalid UTF8 for {}", source.path))?;
        let statement = select_statement(source.kind, text)?;
        let address = encode(i as u8).ok_or("invalid YM evidence slot")?;
        if crate::bridge43::decode(address).is_some() {
            return Err("Yang-Mills domain collides with original Clay address".into());
        }
        records.push(VerifiedEvidence {
            source,
            address,
            statement,
        });
    }
    Ok(Store { records })
}

impl Store {
    #[must_use]
    pub fn lookup(&self, address: u64) -> Option<String> {
        let index = usize::from(decode(address)?);
        let result = self.records.get(index)?;
        if result.address != address {
            return None;
        }
        Some(format!(
            "HTTP0_YM43_SLOT|id={address}|domain=YANG_MILLS_3X4|class={}|source_commit={SOURCE_COMMIT}|source_path={}|source_sha256={}|{}|scope=RECEIPT_INDEX_NOT_SPECTRAL_RECALCULATION|json=0\n",
            result.source.kind,
            result.source.path,
            result.source.sha256,
            result.statement
        ))
    }
}

#[must_use]
pub fn contract() -> String {
    format!(
        "HTTP0_YM43|schema=TERNARY-16-DOMAIN1-V1|capacity={CAPACITY}|power=3^16|domain_fifth_trit=1|occupied={OCCUPIED}|clay_occupied={}|global_occupied={COMBINED_OCCUPIED}|source_commit={SOURCE_COMMIT}|result_status=SOURCE_RECORDED|gap_verdict=OPEN|no_new_ym_computation=1|json=0\n",
        crate::bridge43::POPULATED
    )
}

static STORE: OnceLock<Result<Store, String>> = OnceLock::new();
pub fn runtime_store() -> &'static Result<Store, String> {
    STORE.get_or_init(|| {
        let root = std::env::var("HEB_C_YM43_ROOT")
            .map_err(|_| "HEB_C_YM43_ROOT not set; no source receipts installed".to_string())?;
        load(Path::new(&root))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn disjoint_three_certificates_in_power_of_three_namespace() {
        assert_eq!(CAPACITY, 3u64.pow(16));
        assert_eq!(COMBINED_OCCUPIED, 732);
        let mut seen = BTreeSet::new();
        for index in 0..OCCUPIED as u8 {
            let address = encode(index).expect("registered evidence");
            assert_eq!(address, DOMAIN_OFFSET + u64::from(index));
            assert!(seen.insert(address));
            assert_eq!(decode(address), Some(index));
            assert!(crate::bridge43::decode(address).is_none());
            let digits = crate::bridge43::to_trits16(address).unwrap();
            assert_eq!(digits[4], 1);
            assert_eq!(digits[15], 0);
            assert_eq!(crate::bridge43::from_trits16(&digits), Some(address));
        }
        assert_eq!(seen.len(), OCCUPIED);
    }
    #[test]
    fn invalid_and_unpopulated_addresses_are_not_evidence() {
        assert_eq!(decode(80), None);
        assert_eq!(decode(84), None);
        assert_eq!(decode(CAPACITY), None);
        assert_eq!(encode(3), None);
        assert!(load(Path::new("/__missing_ym43_source__")).is_err());
        assert!(select_statement("PERRON", "PERRON_RESULT|gap_verdict=PASS|json=0\n").is_err());
    }
}
