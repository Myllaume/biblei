"""Tests E2E pour fetchbib.

fetchbib fait un GET HTTP vers l'API Zotero.
pytest-httpserver démarre un vrai serveur local sur localhost.
Le config.yml de test pointe bib_url vers ce serveur.
Le binaire Rust ne sait pas qu'il parle à un mock — aucune modification du code.
"""

import json
import subprocess
from pathlib import Path

import pytest
from conftest import FETCHBIB_BIN, FIXTURES
from pytest_httpserver import HTTPServer

CASES = sorted(
    [
        d
        for d in (FIXTURES / "fetchbib").iterdir()
        if d.is_dir() and d.name.startswith("case")
    ]
)


@pytest.mark.parametrize("case", CASES, ids=[c.name for c in CASES])
def test_fetchbib(case: Path, tmp_path: Path, httpserver: HTTPServer) -> None:
    # Configurer le mock HTTP : GET /items → réponse JSON Zotero
    zotero_json = (case / "zotero_response.json").read_text(encoding="utf-8")
    httpserver.expect_request("/items").respond_with_data(
        zotero_json,
        content_type="application/json",
    )

    # config.yml temporaire : bib_url pointe vers le mock local
    config = (
        f"bib_url: {httpserver.url_for('/items')}\n"
        f"bib_output: {tmp_path / 'bib.json'}\n"
        # Clés non utilisées par fetchbib
        f"lex_url: unused\n"
        f"lex_output: unused\n"
        f"lex_error: unused\n"
        f"record_file: unused\n"
        f"tag_file: unused\n"
        f"tag_output: unused\n"
        f"tag_match: unused\n"
        f"nouns_output: unused\n"
    )
    (tmp_path / "config.yml").write_text(config)

    result = subprocess.run(
        [str(FETCHBIB_BIN)],
        cwd=tmp_path,
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, f"fetchbib a échoué:\n{result.stderr}"

    actual = json.loads((tmp_path / "bib.json").read_text(encoding="utf-8"))
    expected = json.loads(
        (case / "expected_bib.json").read_text(encoding="utf-8")
    )

    # Comparaison insensible à l'ordre des clés JSON
    # (serde_json::to_string_pretty peut varier) — on compare item par item
    # en se basant sur l'id.
    actual_by_id = {item["id"]: item for item in actual}
    expected_by_id = {item["id"]: item for item in expected}
    assert actual_by_id == expected_by_id
