use unidecode::unidecode;

pub fn transliterate_non_ascii(input: &str) -> String {
  unidecode(input)
}

#[cfg(test)]
mod transliterate_non_ascii_tests {
  use super::*;

  #[test]
  fn transliterate_non_ascii_basic() {
    let input: &str = "Évènement";
    let expected = "Evenement";
    let result = transliterate_non_ascii(input);
    assert_eq!(result, expected);
  }
}

pub fn slugify(input: &str) -> String {
  let input = transliterate_non_ascii(input);

  input
    .trim()
    .to_lowercase()
    .split_whitespace()
    .collect::<Vec<_>>()
    .join("-")
}

#[cfg(test)]
mod slugify_tests {
  use super::*;

  #[test]
  fn slugify_basic() {
    let input: &str = "Conférence  de Stresa";
    let expected = "conference-de-stresa";
    let result = slugify(input);
    assert_eq!(result, expected);
  }
}

/// Normalise une chaîne pour la rendre insensible à la casse, aux accents
/// et à la ponctuation. Utilisée pour la détection des liens entre fiches.
pub fn ascii_lowercase(input: &str) -> String {
  // 1. Translittérer les caractères non-ASCII (supprime les accents)
  let input = transliterate_non_ascii(input);
  // 2. Mettre en minuscules
  input.to_lowercase()
}
