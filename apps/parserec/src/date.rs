use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Date {
  pub year: Option<u32>,
  pub month: Option<u32>,
  pub day: Option<u32>,
}

impl Date {
  pub fn to_array(&self) -> Vec<u32> {
    let mut result = Vec::new();
    if let Some(year) = self.year {
      result.push(year);
    }
    if let Some(month) = self.month {
      result.push(month);
    }
    if let Some(day) = self.day {
      result.push(day);
    }
    result
  }
}

fn month_name_to_number(month_name: &str) -> Option<u32> {
  match month_name.to_lowercase().as_str() {
    "janvier" | "january" | "jan" => Some(1),
    "février" | "fevrier" | "february" | "feb" => Some(2),
    "mars" | "march" | "mar" => Some(3),
    "avril" | "april" | "apr" => Some(4),
    "mai" | "may" => Some(5),
    "juin" | "june" | "jun" => Some(6),
    "juillet" | "july" | "jul" => Some(7),
    "août" | "aout" | "august" | "aug" => Some(8),
    "septembre" | "september" | "sep" => Some(9),
    "octobre" | "october" | "oct" => Some(10),
    "novembre" | "november" | "nov" => Some(11),
    "décembre" | "decembre" | "december" | "dec" => Some(12),
    _ => None,
  }
}

/// Parse dates from text
/// Supports formats like:
/// - "1 décembre 2023" -> Date { year: 2023, month: 12, day: 1 }
/// - "décembre 2023" -> Date { year: 2023, month: 12, day: None }
/// - "2023" -> Date { year: 2023, month: None, day: None }
pub fn parse_dates(text: &str) -> Vec<Date> {
  let mut dates = Vec::new();
  let mut parsed_years = HashSet::new();
  let text_lower = text.to_lowercase();

  // Pattern 1: day month year (e.g., "1 décembre 2023")
  let pattern1 = Regex::new(r"(\d{1,2})\s+(janvier|février|fevrier|mars|avril|mai|juin|juillet|août|aout|septembre|octobre|novembre|décembre|decembre)\s+(\d{4})").unwrap();
  for cap in pattern1.captures_iter(&text_lower) {
    if let (Ok(day), Some(month), Ok(year)) = (
      cap[1].parse::<u32>(),
      month_name_to_number(&cap[2]),
      cap[3].parse::<u32>(),
    ) {
      parsed_years.insert(year);
      dates.push(Date {
        year: Some(year),
        month: Some(month),
        day: Some(day),
      });
    }
  }

  // Pattern 2: month year (e.g., "décembre 2023")
  let pattern2 = Regex::new(r"(janvier|février|fevrier|mars|avril|mai|juin|juillet|août|aout|septembre|octobre|novembre|décembre|decembre)\s+(\d{4})").unwrap();
  for cap in pattern2.captures_iter(&text_lower) {
    if let (Some(month), Ok(year)) =
      (month_name_to_number(&cap[1]), cap[2].parse::<u32>())
    {
      parsed_years.insert(year);
      // Only add if not already captured by pattern1 with exact month
      if !dates
        .iter()
        .any(|d| d.year == Some(year) && d.month == Some(month))
      {
        dates.push(Date {
          year: Some(year),
          month: Some(month),
          day: None,
        });
      }
    }
  }

  // Pattern 3: year only (e.g., "2023")
  let pattern3 = Regex::new(r"\b(\d{4})\b").unwrap();
  for cap in pattern3.captures_iter(&text_lower) {
    if let Ok(year) = cap[1].parse::<u32>() {
      // Only add if not already captured by other patterns
      if !parsed_years.contains(&year) {
        dates.push(Date {
          year: Some(year),
          month: None,
          day: None,
        });
      }
    }
  }

  dates
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_day_month_year() {
    let dates = parse_dates("1 décembre 2023");
    assert_eq!(dates.len(), 1);
    assert_eq!(dates[0].day, Some(1));
    assert_eq!(dates[0].month, Some(12));
    assert_eq!(dates[0].year, Some(2023));
    assert_eq!(dates[0].to_array(), vec![2023, 12, 1]);
  }

  #[test]
  fn test_parse_month_year() {
    let dates = parse_dates("décembre 2023");
    assert_eq!(dates.len(), 1);
    assert_eq!(dates[0].day, None);
    assert_eq!(dates[0].month, Some(12));
    assert_eq!(dates[0].year, Some(2023));
    assert_eq!(dates[0].to_array(), vec![2023, 12]);
  }

  #[test]
  fn test_parse_year_only() {
    let dates = parse_dates("2023");
    assert_eq!(dates.len(), 1);
    assert_eq!(dates[0].day, None);
    assert_eq!(dates[0].month, None);
    assert_eq!(dates[0].year, Some(2023));
    assert_eq!(dates[0].to_array(), vec![2023]);
  }

  #[test]
  fn test_parse_multiple_dates_different_months() {
    let dates = parse_dates("janvier 2023 février 2024 mars 2025");
    assert!(dates.len() >= 3);
    // All three months should be found
    assert!(dates
      .iter()
      .any(|d| d.month == Some(1) && d.year == Some(2023)));
    assert!(dates
      .iter()
      .any(|d| d.month == Some(2) && d.year == Some(2024)));
    assert!(dates
      .iter()
      .any(|d| d.month == Some(3) && d.year == Some(2025)));
  }

  #[test]
  fn test_month_name_to_number() {
    assert_eq!(month_name_to_number("janvier"), Some(1));
    assert_eq!(month_name_to_number("février"), Some(2));
    assert_eq!(month_name_to_number("décembre"), Some(12));
    assert_eq!(month_name_to_number("invalid"), None);
  }

  #[test]
  fn test_parse_with_context() {
    let dates = parse_dates("L'événement a eu lieu le 15 mars 2022");
    assert_eq!(dates.len(), 1);
    assert_eq!(dates[0].day, Some(15));
    assert_eq!(dates[0].month, Some(3));
    assert_eq!(dates[0].year, Some(2022));
  }

  #[test]
  fn test_date_to_array() {
    let date1 = Date {
      year: Some(2023),
      month: Some(12),
      day: Some(1),
    };
    assert_eq!(date1.to_array(), vec![2023, 12, 1]);

    let date2 = Date {
      year: Some(2023),
      month: Some(12),
      day: None,
    };
    assert_eq!(date2.to_array(), vec![2023, 12]);

    let date3 = Date {
      year: Some(2023),
      month: None,
      day: None,
    };
    assert_eq!(date3.to_array(), vec![2023]);
  }

  #[test]
  fn test_parse_no_duplicates() {
    let dates = parse_dates("15 mars 2022 mars 2022 2022");
    // Should only have one entry for the complete date
    assert_eq!(dates.len(), 1);
    assert_eq!(dates[0].to_array(), vec![2022, 3, 15]);
  }
}
