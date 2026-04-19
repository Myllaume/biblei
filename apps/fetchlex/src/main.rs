use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    lex_url: String,
    lex_output: String,
}

const LEXIQUE_CATEGORIES: &[&str] = &["NOM", "ADJ"];

fn load_config(config_path: &str) -> Result<Config> {
    let config_content = std::fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    Ok(config)
}

fn check_output_dir_exists(config: &Config) -> Result<()> {
    let output_dir = std::path::Path::new(&config.lex_output)
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

fn download_file(url: &str) -> Result<BufReader<reqwest::blocking::Response>> {
    let client = reqwest::blocking::Client::new();
    let response = match client.get(url).send() {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("✗ Erreur : impossible de télécharger le fichier");
            eprintln!("Détails : {}", e);
            return Err(e.into());
        }
    };
    println!("✓ Téléchargement réussi\n");
    Ok(BufReader::new(response))
}

fn validate_and_process_lexique(config: Config) -> Result<()> {
    println!("Téléchargement du fichier...");
    let reader = download_file(&config.lex_url)?;

    println!("Vérification du format TSV...");
    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(reader);

    // Récupérer et valider les en-têtes
    let headers = csv_reader.headers()?;
    println!("Colonnes trouvées : {} colonnes", headers.len());
    println!("✓ TSV parsable\n");

    println!("Vérification des colonnes requises...");
    let required_columns = vec!["ortho", "lemme", "cgram"];
    for col in &required_columns {
        let found = headers.iter().any(|h| h == *col);
        if found {
            println!("  ✓ Colonne '{}' trouvée", col);
        } else {
            eprintln!("✗ Colonne '{}' manquante", col);
            return Err(anyhow::anyhow!("Colonne '{}' manquante", col));
        }
    }

    // Récupérer les indices des colonnes
    let col_ortho = headers
        .iter()
        .position(|h| h == "ortho")
        .ok_or_else(|| anyhow::anyhow!("Colonne 'ortho' non trouvée"))?;
    let col_lemme = headers
        .iter()
        .position(|h| h == "lemme")
        .ok_or_else(|| anyhow::anyhow!("Colonne 'lemme' non trouvée"))?;
    let col_cgram = headers
        .iter()
        .position(|h| h == "cgram")
        .ok_or_else(|| anyhow::anyhow!("Colonne 'cgram' non trouvée"))?;

    println!("✓ Toutes les colonnes requises présentes\n");

    println!("Traitement des données...");

    // Ouvrir le fichier CSV de sortie
    let mut output_file = File::create(&config.lex_output)?;
    let mut csv_writer = csv::Writer::from_writer(&mut output_file);

    // Écrire l'en-tête
    csv_writer.write_record(&["lemme", "ortho"])?;

    let mut total_rows = 0;
    let mut written_rows = 0;
    let mut errors = 0;

    // Traiter chaque ligne
    for result in csv_reader.records() {
        let Ok(record) = result else {
            eprintln!("Erreur lors de la lecture d'une ligne : {}", result.unwrap_err());
            errors += 1;
            continue;
        };

        total_rows += 1;

        let (Some(ortho), Some(lemme), Some(cgram)) = (
            record.get(col_ortho),
            record.get(col_lemme),
            record.get(col_cgram),
        ) else {
            errors += 1;
            continue;
        };

        if ortho.trim().is_empty() || lemme.trim().is_empty() || cgram.trim().is_empty() {
            errors += 1;
            continue;
        }

        if lemme.trim().len() < 3 {
            continue;
        }

        if !LEXIQUE_CATEGORIES.contains(&cgram) {
            continue;
        }

        let ortho_lower = ortho.to_lowercase();
        let lemme_lower = lemme.to_lowercase();
        csv_writer.write_record(&[&lemme_lower, &ortho_lower])?;
        written_rows += 1;

        if written_rows % 10000 == 0 {
            println!("  {} lignes écrites", written_rows);
        }
    }

    csv_writer.flush()?;

    println!("\n--- Résumé du traitement ---");
    println!("Total de lignes lues : {}", total_rows);
    println!("Lignes écrites : {}", written_rows);
    if errors > 0 {
        println!("Erreurs : {}", errors);
    }
    println!("Fichier créé : {}", config.lex_output);

    Ok(())
}

fn main() -> Result<()> {
    let config = load_config("./config.yml")?;

    check_output_dir_exists(&config)?;

    if let Err(e) = validate_and_process_lexique(config) {
        eprintln!("\n✗ Échec : {}", e);
        return Err(e);
    }

    Ok(())
}
