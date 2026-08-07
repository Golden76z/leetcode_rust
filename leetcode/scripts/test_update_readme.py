import io
import unittest
import pathlib
import subprocess
import tempfile
import unittest.mock

from update_readme import (
    extract_problem_number,
    parse_commit_subject,
    readme_numbers,
    title_from_dirname,
    render_row,
    update_readme,
    TABLE_HEADER,
    TABLE_SEP,
    MAX_VISIBLE_ROWS,
    problem_commits,
    build_new_rows,
    fetch_leetcode_index,
)


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
            commits = problem_commits(pathlib.Path(tmp))
            self.assertEqual(set(commits), {1})
            _, subject = commits[1]
            self.assertEqual(subject, "1 - Two sum - Hashmap (0ms - 2.34mb)")


class BuildNewRowsTest(unittest.TestCase):
    def _fake_root(self, tmp, dirs):
        for d in dirs:
            (pathlib.Path(tmp) / d).mkdir()
        return pathlib.Path(tmp)

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


class FetchLeetcodeIndexTest(unittest.TestCase):
    def test_schema_inattendu_degrade_en_index_vide(self):
        mock_response = unittest.mock.MagicMock()
        mock_response.__enter__ = unittest.mock.MagicMock(return_value=mock_response)
        mock_response.__exit__ = unittest.mock.MagicMock(return_value=None)

        with unittest.mock.patch("update_readme.urllib.request.urlopen", return_value=mock_response), \
             unittest.mock.patch("update_readme.json.load", return_value={"stat_status_pairs": [{"pas_stat": 1}]}), \
             unittest.mock.patch("update_readme.sys.stderr", new_callable=io.StringIO):
            result = fetch_leetcode_index()
        self.assertEqual(result, {})


if __name__ == "__main__":
    unittest.main()
