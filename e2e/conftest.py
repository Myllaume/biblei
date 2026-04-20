from pathlib import Path

# Racine du workspace
ROOT = Path(__file__).parent.parent
FIXTURES = ROOT / "e2e"

# Binaires compilés (cargo build doit avoir été lancé)
FETCHBIB_BIN = ROOT / "target" / "debug" / "fetchbib"
FETCHLEX_BIN = ROOT / "target" / "debug" / "fetchlex"
FILLTAG_BIN  = ROOT / "target" / "debug" / "filltag"
