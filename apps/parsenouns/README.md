# Fetchbib

Application Python.
Collecte des noms communs et noms propres dans le fichier des fiches.

## Specifications

Scanne de tout les termes, au masculin neutre.
Les termes sont listés dans un fichier CSV composé de deux colonnes :

1. terme
2. nombre d'occurences

### Termes ignorés

- clés de citations tel que `Author2026`
- numéros de page tel que `p.615`
- clés yaml tel que `word:`

## Commandes

```bash
make run-parsenouns
```
