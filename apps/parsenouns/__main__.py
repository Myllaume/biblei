"""Main entry point for parsenouns."""

import csv
import logging
from collections import Counter
from pathlib import Path
import re

import spacy
import yaml
from spacy.language import Language
from spacy.tokens import Token

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)

logger = logging.getLogger(__name__)

IGNORED_NOUNS = {"description", "title"}

NAME_YEAR_REGEX = r"^[a-z]+\d{4}$"
PAGE_REGEX= r"^p.\d+$"


def main():
    config_path = Path("config.yml")
    with config_path.open(encoding="utf-8") as f:
        config = yaml.safe_load(f)

    record_file = Path(config["record_file"])
    nouns_output = Path(config["nouns_output"])

    logger.info(f"Lecture de {record_file}")
    text = record_file.read_text(encoding="utf-8")
    text = text.replace('"', "")

    logger.info("Chargement du modèle spacy fr_core_news_sm")
    nlp: Language = spacy.load("fr_core_news_sm")

    # spacy traite 1M caractères par défaut, on désactive la limite
    nlp.max_length = max(len(text) + 1, nlp.max_length)

    lines = [line for line in text.splitlines() if line.strip()]
    total = len(lines)
    logger.info(f"Analyse du texte ({total} lignes)")

    counts: Counter[str] = Counter()
    token: Token
    log_step = max(1, total // 10)

    for i, doc in enumerate(nlp.pipe(lines, batch_size=50)):
        if i % log_step == 0:
            logger.info(f"  {i}/{total} lignes traitées ({100 * i // total}%)")
        for token in doc:
            if token.pos_ not in ("NOUN", "PROPN"):
                continue

            lemma = token.lemma_.strip().lower()
            if len(lemma) < 3:
                continue
            if not lemma or lemma in IGNORED_NOUNS:
                continue
            if not any(c.isalpha() for c in lemma):
                continue
            if re.match(NAME_YEAR_REGEX, lemma):
                continue
            if re.match(PAGE_REGEX, lemma):
                continue
            counts[lemma] += 1

    logger.info(f"  {total}/{total} lignes traitées (100%)")

    nouns_output.parent.mkdir(parents=True, exist_ok=True)
    with nouns_output.open("w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(["noun", "count"])
        for noun, count in sorted(counts.items(), key=lambda x: x[1], reverse=True):
            writer.writerow([noun, count])

    logger.info(f"{len(counts)} noms écrits dans {nouns_output}")


if __name__ == "__main__":
    main()
