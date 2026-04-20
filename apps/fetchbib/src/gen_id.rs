use serde_json::Value;

pub fn gen_id(zotero_item: &Value) -> Option<String> {
  let first_author_family =
    zotero_item.get("author")?.get(0)?.get("family")?.as_str()?;

  let year = zotero_item
    .get("issued")?
    .get("date-parts")?
    .get(0)?
    .get(0)?
    .as_i64()?;

  Some(format!("{}{}", first_author_family, year))
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[test]
  fn test_gen_id_with_valid_data() {
    let zotero_item = json!({
      "author": [
        {
          "family": "Smith",
          "given": "John"
        }
      ],
      "issued": {
        "date-parts": [[2020]]
      }
    });

    let result = gen_id(&zotero_item);
    assert_eq!(result, Some("Smith2020".to_string()));
  }

  #[test]
  fn test_gen_id_with_multiple_authors() {
    let zotero_item = json!({
      "author": [
        {
          "family": "Doe",
          "given": "Jane"
        },
        {
          "family": "Smith",
          "given": "John"
        }
      ],
      "issued": {
        "date-parts": [[2021]]
      }
    });

    let result = gen_id(&zotero_item);
    assert_eq!(result, Some("Doe2021".to_string()));
  }

  #[test]
  fn test_gen_id_missing_author() {
    let zotero_item = json!({
      "issued": {
        "date-parts": [[2020]]
      }
    });

    let result = gen_id(&zotero_item);
    assert_eq!(result, None);
  }

  #[test]
  fn test_gen_id_missing_year() {
    let zotero_item = json!({
      "author": [
        {
          "family": "Smith",
          "given": "John"
        }
      ]
    });

    let result = gen_id(&zotero_item);
    assert_eq!(result, None);
  }

  #[test]
  fn test_gen_id_empty_authors() {
    let zotero_item = json!({
      "author": [],
      "issued": {
        "date-parts": [[2022]]
      }
    });

    let result = gen_id(&zotero_item);
    assert_eq!(result, None);
  }
}
