# README Automation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** GitHub Action déclenchée manuellement qui régénère le tableau des solutions du README (nouvelles lignes depuis les commits + API LeetCode, cap 30 lignes avec archive) et commit sur main.

**Architecture:** Un script Python stdlib (`scripts/update_readme.py`) en fonctions pures, testé par `unittest`. Le README est la source de vérité : le script n'ajoute que les lignes manquantes et ne réécrit jamais les lignes existantes. Un workflow `workflow_dispatch` exécute le script et commit s'il y a un diff.

**Tech Stack:** Python 3 stdlib uniquement (re, subprocess, urllib, json, difflib, argparse, unittest), GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-08-06-readme-automation-design.md`

## Global Constraints

- Python 3 stdlib uniquement — aucune dépendance pip.
- Le script ne modifie jamais une ligne existante du README ; il insère et déplace seulement.
- Toute défaillance (commit non parsable, API LeetCode down) dégrade en `—` / titre dérivé du dossier — le run ne casse jamais.
- Cap de 30 lignes dans le tableau principal ; l'excédent (le plus ancien) bascule dans le bloc `<details>` d'archive.
- **Convention utilisateur : UN SEUL commit pour tout le plan, à la fin (Task 5), message en français conventional-commit. Ignorer toute envie de committer par tâche.**
- Messages/­docstrings du script en français ; contenu généré du README en anglais (cohérent avec l'existant).

---

### Task 1: Parsing des sujets de commit et des noms de dossiers

**Files:**
- Create: `scripts/update_readme.py`
- Create: `scripts/test_update_readme.py`

**Interfaces:**
- Produces: `extract_problem_number(dirname: str) -> int | None` ; `parse_commit_subject(subject: str) -> tuple[int, str, str] | None` (retourne `(numéro, technique, perf)`, `perf` au format `"0ms / 4.38mb"` ou `"—"`) ; constantes `PROBLEM_DIR_RE`, `PERF_RE`.

- [ ] **Step 1: Écrire les tests qui échouent**

Créer `scripts/test_update_readme.py` :

```python
import unittest

from update_readme import extract_problem_number, parse_commit_subject


class ExtractProblemNumberTest(unittest.TestCase):
    def test_dossier_standard(self):
        self.assertEqual(extract_problem_number("daily_temperatures_739"), 739)
        self.assertEqual(extract_problem_number("two_sum_1"), 1)

    def test_dossier_hors_probleme(self):
        self.assertIsNone(extract_problem_number("docs"))
        self.assertIsNone(extract_problem_number("scripts"))
        self.assertIsNone(extract_problem_number(".github"))


class ParseCommitSubjectTest(unittest.TestCase):
    def test_format_complet(self):
        self.assertEqual(
            parse_commit_subject("739 - Daily temperatures - Stack (0ms - 4.38mb)"),
            (739, "Stack", "0ms / 4.38mb"),
        )

    def test_sans_perf(self):
        self.assertEqual(
            parse_commit_subject("383 - Ransom note - HashMap"),
            (383, "HashMap", "—"),
        )

    def test_technique_longue_avec_perf(self):
        self.assertEqual(
            parse_commit_subject(
                "49 - Group anagrams - HashMap with Vec of indexes as key, index as value (4ms - 5.36mb)"
            ),
            (49, "HashMap with Vec of indexes as key, index as value", "4ms / 5.36mb"),
        )

    def test_sujet_sans_numero(self):
        self.assertIsNone(parse_commit_subject("Update README.md"))
        self.assertIsNone(parse_commit_subject("Fresh start"))

    def test_multi_problemes_ignore(self):
        # « 344 & 977 - ... » : pas un numéro simple → ignoré
        self.assertIsNone(parse_commit_subject("344 & 977 - Two pointers technique"))

    def test_deux_segments_seulement(self):
        # « 56 - 54 » : numéro valide mais pas de segment technique
        self.assertEqual(parse_commit_subject("56 - 54"), (56, "—", "—"))


