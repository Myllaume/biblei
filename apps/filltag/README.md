# Filltag

Application Rust.
Complète la liste des mots-clés contenus dans `tag_file`.

## Specifications

Chargement des fichiers `lex_output` et `tag_file`.
Pour chaque mot-clé de `tag_file`, on cherche une correspondance dans `lex_output`.

### Correspondances

Pour faire correspondre un lemme de `lex_output` à un mot-clé de `tag_file`, on utilise les règles suivantes :

- ignore la casse
- ignore les accents et caractères non-ascii
- **correspondance stricte entre le mot-clé et le lemme**

### Données d'entrée

#### `tag_file` :

```yml
France:
  - français
Allemagne:
  - allemand
```

#### `lex_output` :

```yml
lemme,ortho
français,française
français,françaises
```

### Données de sortie

On exporte les fichiers

#### `tag_output`

Liste des mots-clés à réutiliser pour l'analyse des fiches.

```json
{
  "France": ["France", "français", "française", "françaises"],
  "Allemagne": ["Allemagne", "allemand", "allemande", "allemands", "allemandes"]
}
```

#### `tag_match`

Liste des correspondances et non-correspondances entre `tag_file` et `lex_output`.

```csv
tag,has_match
français,true
allemand,true
foo,false
```

## Commandes

```bash
make build-filltag
make run-filltag
```
