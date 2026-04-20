"""Tests E2E pour filltag.

filltag est une transformation pure fichier→fichier (pas de HTTP).
Le test génère un config.yml temporaire pointant vers les fixtures,
lance le binaire compilé, et compare les sorties avec les fichiers attendus.
"""

import csv
import json
import subprocess
from pathlib import Path

import pytest

from conftest import FILLTAG_BIN, FIXTURES


CASES = sorted(
    [
        d
        for d in (FIXTURES / "filltag").iterdir()
        if d.is_dir() and d.name.startswith("case")
    ]
)


@pytest.mark.parametrize("case", CASES, ids=[c.name for c in CASES])
def test_filltag(case: Path, tmp_path: Path) -> None:
    # Construire un config.yml temporaire pointant vers les fixtures
    config = (
        f"lex_output: {case / 'lexique.csv'}\n"
        f"tag_file: {case / 'dict.yml'}\n"
        f"tag_output: {tmp_path / 'tags.json'}\n"
        f"tag_match: {tmp_path / 'tag_match.csv'}\n"
        # Clés non utilisées par filltag mais requises par la struct Config
        f"lex_url: unused\n"
        f"bib_url: unused\n"
        f"bib_output: unused\n"
        f"record_file: unused\n"
        f"nouns_output: unused\n"
    )
    (tmp_path / "config.yml").write_text(config)

    result = subprocess.run(
        [str(FILLTAG_BIN)],
        cwd=tmp_path,
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, f"filltag a échoué:\n{result.stderr}"

    # --- Vérification tags.json ---
    actual_tags = json.loads((tmp_path / "tags.json").read_text())
    expected_tags = json.loads((case / "expected_tags.json").read_text())

    # Normaliser les tableaux de formes avant comparaison (ordre déterministe)
    assert set(actual_tags.keys()) == set(expected_tags.keys())
    for key in expected_tags:
        assert sorted(actual_tags[key]) == sorted(expected_tags[key]), (
            f"Catégorie '{key}': attendu {sorted(expected_tags[key])}, "
            f"obtenu {sorted(actual_tags[key])}"
        )

    # --- Vérification tag_match.csv ---
    def read_csv_rows(path: Path) -> set[tuple[str, ...]]:
        with path.open(newline="", encoding="utf-8") as f:
            reader = csv.DictReader(f)
            return {(row["tag"], row["has_match"]) for row in reader}

    actual_match = read_csv_rows(tmp_path / "tag_match.csv")
    expected_match = read_csv_rows(case / "expected_tag_match.csv")
    assert actual_match == expected_match
