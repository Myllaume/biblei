use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
  #[serde(flatten)]
  pub tags: HashMap<String, Vec<String>>,
}

impl Tags {
  /// Load tags from a JSON file
  pub fn load(file_path: &str) -> Result<Self> {
    let content = fs::read_to_string(file_path)?;
    let tags: Tags = serde_json::from_str(&content)?;
    Ok(tags)
  }

  /// Parse tags from fulltext
  /// Returns a list of tag names that match keywords found in the fulltext
  pub fn parse_from_fulltext(&self, fulltext: &str) -> Vec<String> {
    let mut matched_tags = Vec::new();

    for (tag_name, keywords) in &self.tags {
      for keyword in keywords {
        // Case-insensitive search for keyword as a whole word
        if fulltext.contains(keyword) {
          if !matched_tags.contains(tag_name) {
            matched_tags.push(tag_name.clone());
          }
          break; // Move to next tag once one keyword matches
        }
      }
    }

    matched_tags
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_from_fulltext() {
    let mut tags_map = HashMap::new();
    tags_map.insert("politique".to_string(), vec!["gouvernement".to_string()]);
    tags_map.insert(
      "test".to_string(),
      vec!["foo".to_string(), "bar".to_string()],
    );

    let tags = Tags { tags: tags_map };

    let fulltext = "le gouvernement est responsable";
    let matched = tags.parse_from_fulltext(fulltext);
    assert!(matched.contains(&"politique".to_string()));
  }

  #[test]
  fn test_parse_from_fulltext_multiple_keywords() {
    let mut tags_map = HashMap::new();
    tags_map.insert(
      "socialisme".to_string(),
      vec![
        "socialisme".to_string(),
        "marxiste".to_string(),
        "marxisme".to_string(),
      ],
    );

    let tags = Tags { tags: tags_map };

    let fulltext = "le marxisme est une philosophie";
    let matched = tags.parse_from_fulltext(fulltext);
    assert!(matched.contains(&"socialisme".to_string()));
  }

  #[test]
  fn test_parse_from_fulltext_no_match() {
    let mut tags_map = HashMap::new();
    tags_map.insert("politique".to_string(), vec!["gouvernement".to_string()]);

    let tags = Tags { tags: tags_map };

    let fulltext = "ceci est un texte sans mots cles";
    let matched = tags.parse_from_fulltext(fulltext);
    assert!(matched.is_empty());
  }
}
