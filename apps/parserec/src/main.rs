use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

mod date;
mod quotes;
mod record;
mod string;
mod tags;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
  record_file: String,
  tag_output: String,
  record_output: String,
  #[serde(default)]
  bib_output: Option<String>,
  #[serde(default)]
  tags_file: Option<String>,
}

fn load_config(config_path: &str) -> Result<Config> {
  let config_content = std::fs::read_to_string(config_path)?;
  let config: Config = serde_yaml::from_str(&config_content)?;
  Ok(config)
}

fn main() -> Result<()> {
  let config = load_config("./config.yml")?;

  let records = record::load_records_with_tags(
    &config.record_file,
    config.tags_file.as_deref(),
    config.bib_output.as_deref(),
  )?;

  let json = serde_json::to_string_pretty(&records)?;
  let mut file = File::create(&config.record_output)?;
  file.write_all(json.as_bytes())?;

  println!(
    "Exported {} records to {}",
    records.len(),
    config.record_output
  );

  Ok(())
}
