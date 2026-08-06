#!/usr/bin/env python3
"""Régénère le tableau des solutions du README.

Détecte les dossiers de problèmes absents du tableau, retrouve leurs
métadonnées (message de commit + API LeetCode) et insère les lignes
manquantes en haut du tableau ; au-delà de 30 lignes, les plus
anciennes basculent dans l'archive.
"""

import argparse
import difflib
import json
import re
import subprocess
import sys
import urllib.request
from pathlib import Path

PROBLEM_DIR_RE = re.compile(r"^[a-z0-9_]+_(\d+)$")
PERF_RE = re.compile(r"\((\d+ms) - ([\d.]+mb)\)\s*$", re.IGNORECASE)
TABLE_HEADER = "| # | Problem | Difficulty | Technique | Runtime / Memory |"
TABLE_SEP = "|---:|---------|:---:|-----------|:---:|"
ROW_RE = re.compile(r"^\| (\d+) \|")
MAX_VISIBLE_ROWS = 30
COUNT_RE = re.compile(r"^## Solutions — \d+ solved$", re.MULTILINE)


def extract_problem_number(dirname):
    """Numéro d'un dossier « problem_name_number », sinon None."""
    m = PROBLEM_DIR_RE.match(dirname)
    return int(m.group(1)) if m else None


def parse_commit_subject(subject):
    """« 739 - Daily temperatures - Stack (0ms - 4.38mb) » → (739, "Stack", "0ms / 4.38mb").

    Retourne None si le sujet ne commence pas par « <numéro> - ».
    Technique/perf absentes ou non parsables → « — ».
    """
    parts = subject.split(" - ", 2)
    if len(parts) < 2 or not parts[0].strip().isdigit():
        return None
    number = int(parts[0])
    technique, perf = "—", "—"
    if len(parts) == 3:
        technique = parts[2].strip()
        m = PERF_RE.search(technique)
        if m:
            perf = f"{m.group(1)} / {m.group(2)}"
            technique = technique[: m.start()].strip() or "—"
    return (number, technique, perf)


def readme_numbers(text):
    """Numéros déjà présents (tableau principal + archive)."""
    matches = (ROW_RE.match(line) for line in text.splitlines())
    return {int(m.group(1)) for m in matches if m}


def title_from_dirname(dirname):
    """Titre de secours quand l'API LeetCode est injoignable."""
    words = dirname.rsplit("_", 1)[0].split("_")
    return " ".join(w.capitalize() for w in words)


def render_row(number, title, url, difficulty, technique, perf):
    problem = f"[{title}]({url})" if url else title
    return f"| {number} | {problem} | {difficulty} | {technique} | {perf} |"


def update_readme(text, new_rows):
    """Insère new_rows (déjà triées, plus récent en premier) en haut du tableau.

    Applique le cap de 30 lignes : l'excédent bascule en haut de l'archive.
    Ne touche à rien d'autre — les lignes existantes sont déplacées, jamais
    réécrites.
    """
    lines = text.splitlines()
    sep_idx = lines.index(TABLE_SEP)
    j = sep_idx + 1
    while j < len(lines) and ROW_RE.match(lines[j]):
        j += 1
    main_rows = new_rows + lines[sep_idx + 1 : j]
    overflow = main_rows[MAX_VISIBLE_ROWS:]
    main_rows = main_rows[:MAX_VISIBLE_ROWS]

    tail = lines[j:]
    close_idx = tail.index("</details>")
    archive_rows = overflow + [l for l in tail[:close_idx] if ROW_RE.match(l)]
    if archive_rows:
        summary_idx = next(i for i, l in enumerate(tail) if "<summary>" in l)
        tail = (tail[: summary_idx + 1]
                + ["", TABLE_HEADER, TABLE_SEP, *archive_rows, ""]
                + tail[close_idx:])

    new_text = "\n".join(lines[: sep_idx + 1] + main_rows + tail)
    if text.endswith("\n") and not new_text.endswith("\n"):
        new_text += "\n"
    total = len(main_rows) + len(archive_rows)
    return COUNT_RE.sub(f"## Solutions — {total} solved", new_text)


LEETCODE_API = "https://leetcode.com/api/problems/all/"
DIFFICULTIES = {1: "🟢 Easy", 2: "🟡 Medium", 3: "🔴 Hard"}


def problem_commits(root):
    """Numéro → (timestamp, sujet) du commit le plus récent « <numéro> - … »."""
    out = subprocess.run(
        ["git", "-C", str(root), "log", "--pretty=%at\t%s"],
        capture_output=True, text=True, check=True,
    ).stdout
    commits = {}
    for line in out.splitlines():
        ts, _, subject = line.partition("\t")
        parsed = parse_commit_subject(subject)
        if parsed and parsed[0] not in commits:  # le log est du plus récent au plus ancien
            commits[parsed[0]] = (int(ts), subject)
    return commits


def fetch_leetcode_index():
    """Numéro → (titre officiel, url, difficulté). {} si l'API est injoignable."""
    try:
        req = urllib.request.Request(LEETCODE_API, headers={"User-Agent": "Mozilla/5.0"})
        with urllib.request.urlopen(req, timeout=30) as resp:
            data = json.load(resp)
        index = {}
        for pair in data.get("stat_status_pairs", []):
            stat = pair["stat"]
            index[int(stat["frontend_question_id"])] = (
                stat["question__title"],
                f"https://leetcode.com/problems/{stat['question__title_slug']}/",
                DIFFICULTIES.get(pair["difficulty"]["level"], "—"),
            )
        return index
    except Exception as exc:  # dégradation volontaire, jamais bloquant
        print(f"API LeetCode injoignable ({exc}) — titres dérivés des dossiers.",
              file=sys.stderr)
        return {}


def build_new_rows(root, readme_text):
    """Lignes à insérer, triées du plus récemment commité au plus ancien."""
    dirs = {}
    for child in sorted(root.iterdir()):
        if child.is_dir():
            num = extract_problem_number(child.name)
            if num is not None:
                dirs[num] = child.name
    missing = sorted(set(dirs) - readme_numbers(readme_text))
    if not missing:
        return []
    commits = problem_commits(root)
    index = fetch_leetcode_index()
    entries = []
    for num in missing:
        ts, subject = commits.get(num, (0, None))
        technique, perf = ("—", "—")
        if subject is not None:
            _, technique, perf = parse_commit_subject(subject)
        title, url, difficulty = index.get(num) or (
            title_from_dirname(dirs[num]), None, "—")
        entries.append((ts, render_row(num, title, url, difficulty, technique, perf)))
    entries.sort(key=lambda e: e[0], reverse=True)
    return [r for _, r in entries]


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path,
                        default=Path(__file__).resolve().parent.parent)
    parser.add_argument("--dry-run", action="store_true",
                        help="affiche le diff sans écrire")
    args = parser.parse_args(argv)

    readme_path = args.root / "README.md"
    old = readme_path.read_text(encoding="utf-8")
    new_rows = build_new_rows(args.root, old)
    if not new_rows:
        print("README à jour — aucun nouveau problème.")
        return 0
    new = update_readme(old, new_rows)
    if args.dry_run:
        sys.stdout.writelines(difflib.unified_diff(
            old.splitlines(keepends=True), new.splitlines(keepends=True),
            "README.md (avant)", "README.md (après)"))
    else:
        readme_path.write_text(new, encoding="utf-8")
        print(f"README mis à jour : {len(new_rows)} nouvelle(s) ligne(s).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
