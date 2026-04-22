"""Tests E2E pour parsenouns.

parsenouns est une transformation pure fichier→fichier (pas de HTTP).
Le test génère un config.yml temporaire pointant vers les fixtures,
lance le script Python via poetry, et compare la sortie nouns.csv
avec le fichier attendu.
"""

import csv
import subprocess
import sys
from pathlib import Path

import pytest
from conftest import FIXTURES

CASES = sorted(
    [
        d
        for d in (FIXTURES / "parsenouns").iterdir()
        if d.is_dir() and d.name.startswith("case")
    ]
)


@pytest.mark.parametrize("case", CASES, ids=[c.name for c in CASES])
def test_parsenouns(case: Path, tmp_path: Path) -> None:
    config = (
        f"record_file: {case / 'records.yml'}\n"
        f"nouns_output: {tmp_path / 'nouns.csv'}\n"
    )
    (tmp_path / "config.yml").write_text(config)

    result = subprocess.run(
        [sys.executable, "-m", "parsenouns"],
        cwd=tmp_path,
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, f"parsenouns a échoué:\n{result.stderr}"

    def read_csv_rows(path: Path) -> dict[str, int]:
        with path.open(newline="", encoding="utf-8") as f:
            reader = csv.DictReader(f)
            return {row["noun"]: int(row["count"]) for row in reader}

    actual = read_csv_rows(tmp_path / "nouns.csv")
    expected = read_csv_rows(case / "exected_nouns.csv")
    assert actual == expected, (
        f"Noms attendus: {expected}, obtenus: {actual}"
    )
