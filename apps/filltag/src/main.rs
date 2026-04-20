use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use unidecode::unidecode;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    lex_output: String,
    tag_file: String,
    tag_output: String,
    tag_match: String,
}

fn load_config(config_path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}

fn normalize(s: &str) -> String {
    unidecode(s).to_lowercase()
}

/// Charge le lexique CSV (colonnes : lemme, ortho)
/// Retourne une map normalized_lemme → Vec<ortho>
fn load_lexique(path: &str) -> Result<HashMap<String, Vec<String>>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records() {
        let record = result?;
        let lemme = record.get(0).unwrap_or("").to_string();
        let ortho = record.get(1).unwrap_or("").to_string();
        if lemme.is_empty() || ortho.is_empty() {
            continue;
        }
        map.entry(normalize(&lemme)).or_default().push(ortho);
    }
    Ok(map)
}

/// Charge le fichier YAML des tags : catégorie → Vec<mot-clé>
fn load_tags(path: &str) -> Result<HashMap<String, Vec<String>>> {
    let content = std::fs::read_to_string(path)?;
    let tags: HashMap<String, Vec<String>> = serde_yaml::from_str(&content)?;
    Ok(tags)
}

fn ensure_parent_dir(path: &str) -> Result<()> {
    if let Some(dir) = std::path::Path::new(path).parent() {
        if !dir.as_os_str().is_empty() && !dir.exists() {
            std::fs::create_dir_all(dir)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let config = load_config("config.yml")?;

    println!("Chargement du lexique : {}", config.lex_output);
    let lexique = load_lexique(&config.lex_output)?;
    println!("✓ {} lemmes chargés", lexique.len());

    println!("Chargement des tags : {}", config.tag_file);
    let tags = load_tags(&config.tag_file)?;
    println!("✓ {} catégories chargées", tags.len());

    // Correspondance : pour chaque catégorie, on collecte :
    //   - la clé de catégorie elle-même
    //   - toutes les formes orthographiques des mots-clés trouvés dans le lexique
    let mut tag_output: HashMap<String, Vec<String>> = HashMap::new();
    // Résultat des correspondances : (mot-clé, trouvé)
    let mut tag_match_rows: Vec<(String, bool)> = Vec::new();

    for (category, keywords) in &tags {
        let mut forms: Vec<String> = vec![normalize(category)];
        for keyword in keywords {
            let norm = normalize(keyword);
            if let Some(orthos) = lexique.get(&norm) {
                forms.extend(orthos.iter().map(|o| normalize(o)));
                tag_match_rows.push((keyword.clone(), true));
            } else {
                forms.push(norm);
                tag_match_rows.push((keyword.clone(), false));
            }
        }
        tag_output.insert(category.clone(), forms);
    }

    // Écriture de tag_output (JSON)
    ensure_parent_dir(&config.tag_output)?;
    println!("Écriture de {}", config.tag_output);
    let output_file = File::create(&config.tag_output)?;
    serde_json::to_writer_pretty(output_file, &tag_output)?;
    println!("✓ {} écrit", config.tag_output);

    // Écriture de tag_match (CSV)
    ensure_parent_dir(&config.tag_match)?;
    println!("Écriture de {}", config.tag_match);
    let mut writer = csv::Writer::from_path(&config.tag_match)?;
    writer.write_record(["tag", "has_match"])?;
    for (tag, has_match) in &tag_match_rows {
        writer.write_record([tag.as_str(), if *has_match { "true" } else { "false" }])?;
    }
    writer.flush()?;
    println!("✓ {} écrit", config.tag_match);

    let matched = tag_match_rows.iter().filter(|(_, m)| *m).count();
    let total = tag_match_rows.len();
    println!("\nRésumé : {}/{} mots-clés trouvés dans le lexique", matched, total);

    Ok(())
}
