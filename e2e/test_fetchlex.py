"""Tests E2E pour fetchlex.

fetchlex streame un TSV depuis une URL.
pytest-httpserver sert le fichier TSV de fixture en local.
Le binaire Rust lit les headers HTTP et streame le body — comportement identique
à une vraie requête, sans aucune modification du code.
"""

import csv
import subprocess
from pathlib import Path

import pytest
from conftest import FETCHLEX_BIN, FIXTURES
from pytest_httpserver import HTTPServer

CASES = sorted(
    [
        d
        for d in (FIXTURES / "fetchlex").iterdir()
        if d.is_dir() and d.name.startswith("case")
    ]
)


@pytest.mark.parametrize("case", CASES, ids=[c.name for c in CASES])
def test_fetchlex(case: Path, tmp_path: Path, httpserver: HTTPServer) -> None:
    # Configurer le mock HTTP : GET /lexique → réponse TSV
    tsv_content = (case / "lexique_response.tsv").read_bytes()
    httpserver.expect_request("/lexique").respond_with_data(
        tsv_content,
        content_type="text/tab-separated-values; charset=utf-8",
    )

    # Fichiers de sortie dans tmp_path.
    # Le répertoire doit exister pour lex_error.
    errors_path = tmp_path / "dist" / "lex_errors.csv"
    errors_path.parent.mkdir(parents=True, exist_ok=True)

    config = (
        f"lex_url: {httpserver.url_for('/lexique')}\n"
        f"lex_output: {tmp_path / 'lexique.csv'}\n"
        f"lex_error: {errors_path}\n"
        # Clés non utilisées par fetchlex
        f"bib_url: unused\n"
        f"bib_output: unused\n"
        f"record_file: unused\n"
        f"tag_file: unused\n"
        f"tag_output: unused\n"
        f"tag_match: unused\n"
        f"nouns_output: unused\n"
    )
    (tmp_path / "config.yml").write_text(config)

    result = subprocess.run(
        [str(FETCHLEX_BIN)],
        cwd=tmp_path,
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, f"fetchlex a échoué:\n{result.stderr}"

    # Comparaison ligne à ligne (l'ordre est déterministe : ordre du TSV)
    def read_csv_rows(path: Path) -> list[tuple[str, ...]]:
        with path.open(newline="", encoding="utf-8") as f:
            reader = csv.reader(f)
            return [tuple(row) for row in reader]

    actual_rows = read_csv_rows(tmp_path / "lexique.csv")
    expected_rows = read_csv_rows(case / "expected_lexique.csv")
    assert actual_rows == expected_rows
