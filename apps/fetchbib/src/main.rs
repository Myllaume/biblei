use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    bib_url: String,
    bib_output: String,
}

fn load_config(config_path: &str) -> Result<Config> {
    let config_content = std::fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    Ok(config)
}

fn check_output_dir_exists(config: &Config) -> Result<()> {
    let output_dir = std::path::Path::new(&config.bib_output)
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Impossible de déterminer le répertoire de sortie"))?;

    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)?;
        println!("✓ Répertoire de sortie créé : {}", output_dir.display());
    } else {
        println!(
            "✓ Répertoire de sortie existe déjà : {}",
            output_dir.display()
        );
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub family: String,
    pub given: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IssuedDate {
    #[serde(rename = "date-parts")]
    pub date_parts: Vec<Vec<i64>>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ZoteroItem {
    #[validate(length(min = 1, message = "id ne doit pas être vide"))]
    pub id: String,

    #[validate(length(min = 1, message = "title ne doit pas être vide"))]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,

    #[validate(length(min = 1, message = "author ne doit pas être vide"))]
    pub author: Vec<Author>,

    pub issued: IssuedDate,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "type")]
    pub item_type: String,
}

fn gen_id(zotero_item: &Value) -> Option<String> {
    let first_author_family = zotero_item.get("author")?.get(0)?.get("family")?.as_str()?;

    let year = zotero_item
        .get("issued")?
        .get("date-parts")?
        .get(0)?
        .get(0)?
        .as_i64()?;

    Some(format!("{}{}", first_author_family, year))
}

fn convert_to_zotero_item(raw_item: &Value) -> Result<ZoteroItem> {
    let id = gen_id(raw_item)
        .ok_or_else(|| anyhow::anyhow!("Impossible de générer l'ID pour l'entrée"))?;

    let mut item: ZoteroItem = serde_json::from_value(raw_item.clone())?;
    item.id = id;

    item.validate()
        .map_err(|e| anyhow::anyhow!("Validation de l'entrée échouée: {}", e))?;

    Ok(item)
}

fn fetch_zotero_data(url: &str) -> Result<String> {
    println!("Téléchargement du fichier...");
    let client = reqwest::blocking::Client::new();
    let response = match client.get(url).send() {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("✗ Erreur : impossible de télécharger le fichier");
            eprintln!("Détails : {}", e);
            return Err(e.into());
        }
    };

    match response.text() {
        Ok(text) => {
            println!("✓ Téléchargement réussi\n");
            Ok(text)
        }
        Err(e) => {
            eprintln!("✗ Erreur : impossible de lire le contenu téléchargé");
            eprintln!("Détails : {}", e);
            Err(e.into())
        }
    }
}

fn parse_zotero_json(content: &str) -> Result<Value> {
    println!("Vérification du format JSON...");
    match serde_json::from_str(content) {
        Ok(data) => {
            println!("✓ JSON parsable\n");
            Ok(data)
        }
        Err(e) => {
            eprintln!("✗ Erreur : impossible de parser le JSON");
            eprintln!("Détails : {}", e);
            Err(e.into())
        }
    }
}

fn validate_and_convert_items(raw_items: &[Value]) -> (Vec<ZoteroItem>, Vec<String>) {
    println!("Validation des entrées...");
    let mut validated_items = Vec::new();
    let mut errors = Vec::new();

    for (index, raw_item) in raw_items.iter().enumerate() {
        match convert_to_zotero_item(raw_item) {
            Ok(item) => {
                validated_items.push(item);
            }
            Err(e) => {
                let error_msg = format!(
                    "✗ Erreur : impossible de valider l'entrée à l'index {}: {}",
                    index, e
                );
                eprintln!("{}", error_msg);
                errors.push(error_msg);
            }
        }
    }

    if !errors.is_empty() {
        eprintln!(
            "Attention : {} entrée(s) n'ont pas pu être validées",
            errors.len()
        );
    } else {
        println!("✓ Toutes les entrées sont valides\n");
    }

    (validated_items, errors)
}

fn save_bibliography(items: &[ZoteroItem], output_path: &str) -> Result<()> {
    println!("Écriture du fichier...");
    let json_bib = serde_json::to_string_pretty(items)?;
    let mut file = File::create(output_path)?;
    file.write_all(json_bib.as_bytes())?;
    println!("\n--- Résumé du traitement ---");
    println!("Total d'entrées écrites : {}", items.len());
    println!("Fichier créé : {}", output_path);
    Ok(())
}

fn main() -> Result<()> {
    let config = load_config("./config.yml")?;

    check_output_dir_exists(&config)?;

    let content = fetch_zotero_data(&config.bib_url)?;
    let zotero_data: Value = parse_zotero_json(&content)?;

    if let Some(raw_items) = zotero_data.get("items").and_then(|v| v.as_array()) {
        let (validated_items, _errors) = validate_and_convert_items(raw_items);

        if validated_items.is_empty() {
            eprintln!("\n✗ Échec : aucune entrée valide trouvée");
            return Ok(());
        }

        save_bibliography(&validated_items, &config.bib_output)?;
    } else {
        eprintln!("\n✗ Échec : impossible de trouver les items dans la réponse JSON");
    }

    Ok(())
}
