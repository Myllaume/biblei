use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
  // Simple test to verify tag parsing logic
  println!("Testing Tag Parsing Logic\n");

  // Simulating tags
  let tags = vec![
    ("politique", vec!["gouvernement"]),
    ("revolution", vec!["revolution", "revolutionnaire"]),
    ("marxisme", vec!["marxisme", "marxiste"]),
    ("socialisme", vec!["socialisme"]),
  ];

  // Test 1: Gouvernement
  let fulltext1 = "gouvernement britannique";
  let mut matched1 = Vec::new();
  for (tag_name, keywords) in &tags {
    for keyword in keywords {
      if fulltext1.contains(keyword) {
        if !matched1.contains(&tag_name.to_string()) {
          matched1.push(tag_name.to_string());
        }
        break;
      }
    }
  }
  println!("Test 1 - '{}': {:?}", fulltext1, matched1);

  // Test 2: Révolution
  let fulltext2 =
    "revolution francaise un evenement majeur avec des revolutionnaire";
  let mut matched2 = Vec::new();
  for (tag_name, keywords) in &tags {
    for keyword in keywords {
      if fulltext2.contains(keyword) {
        if !matched2.contains(&tag_name.to_string()) {
          matched2.push(tag_name.to_string());
        }
        break;
      }
    }
  }
  println!("Test 2 - '{}': {:?}", fulltext2, matched2);

  // Test 3: Marxisme et Socialisme
  let fulltext3 = "philosophie marxiste le marxisme et les marxistes ont influence le socialisme";
  let mut matched3 = Vec::new();
  for (tag_name, keywords) in &tags {
    for keyword in keywords {
      if fulltext3.contains(keyword) {
        if !matched3.contains(&tag_name.to_string()) {
          matched3.push(tag_name.to_string());
        }
        break;
      }
    }
  }
  println!("Test 3 - '{}': {:?}", fulltext3, matched3);

  Ok(())
}
