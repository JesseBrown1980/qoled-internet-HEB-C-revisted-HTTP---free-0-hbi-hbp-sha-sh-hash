//! Exact Asolaria Law 0–71 and Book IX source-address reader.
//! Original bytes embedded at compile time. No model or remote fetch needed.
use crate::sha256::hash_bytes;
const BOOK: &str = include_str!("../canon/COMBINED-BOOK-OF-LAWS.original.md");
const LATER: &str = include_str!("../canon/RIME_LAWS_58_TO_71_SOURCE_SNAPSHOTS.md");

fn compiled(n: u16) -> Option<String> {
    let needle = format!("### Law {n} ");
    let mut found = false;
    let mut body = String::new();
    for line in BOOK.split_inclusive('\n') {
        if !found {
            if line.starts_with(&needle) {
                found = true;
                body.push_str(line);
            }
        } else if line.starts_with("### ") || line.starts_with("## ") || line.starts_with("# ") {
            break;
        } else {
            body.push_str(line);
        }
    }
    if found {
        Some(body)
    } else {
        None
    }
}
fn later(n: u16) -> Vec<String> {
    if !(58..=71).contains(&n) {
        return Vec::new();
    }
    let prefix = format!("books/LAW-{n}-");
    let mut active = false;
    let mut path = String::new();
    let mut body = String::new();
    let mut result = Vec::new();
    for line in LATER.split_inclusive('\n') {
        if let Some(p) = line.strip_prefix("## BEGIN IMMUTABLE SOURCE") {
            active = false;
            body.clear();
            path = p.trim().trim_start_matches('—').trim().to_owned();
        } else if line.starts_with("## END IMMUTABLE SOURCE") {
            if active {
                result.push(body.clone());
            }
            active = false;
            path.clear();
        } else if !path.is_empty() {
            if !active && line.starts_with("# ") && path.starts_with(&prefix) {
                active = true;
                body.push_str(line);
            } else if active {
                body.push_str(line);
            }
        }
    }
    result
}
pub fn law(n: u16, variant: Option<usize>) -> Option<String> {
    if n == 27 || n > 71 {
        return None;
    }
    let text = match variant {
        Some(i) => later(n).get(i).cloned(),
        None => compiled(n).or_else(|| later(n).into_iter().next()),
    }?;
    let sha = hash_bytes(text.as_bytes());
    Some(format!(
        "LAWADDR|number={n}|variant={}|bytes={}|sha256={sha}|original=1|json=0\n{text}",
        variant.map_or_else(|| "default".into(), |i| i.to_string()),
        text.len()
    ))
}
fn rules() -> Vec<&'static str> {
    let start = BOOK
        .find("## IX.H — THE FULL CATALOGUE")
        .unwrap_or(BOOK.len());
    let end = BOOK[start..]
        .find("\n## PROVENANCE")
        .map_or(BOOK.len(), |n| start + n);
    BOOK[start..end]
        .lines()
        .filter(|l| l.starts_with("- "))
        .collect()
}
pub fn rule(name: &str) -> Option<String> {
    if name.is_empty()
        || name.len() > 120
        || !name
            .bytes()
            .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit() || c == b'-')
    {
        return None;
    }
    let backtick = char::from(96);
    let needle = format!("- {backtick}{name}{backtick} ");
    rules()
        .into_iter()
        .find(|r| r.starts_with(&needle))
        .map(|r| {
            format!(
                "RULE|name={name}|sha256={}|source=BOOK_IX|json=0\n{r}\n",
                hash_bytes(r.as_bytes())
            )
        })
}
pub fn index() -> String {
    let sha = hash_bytes(BOOK.as_bytes());
    let mut out = format!(
        "LAWSET|book_sha256={sha}|book9_rules={}|law27=UNASSIGNED|json=0\n",
        rules().len()
    );
    for n in 0..=71 {
        let status = if law(n, None).is_some() {
            "PRESENT"
        } else {
            "UNASSIGNED"
        };
        out.push_str(&format!(
            "LAW|number={n}|status={status}|variants={}|json=0\n",
            later(n).len()
        ));
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complete_preserved_laws_and_variants() {
        assert_eq!(BOOK.as_bytes().len(), 710577);
        assert_eq!(rules().len(), 1940);
        assert!(law(0, None).unwrap().contains("Center is Free"));
        assert!(law(27, None).is_none());
        assert!(law(58, None).unwrap().contains("Sixth Day"));
        assert_eq!(later(58).len(), 2);
        assert_eq!(later(71).len(), 1);
        assert!(law(71, None).unwrap().contains("Law 71"));
        assert!(law(72, None).is_none());
    }
    #[test]
    fn all_original_book9_handles_resolve() {
        let items = rules();
        let backtick = char::from(96);
        let mut unique = std::collections::BTreeSet::new();
        for r in items {
            let name = r.trim_start_matches("- ").split(backtick).nth(1).unwrap();
            assert!(unique.insert(name));
            assert!(rule(name).is_some());
        }
        assert!(rule("../not-valid").is_none());
    }
}
