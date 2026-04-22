# ParseRec

Application Rust.
Collecte les fiches de lecture et les références bibliques dans le fichier des fiches.

## Specifications

Extrait tout les fiches au format YAML:

```yaml
- title: Titre de la fiche
  alias:
    - Alias de la fiche
  description: Description de la fiche
  tags:
    - Tag1
  references:
    - Reference1
```

### Éléments parsés

#### Dates

Input : `1 décembre 2023`
Output : [2023, 12, 1]

Input : `décembre 2023`
Output : [2023, 12]

Input : `2023`
Output : [2023]

#### Références bibliographiques

Input : `(Chapoutot2023 p.141)`
Output : `Chapoutot2023`, `[141]`

Input : `(Chapoutot2023 p.141-143)`
Output : `Chapoutot2023`, `[141,142,143]`

Input : `(Chapoutot2023 p.141, p.145)`
Output : `Chapoutot2023`, `[141,145]`

Input : `(Chapoutot2023 p.141 ; Werth2025 p.615)`
Output : `Chapoutot2023`, `[141]`, `Werth2025`, `[615]`

#### Tags

`tags.json` contient la liste des tags :

```json
{
  "politique": ["gouvernement"]
}
```

Input : `Le Gouvernement est responsable.`
Output : `politique`

#### Liens

```yaml
- title: Winston Churchill
  description: Premier ministre britannique.
- title: Charte de l'Atlantique
  description: Accord signé par Winston Churchill.
```

Output : `Charte de l'Atlantique` -> `Winston Churchill`

## Commandes

```bash
make build-parserec
make run-parserec
```
