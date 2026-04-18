# Fetchbib

Application Rust.
Téléchargement de la liste des notices bibliographiques depuis l'API Zotero.

## Specifications

Téléchargement du fichier au format CSL-JSON.
On vérifie et filtres les champs enregistrés.
Chaque notice est dotée d'un identifiant.

### Identifiant de notice

L'identifiant est formé tel que `<nom du premier auteur>+<date de parution>`.

## Commandes

```bash
make build-fetchbib
make run-fetchbib
```
