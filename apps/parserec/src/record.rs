use crate::date;
use crate::quotes::{load_bib_ids, parse_quotes, Quote};
use crate::string::ascii_lowercase;
use crate::string::slugify;
use crate::tags::Tags;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use validator::{Validate, ValidationError};
// mod link;
// mod parse;

fn validate_tags_not_blank(tags: &Vec<String>) -> Result<(), ValidationError> {
  for tag in tags {
    if tag.trim().is_empty() {
      let mut error = ValidationError::new("blank_tag");
      error.message = Some("Un tag ne peut pas être une chaîne vide".into());
      return Err(error);
    }
  }
  Ok(())
}

fn validate_alias_not_blank(
  alias: &Vec<String>,
) -> Result<(), ValidationError> {
  for a in alias {
    if a.trim().is_empty() {
      let mut error = ValidationError::new("blank_alias");
      error.message = Some("Un alias ne peut pas être une chaîne vide".into());
      return Err(error);
    }
  }
  Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Record {
  #[serde(skip_deserializing)]
  pub url: String,

  #[validate(length(min = 1, message = "Le titre ne peut pas être vide"))]
  pub title: String,

  #[validate(custom(function = "validate_alias_not_blank"))]
  #[serde(default)]
  pub alias: Vec<String>,

  #[validate(custom(function = "validate_tags_not_blank"))]
  #[serde(default)]
  pub tags: Vec<String>,

  pub description: Option<String>,

  pub fulltext: String,

  #[serde(skip_deserializing)]
  pub dates: Vec<Vec<u32>>,

  #[serde(skip_deserializing)]
  pub quotes: Vec<Quote>,

  #[serde(skip_deserializing)]
  pub links: Vec<String>,

  #[serde(skip_deserializing)]
  pub backlinks: Vec<String>,
}

impl Record {
  fn gen_url(title: &str) -> String {
    slugify(title)
  }
}

fn gen_fulltext(title: &str, description: &Option<String>) -> String {
  let mut full_text = title.to_string();
  if let Some(desc) = description {
    full_text.push(' ');
    full_text.push_str(desc);
  }
  ascii_lowercase(&full_text)
}

pub fn load_records_with_tags(
  file_path: &str,
  tags_file: Option<&str>,
  bib_file: Option<&str>,
) -> Result<Vec<Record>> {
  let content = fs::read_to_string(file_path)?;

  // Deserialize as a vector of Record without URLs
  #[derive(Debug, Deserialize)]
  struct RecordWithoutUrl {
    title: String,
    #[serde(default)]
    alias: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
    description: Option<String>,
  }

  let records_without_url: Vec<RecordWithoutUrl> =
    serde_yaml::from_str(&content)?;

  // Load tags if provided
  let tags = if let Some(tags_path) = tags_file {
    Some(Tags::load(tags_path)?)
  } else {
    None
  };

  // Load bib IDs if provided
  let bib_ids: HashSet<String> = if let Some(bib_path) = bib_file {
    load_bib_ids(bib_path)?
  } else {
    HashSet::new()
  };

  // Convert to Record with generated URLs and parsed tags
  let records = records_without_url
    .into_iter()
    .map(|r| {
      let fulltext = gen_fulltext(&r.title, &r.description);
      let mut parsed_tags = r.tags;

      // Parse tags from fulltext if tags are available
      if let Some(ref tags_obj) = tags {
        let auto_tags = tags_obj.parse_from_fulltext(&fulltext);
        for tag in auto_tags {
          if !parsed_tags.contains(&tag) {
            parsed_tags.push(tag);
          }
        }
      }

      // Parse dates from fulltext
      let parsed_dates = date::parse_dates(&fulltext)
        .into_iter()
        .map(|d| d.to_array())
        .collect();

      // Parse quotes from raw description (preserves original case for bib keys)
      let raw_text =
        format!("{} {}", r.title, r.description.as_deref().unwrap_or(""));
      let quotes = parse_quotes(&raw_text, &bib_ids);

      Record {
        url: Record::gen_url(&r.title),
        title: r.title.clone(),
        alias: r.alias,
        tags: parsed_tags,
        description: r.description.clone(),
        fulltext,
        dates: parsed_dates,
        quotes,
        links: Vec::new(),
        backlinks: Vec::new(),
      }
    })
    .collect();

  // Compute links and backlinks
  let records = resolve_links(records);

  Ok(records)
}

/// Build the search terms for a record: normalized title + normalized aliases.
fn search_terms(
  url: &str,
  title: &str,
  alias: &[String],
) -> Vec<(String, String)> {
  let mut terms = vec![(url.to_string(), ascii_lowercase(title))];
  for a in alias {
    terms.push((url.to_string(), ascii_lowercase(a)));
  }
  terms
}

/// For each record, scan every other record's fulltext for this record's title/aliases.
/// Populate `links` and `backlinks` based on matches.
fn resolve_links(mut records: Vec<Record>) -> Vec<Record> {
  // Build index: url -> list of normalized search terms
  let term_index: Vec<(String, Vec<String>)> = records
    .iter()
    .map(|r| {
      let terms = search_terms(&r.url, &r.title, &r.alias)
        .into_iter()
        .map(|(_, term)| term)
        .collect();
      (r.url.clone(), terms)
    })
    .collect();

  let n = records.len();
  for i in 0..n {
    for j in 0..n {
      if i == j {
        continue;
      }
      let target_url = &term_index[j].0;
      let target_terms = &term_index[j].1;
      let source_fulltext = &records[i].fulltext;

      let matches = target_terms
        .iter()
        .any(|term| source_fulltext.contains(term.as_str()));
      if matches {
        let target_url = target_url.clone();
        let source_url = records[i].url.clone();
        if !records[i].links.contains(&target_url) {
          records[i].links.push(target_url.clone());
        }
        if !records[j].backlinks.contains(&source_url) {
          records[j].backlinks.push(source_url);
        }
      }
    }
  }

  records
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_record(
    url: &str,
    title: &str,
    alias: Vec<&str>,
    fulltext: &str,
  ) -> Record {
    Record {
      url: url.to_string(),
      title: title.to_string(),
      alias: alias.into_iter().map(|s| s.to_string()).collect(),
      tags: Vec::new(),
      description: None,
      fulltext: fulltext.to_string(),
      dates: Vec::new(),
      quotes: Vec::new(),
      links: Vec::new(),
      backlinks: Vec::new(),
    }
  }

  #[test]
  fn test_links_basic() {
    let records = vec![
      make_record(
        "winston-churchill",
        "Winston Churchill",
        vec![],
        "premier ministre britannique",
      ),
      make_record(
        "charte-de-l-atlantique",
        "Charte de l'Atlantique",
        vec![],
        "accord signe par winston churchill",
      ),
    ];
    let resolved = resolve_links(records);

    let churchill = resolved
      .iter()
      .find(|r| r.url == "winston-churchill")
      .unwrap();
    let charte = resolved
      .iter()
      .find(|r| r.url == "charte-de-l-atlantique")
      .unwrap();

    assert!(
      charte.links.contains(&"winston-churchill".to_string()),
      "Charte doit avoir un lien vers Churchill"
    );
    assert!(
      churchill
        .backlinks
        .contains(&"charte-de-l-atlantique".to_string()),
      "Churchill doit avoir un rétrolien depuis Charte"
    );
  }

  #[test]
  fn test_links_via_alias() {
    let records = vec![
      make_record(
        "winston-churchill",
        "Winston Churchill",
        vec!["Churchill"],
        "premier ministre britannique",
      ),
      make_record(
        "charte-de-l-atlantique",
        "Charte de l'Atlantique",
        vec![],
        "accord signe par churchill",
      ),
    ];
    let resolved = resolve_links(records);

    let charte = resolved
      .iter()
      .find(|r| r.url == "charte-de-l-atlantique")
      .unwrap();
    assert!(
      charte.links.contains(&"winston-churchill".to_string()),
      "Charte doit lier Churchill via alias"
    );
  }

  #[test]
  fn test_no_self_links() {
    let records = vec![make_record(
      "winston-churchill",
      "Winston Churchill",
      vec![],
      "winston churchill premier ministre",
    )];
    let resolved = resolve_links(records);

    let churchill = &resolved[0];
    assert!(churchill.links.is_empty(), "Pas d'auto-lien");
    assert!(churchill.backlinks.is_empty(), "Pas d'auto-rétrolien");
  }

  #[test]
  fn test_no_duplicate_links() {
    // Title et alias pointent tous deux vers la même cible
    let records = vec![
      make_record("cible", "Cible", vec!["la cible"], "une fiche cible"),
      make_record(
        "source",
        "Source",
        vec![],
        "cible la cible sont mentionnees",
      ),
    ];
    let resolved = resolve_links(records);

    let source = resolved.iter().find(|r| r.url == "source").unwrap();
    let count = source
      .links
      .iter()
      .filter(|l| l.as_str() == "cible")
      .count();
    assert_eq!(
      count, 1,
      "Un lien unique même si titre et alias matchent tous les deux"
    );
  }

  #[test]
  fn test_backlinks_populated() {
    let records = vec![
      make_record("a", "Alpha", vec![], "texte sans reference"),
      make_record("b", "Beta", vec![], "texte mentionnant alpha"),
      make_record("c", "Gamma", vec![], "texte mentionnant alpha aussi"),
    ];
    let resolved = resolve_links(records);

    let alpha = resolved.iter().find(|r| r.url == "a").unwrap();
    assert!(alpha.backlinks.contains(&"b".to_string()));
    assert!(alpha.backlinks.contains(&"c".to_string()));
  }
}
