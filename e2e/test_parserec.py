"""Tests E2E pour parserec.

parserec est une transformation pure fichier→fichier (pas de HTTP).
Le test génère un config.yml temporaire pointant vers les fixtures,
lance le binaire compilé, et compare la sortie records.json avec le
fichier attendu.
"""

import json
import subprocess
from pathlib import Path

import pytest
from conftest import FIXTURES, PARSEREC_BIN

CASES = sorted(
    [
        d
        for d in (FIXTURES / "parserec").iterdir()
        if d.is_dir() and d.name.startswith("case")
    ]
)


@pytest.mark.parametrize("case", CASES, ids=[c.name for c in CASES])
def test_parserec(case: Path, tmp_path: Path) -> None:
    config = (
        f"record_file: {case / 'records.yml'}\n"
        f"record_output: {tmp_path / 'records.json'}\n"
        f"tag_output: {FIXTURES.parent / 'assets' / 'tags.json'}\n"
        f"bib_output: {FIXTURES.parent / 'assets' / 'bib.json'}\n"
    )
    (tmp_path / "config.yml").write_text(config)

    result = subprocess.run(
        [str(PARSEREC_BIN)],
        cwd=tmp_path,
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, f"parserec a échoué:\n{result.stderr}"

    actual = json.loads((tmp_path / "records.json").read_text(encoding="utf-8"))
    expected = json.loads(
        (case / "expected_records.json").read_text(encoding="utf-8")
    )

    # Comparaison insensible à l'ordre des enregistrements.
    # On indexe par url.
    actual_by_url = {r["url"]: r for r in actual}
    expected_by_url = {r["url"]: r for r in expected}
    assert set(actual_by_url.keys()) == set(expected_by_url.keys()), (
        f"URLs attendues: {set(expected_by_url.keys())}, "
        f"obtenues: {set(actual_by_url.keys())}"
    )
    for url, expected_record in expected_by_url.items():
        actual_record = actual_by_url[url]
        # Listes non ordonnées : links, backlinks, tags, alias, quotes
        for field in ("links", "backlinks", "tags", "alias"):
            assert sorted(actual_record[field]) == sorted(
                expected_record[field]
            ), (
                f"[{url}] champ '{field}': attendu "
                f"{sorted(expected_record[field])}, "
                f"obtenu {sorted(actual_record[field])}"
            )
        # Listes ordonnées : dates, quotes
        assert actual_record["dates"] == expected_record["dates"], (
            f"[{url}] champ 'dates': attendu {expected_record['dates']}, "
            f"obtenu {actual_record['dates']}"
        )
        assert actual_record["quotes"] == expected_record["quotes"], (
            f"[{url}] champ 'quotes': attendu {expected_record['quotes']}, "
            f"obtenu {actual_record['quotes']}"
        )
        # Scalaires
        for field in ("url", "title", "description", "fulltext"):
            assert actual_record[field] == expected_record[field], (
                f"[{url}] champ '{field}': attendu {expected_record[field]!r}, "
                f"obtenu {actual_record[field]!r}"
            )