if __name__ == "__main__":
    unittest.main()
```

- [ ] **Step 2: Vérifier que les tests échouent**

Run: `cd scripts && python3 -m unittest test_update_readme -v`
Expected: erreur d'import (`No module named 'update_readme'`).

- [ ] **Step 3: Implémentation minimale**

Créer `scripts/update_readme.py` :

```python
#!/usr/bin/env python3
"""Régénère le tableau des solutions du README.

Détecte les dossiers de problèmes absents du tableau, retrouve leurs
métadonnées (message de commit + API LeetCode) et insère les lignes
manquantes en haut du tableau ; au-delà de 30 lignes, les plus
anciennes basculent dans l'archive.
"""

import re

PROBLEM_DIR_RE = re.compile(r"^[a-z0-9_]+_(\d+)$")
PERF_RE = re.compile(r"\((\d+ms) - ([\d.]+mb)\)\s*$", re.IGNORECASE)


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
```

- [ ] **Step 4: Vérifier que les tests passent**

Run: `cd scripts && python3 -m unittest test_update_readme -v`
Expected: tous PASS.

---

### Task 2: Lecture du README et rendu des lignes

**Files:**
- Modify: `scripts/update_readme.py`
- Modify: `scripts/test_update_readme.py`

**Interfaces:**
- Consumes: rien de nouveau (module de la Task 1).
- Produces: `readme_numbers(text: str) -> set[int]` ; `title_from_dirname(dirname: str) -> str` ; `render_row(number, title, url, difficulty, technique, perf) -> str` (`url=None` → titre en texte brut) ; constantes `TABLE_HEADER`, `TABLE_SEP`, `ROW_RE`.

- [ ] **Step 1: Ajouter les tests qui échouent**

Ajouter à `scripts/test_update_readme.py` (imports en haut : `readme_numbers, title_from_dirname, render_row, TABLE_HEADER, TABLE_SEP`) :

```python
class TitleFromDirnameTest(unittest.TestCase):
    def test_titre_depuis_dossier(self):
        self.assertEqual(title_from_dirname("daily_temperatures_739"), "Daily Temperatures")
        self.assertEqual(title_from_dirname("two_sum_1"), "Two Sum")


class RenderRowTest(unittest.TestCase):
    def test_ligne_complete(self):
        self.assertEqual(
            render_row(739, "Daily Temperatures",
                       "https://leetcode.com/problems/daily-temperatures/",
                       "🟡 Medium", "Stack", "0ms / 4.38mb"),
            "| 739 | [Daily Temperatures](https://leetcode.com/problems/daily-temperatures/)"
            " | 🟡 Medium | Stack | 0ms / 4.38mb |",
        )

    def test_sans_url(self):
        self.assertEqual(
            render_row(739, "Daily Temperatures", None, "—", "—", "—"),
            "| 739 | Daily Temperatures | — | — | — |",
        )


class ReadmeNumbersTest(unittest.TestCase):
    def test_numeros_du_tableau_et_de_larchive(self):
        text = "\n".join([
            "## Solutions — 3 solved", "", TABLE_HEADER, TABLE_SEP,
            "| 682 | [Baseball Game](https://x/) | 🟢 Easy | Stack | 0ms / 2.33mb |",
            "| 42 | [Trapping Rain Water](https://x/) | 🔴 Hard | Two pointers | — |",
            "", "<details>", "<summary>📁 Older solutions</summary>", "",
            TABLE_HEADER, TABLE_SEP,
            "| 13 | [Roman to Integer](https://x/) | 🟢 Easy | Value mapping | 1ms |",
            "", "</details>", "",
        ])
        self.assertEqual(readme_numbers(text), {682, 42, 13})
