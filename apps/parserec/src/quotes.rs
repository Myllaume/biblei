use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Quote {
  pub key: String,
  pub pages: Vec<u32>,
}

/// Load bibliographic IDs from a CSL-JSON file.
pub fn load_bib_ids(file_path: &str) -> Result<HashSet<String>> {
  let content = fs::read_to_string(file_path)?;
  let items: Vec<serde_json::Value> = serde_json::from_str(&content)?;
  let ids = items
    .into_iter()
    .filter_map(|v| v["id"].as_str().map(|s| s.to_string()))
    .collect();
  Ok(ids)
}

/// Parse page numbers from a page specification string.
/// Supports: `p.141`, `p.141-143`, `p.141, p.145`.
fn parse_pages(spec: &str) -> Vec<u32> {
  let mut pages = Vec::new();
  let re = Regex::new(r"(\d+)(?:-(\d+))?").unwrap();
  let cleaned = spec.replace("p.", "");
  for cap in re.captures_iter(&cleaned) {
    let start: u32 = cap[1].parse().unwrap_or(0);
    match cap.get(2) {
      Some(end_match) => {
        let end: u32 = end_match.as_str().parse().unwrap_or(start);
        for p in start..=end {
          pages.push(p);
        }
      }
      None => pages.push(start),
    }
  }
  pages
}

/// Parse bibliographic references from text.
///
/// Recognized formats (may be combined with `;`):
/// - `(Chapoutot2023 p.141)`
/// - `(Chapoutot2023 p.141-143)`
/// - `(Chapoutot2023 p.141, p.145)`
/// - `(Chapoutot2023 p.141 ; Werth2025 p.615)`
///
/// Only references whose key is present in `bib_ids` are returned.
pub fn parse_quotes(text: &str, bib_ids: &HashSet<String>) -> Vec<Quote> {
  let mut quotes = Vec::new();
  let paren_re = Regex::new(r"\(([^)]+)\)").unwrap();
  let ref_re =
    Regex::new(r"([A-Z][a-zA-Z]+\d{4})\s+(p\.\d[\d\-]*(?:,\s*p\.\d[\d\-]*)*)")
      .unwrap();

  for paren_cap in paren_re.captures_iter(text) {
    let inner = &paren_cap[1];
    for part in inner.split(';') {
      let part = part.trim();
      if let Some(ref_cap) = ref_re.captures(part) {
        let key = ref_cap[1].to_string();
        if bib_ids.contains(&key) {
          let pages = parse_pages(&ref_cap[2]);
          if !pages.is_empty() {
            quotes.push(Quote { key, pages });
          }
        }
      }
    }
  }

  quotes
}

#[cfg(test)]
mod tests {
  use super::*;

  fn bib(keys: &[&str]) -> HashSet<String> {
    keys.iter().map(|s| s.to_string()).collect()
  }

  #[test]
  fn test_single_page() {
    let bib_ids = bib(&["Chapoutot2023"]);
    let result = parse_quotes("texte (Chapoutot2023 p.141)", &bib_ids);
    assert_eq!(
      result,
      vec![Quote {
        key: "Chapoutot2023".to_string(),
        pages: vec![141]
      }]
    );
  }

  #[test]
  fn test_page_range() {
    let bib_ids = bib(&["Chapoutot2023"]);
    let result = parse_quotes("texte (Chapoutot2023 p.141-143)", &bib_ids);
    assert_eq!(
      result,
      vec![Quote {
        key: "Chapoutot2023".to_string(),
        pages: vec![141, 142, 143]
      }]
    );
  }

  #[test]
  fn test_multiple_pages() {
    let bib_ids = bib(&["Chapoutot2023"]);
    let result = parse_quotes("texte (Chapoutot2023 p.141, p.145)", &bib_ids);
    assert_eq!(
      result,
      vec![Quote {
        key: "Chapoutot2023".to_string(),
        pages: vec![141, 145]
      }]
    );
  }

  #[test]
  fn test_multiple_refs() {
    let bib_ids = bib(&["Chapoutot2023", "Werth2025"]);
    let result =
      parse_quotes("(Chapoutot2023 p.141 ; Werth2025 p.615)", &bib_ids);
    assert_eq!(
      result,
      vec![
        Quote {
          key: "Chapoutot2023".to_string(),
          pages: vec![141]
        },
        Quote {
          key: "Werth2025".to_string(),
          pages: vec![615]
        },
      ]
    );
  }

  #[test]
  fn test_unknown_key_filtered() {
    let bib_ids = bib(&["Chapoutot2023"]);
    let result = parse_quotes("texte (Unknown2023 p.50)", &bib_ids);
    assert!(result.is_empty());
  }

  #[test]
  fn test_no_reference() {
    let bib_ids = bib(&["Chapoutot2023"]);
    let result = parse_quotes("texte sans référence", &bib_ids);
    assert!(result.is_empty());
  }

  #[test]
  fn test_parse_pages_range() {
    assert_eq!(parse_pages("p.141-143"), vec![141, 142, 143]);
  }

  #[test]
  fn test_parse_pages_multi() {
    assert_eq!(parse_pages("p.141, p.145"), vec![141, 145]);
  }
}
