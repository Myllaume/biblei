# biblei

Biblical text processing tools - Monorepo Python/Rust/TS.

## Commandes

- `make init` : initialise l’environnement du workspace via `./apps/init.sh`.
- `make build` : build toutes les apps Rust en release et lance `poetry install`.
- `make build-debug APP=<app>` : build une app Rust précise en debug, par exemple `APP=fetchbib`.
- `make run` : exécute la chaîne complète et génère tous les fichiers de sortie.
- `make test APP=<app>` : lance les tests unitaires de l’app indiquée.
- `make test APP=parsenouns` : lance les tests Python de l’app `parsenouns` via `pytest`.
- `make test-e2e` : lance les tests end-to-end du monorepo.
- `make format` : formate le code Rust et Python.
- `make lint` : vérifie le code Rust et Python avec les linters.

Apps disponibles pour `make test APP=...` : `fetchbib`, `fetchlex`, `filltag`, `parserec`, `parsenouns`.