```

- [ ] **Step 2: Vérifier l'échec**

Run: `cd scripts && python3 -m unittest test_update_readme -v`
Expected: `ImportError` sur les nouveaux noms.

- [ ] **Step 3: Implémentation**

Ajouter à `scripts/update_readme.py` :

```python
TABLE_HEADER = "| # | Problem | Difficulty | Technique | Runtime / Memory |"
TABLE_SEP = "|---:|---------|:---:|-----------|:---:|"
ROW_RE = re.compile(r"^\| (\d+) \|")


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
```

- [ ] **Step 4: Vérifier que tout passe**

Run: `cd scripts && python3 -m unittest test_update_readme -v`
Expected: tous PASS.

---

### Task 3: Mise à jour du README (insertion, cap 30, archive, compteur)

**Files:**
- Modify: `scripts/update_readme.py`
- Modify: `scripts/test_update_readme.py`

**Interfaces:**
- Consumes: `TABLE_HEADER`, `TABLE_SEP`, `ROW_RE` (Task 2).
- Produces: `update_readme(text: str, new_rows: list[str]) -> str` ; constantes `MAX_VISIBLE_ROWS = 30`, `COUNT_RE`.

- [ ] **Step 1: Ajouter les tests qui échouent**

Ajouter à `scripts/test_update_readme.py` (importer aussi `update_readme`, `MAX_VISIBLE_ROWS`) :

```python
PLACEHOLDER = "_Nothing here yet — everything still fits in the table above._"


def row(n):
    return f"| {n} | [P{n}](https://leetcode.com/problems/p{n}/) | 🟢 Easy | T | — |"


def build_readme(main_rows, archive_rows=()):
    lines = [
        "# Test repo", "",
        f"## Solutions — {len(main_rows) + len(archive_rows)} solved", "",
        TABLE_HEADER, TABLE_SEP, *main_rows, "",
        "<!-- Keep the table above at max 30 rows. -->",
        "<details>", "<summary>📁 Older solutions</summary>", "",
    ]
    if archive_rows:
        lines += [TABLE_HEADER, TABLE_SEP, *archive_rows, ""]
    else:
        lines += [PLACEHOLDER, ""]
    lines += ["</details>", ""]
    return "\n".join(lines)


class UpdateReadmeTest(unittest.TestCase):
    def test_insertion_en_haut_et_compteur(self):
        text = build_readme([row(2), row(1)])
        result = update_readme(text, [row(3)])
        self.assertEqual(result, build_readme([row(3), row(2), row(1)]))

    def test_sans_nouvelle_ligne_texte_inchange(self):
        text = build_readme([row(2), row(1)], [row(0)])
        self.assertEqual(update_readme(text, []), text)

    def test_debordement_cree_larchive(self):
        main = [row(n) for n in range(MAX_VISIBLE_ROWS, 0, -1)]  # 30 lignes
        text = build_readme(main)
        result = update_readme(text, [row(99)])
        expected = build_readme([row(99)] + main[:-1], [row(1)])
        self.assertEqual(result, expected)

    def test_debordement_vers_archive_existante(self):
        main = [row(n) for n in range(MAX_VISIBLE_ROWS + 1, 1, -1)]  # 31..2
        text = build_readme(main, [row(1)])
        result = update_readme(text, [row(99)])
        expected = build_readme([row(99)] + main[:-1], [row(2), row(1)])
        self.assertEqual(result, expected)

    def test_compteur_total(self):
        text = build_readme([row(2), row(1)])
        result = update_readme(text, [row(3)])
        self.assertIn("## Solutions — 3 solved", result)
