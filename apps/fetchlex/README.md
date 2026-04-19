# Fetchbib

Application Rust.
Téléchargement du lexique français.

## Specifications

Téléchargement du fichier au format TSV vers format CSV.
On filtre les colonnes pour ne prendre que

- lemme : forme canonique du mot
- ortho : variante orthographique du mot

On filtre les lignes pour ne prendre que

- les noms communs (cgram = "NOM") et les adjectifs (cgram = "ADJ").
- les mots de plus de 2 lettres.

On exporte les fichiers

- `nouns_output` : noms communs et adjectifs à réutiliser pour l'analyse des fiches
- `nouns_errors` : erreurs de téléchargement et de filtrage

## Commandes

```bash
make build-fetchlex
make run-fetchlex
```