```

- [ ] **Step 2: Vérifier l'échec**

Run: `cd scripts && python3 -m unittest test_update_readme -v`
Expected: `ImportError` sur `update_readme` / `MAX_VISIBLE_ROWS`.

- [ ] **Step 3: Implémentation**

Ajouter à `scripts/update_readme.py` :

```python
MAX_VISIBLE_ROWS = 30
COUNT_RE = re.compile(r"^## Solutions — \d+ solved$", re.MULTILINE)


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
```

- [ ] **Step 4: Vérifier que tout passe**

Run: `cd scripts && python3 -m unittest test_update_readme -v`
Expected: tous PASS.

---

### Task 4: Intégration git + API LeetCode + main()

**Files:**
- Modify: `scripts/update_readme.py`
- Modify: `scripts/test_update_readme.py`

**Interfaces:**
- Consumes: tout ce qui précède.
- Produces: `problem_commits(root: Path) -> dict[int, tuple[int, str]]` (numéro → `(timestamp, sujet)` du commit le plus récent) ; `fetch_leetcode_index() -> dict[int, tuple[str, str, str]]` (numéro → `(titre, url, difficulté)`, `{}` si API down) ; `build_new_rows(root: Path, readme_text: str) -> list[str]` ; `main(argv=None) -> int` avec `--root` et `--dry-run`.

- [ ] **Step 1: Ajouter les tests qui échouent**

Ajouter à `scripts/test_update_readme.py` (imports : `problem_commits, build_new_rows` + `pathlib.Path`, `subprocess`, `tempfile`, `unittest.mock`) :

```python
class ProblemCommitsTest(unittest.TestCase):
    def test_prend_le_commit_le_plus_recent_par_numero(self):
        with tempfile.TemporaryDirectory() as tmp:
            def git(*args):
                subprocess.run(
                    ["git", "-C", tmp, "-c", "user.name=t", "-c", "user.email=t@t",
                     *args],
                    check=True, capture_output=True,
                )
            git("init")
            git("commit", "--allow-empty", "-m", "1 - Two sum - Brute force")
            git("commit", "--allow-empty", "-m", "Update README.md")
            git("commit", "--allow-empty", "-m", "1 - Two sum - Hashmap (0ms - 2.34mb)")
            commits = problem_commits(Path(tmp))
            self.assertEqual(set(commits), {1})
            _, subject = commits[1]
            self.assertEqual(subject, "1 - Two sum - Hashmap (0ms - 2.34mb)")


class BuildNewRowsTest(unittest.TestCase):
    def _fake_root(self, tmp, dirs):
        for d in dirs:
            (Path(tmp) / d).mkdir()
        return Path(tmp)

    def test_construit_les_lignes_manquantes(self):
        readme = build_readme([row(1)])
        with tempfile.TemporaryDirectory() as tmp:
            root = self._fake_root(tmp, ["two_sum_1", "daily_temperatures_739", "docs"])
            with unittest.mock.patch(
                "update_readme.problem_commits",
                return_value={739: (100, "739 - Daily temperatures - Stack (0ms - 4.38mb)")},
            ), unittest.mock.patch(
                "update_readme.fetch_leetcode_index",
                return_value={739: ("Daily Temperatures",
                                    "https://leetcode.com/problems/daily-temperatures/",
                                    "🟡 Medium")},
            ):
                rows = build_new_rows(root, readme)
        self.assertEqual(rows, [
            "| 739 | [Daily Temperatures](https://leetcode.com/problems/daily-temperatures/)"
            " | 🟡 Medium | Stack | 0ms / 4.38mb |",
        ])

    def test_degradation_sans_commit_ni_api(self):
        readme = build_readme([row(1)])
        with tempfile.TemporaryDirectory() as tmp:
            root = self._fake_root(tmp, ["two_sum_1", "daily_temperatures_739"])
            with unittest.mock.patch(
                "update_readme.problem_commits", return_value={}
            ), unittest.mock.patch(
                "update_readme.fetch_leetcode_index", return_value={}
            ):
                rows = build_new_rows(root, readme)
        self.assertEqual(rows, ["| 739 | Daily Temperatures | — | — | — |"])

    def test_rien_a_faire(self):
        readme = build_readme([row(1)])
        with tempfile.TemporaryDirectory() as tmp:
            root = self._fake_root(tmp, ["two_sum_1"])
            rows = build_new_rows(root, readme)
        self.assertEqual(rows, [])
```

- [ ] **Step 2: Vérifier l'échec**

Run: `cd scripts && python3 -m unittest test_update_readme -v`
Expected: `ImportError` sur les nouveaux noms.

- [ ] **Step 3: Implémentation**

Compléter `scripts/update_readme.py` — imports en tête de fichier :

```python
import argparse
import difflib
import json
import subprocess
import sys
import urllib.request
from pathlib import Path
```

puis les fonctions :

```python
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
```

- [ ] **Step 4: Vérifier que tout passe**

Run: `cd scripts && python3 -m unittest test_update_readme -v`
Expected: tous PASS (y compris les tests des tâches précédentes).

---

### Task 5: Workflow GitHub Actions, vérification bout en bout, commit unique

**Files:**
- Create: `.github/workflows/update-readme.yml`
- Modify: `.gitignore` (ajouter `__pycache__/` si absent)
- Modify: `README.md` (effet du premier vrai run : ligne 739)

**Interfaces:**
- Consumes: `scripts/update_readme.py` complet (Tasks 1–4).
- Produces: le livrable final.

- [ ] **Step 1: Écrire le workflow**

Créer `.github/workflows/update-readme.yml` :

```yaml
name: Update README

on:
  workflow_dispatch:

permissions:
  contents: write

jobs:
  update:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # le script lit tout l'historique git

      - name: Régénérer le README
        run: python3 scripts/update_readme.py

      - name: Commit et push si diff
        run: |
          if git diff --quiet README.md; then
            echo "README à jour — rien à committer."
          else
            git config user.name "github-actions[bot]"
            git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
            git add README.md
            git commit -m "docs(readme): mise à jour du tableau des solutions"
            git push
          fi
```

- [ ] **Step 2: Vérifier le YAML et le .gitignore**

Run: `python3 -c "import yaml,sys; yaml.safe_load(open('.github/workflows/update-readme.yml'))" 2>/dev/null || echo "pas de PyYAML — vérifier à l'œil"` et `grep -q __pycache__ .gitignore || echo "à ajouter"`.
Ajouter `__pycache__/` au `.gitignore` si absent.

- [ ] **Step 3: Vérification bout en bout en local**

Run: `python3 scripts/update_readme.py --dry-run`
Expected: un diff unified montrant la ligne `| 739 | [Daily Temperatures](…) | 🟡 Medium | Stack | 0ms / 4.38mb |` insérée en haut du tableau et `## Solutions — 30 solved`. Rien d'autre ne doit bouger dans le diff.

- [ ] **Step 4: Vrai run local**

Run: `python3 scripts/update_readme.py` puis `git diff README.md`
Expected: mêmes changements que le dry-run, écrits dans le fichier.

- [ ] **Step 5: Suite complète une dernière fois**

Run: `cd scripts && python3 -m unittest test_update_readme -v`
Expected: tous PASS.

- [ ] **Step 6: Commit unique du plan**

```bash
git add scripts/ .github/ .gitignore README.md docs/superpowers/plans/2026-08-06-readme-automation.md
git commit -m "feat(readme): action de mise à jour automatique du tableau des solutions

Script Python stdlib (scripts/update_readme.py) : détection des dossiers
de problèmes absents du tableau, métadonnées depuis les messages de
commit et l'API LeetCode, cap de 30 lignes avec bascule en archive.
Workflow workflow_dispatch qui commit le README régénéré sur main.

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

- [ ] **Step 7: Pousser et tester le workflow réel**

```bash
git push
gh workflow run update-readme.yml
gh run watch
```

Expected: le run se termine en succès avec « README à jour — rien à committer » (la ligne 739 ayant déjà été ajoutée en local à l'étape 4).
