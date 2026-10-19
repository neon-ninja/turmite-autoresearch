"""Small publishing fixtures: saved outputs, paths, provenance, and repeatability."""
import contextlib
import hashlib
import io
import json
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import nbformat

import experiments


class PublishingTest(unittest.TestCase):
    def test_saved_outputs_links_and_repeatability(self):
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            blog = root / "blog"
            blog.mkdir()
            first = root / "experiments/2026-10-19-first"
            second = root / "experiments/2026-10-19-second"
            first.mkdir(parents=True)
            second.mkdir()
            (first / "README.md").write_text(
                "# First experiment\n\n[Jump](#first-experiment)\n\n[Other](../2026-10-19-second/README.md)\n\n"
                "[Code](script.py)\n\n![Figure](plot.svg)\n\n[Missing](gone.csv)\n")
            (first / "script.py").write_text("print('source')\n")
            (first / "plot.svg").write_text('<svg xmlns="http://www.w3.org/2000/svg"/>')
            (second / "README.md").write_text("# No notebook yet\n")
            (first / "NOTEBOOK.md").write_text("# Dated log\n\nObservation.\n")
            nested = first / "notebooks"
            nested.mkdir()
            notebook = nbformat.v4.new_notebook(cells=[
                nbformat.v4.new_markdown_cell("# Saved narrative"),
                nbformat.v4.new_code_cell("raise RuntimeError('must not execute')", outputs=[
                    nbformat.v4.new_output("stream", name="stdout", text="SAVED_OUTPUT_42\n"),
                    nbformat.v4.new_output("error", ename="ValueError", evalue="saved error", traceback=["saved traceback"]),
                ]),
            ])
            # Exercise the older notebook format without modifying its on-disk IDs.
            for cell in notebook.cells:
                cell.pop("id", None)
            notebook_path = nested / "detail.ipynb"
            notebook_path.write_text(json.dumps(notebook), encoding="utf-8-sig")
            original = notebook_path.read_bytes()
            internal = root / "experiments/_template"
            internal.mkdir()
            (internal / "README.md").write_text("# Not an experiment")
            for args in (["init", "-q"], ["add", "."],
                         ["-c", "user.name=Fixture", "-c", "user.email=fixture@example.test", "commit", "-qm", "Fixture"]):
                subprocess.run(["git", *args], cwd=root, check=True)
            (first / "untracked.ipynb").write_text("invalid untracked notebook")

            with patch.multiple(experiments, ROOT=root, BLOG=blog, OUTPUT=blog / "experiments"):
                def render():
                    with contextlib.redirect_stdout(io.StringIO()):
                        experiments.build("/project", "https://github.com/example/project")
                    return {p.relative_to(blog).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
                            for p in blog.rglob("*") if p.is_file()}

                snapshot = render()
                self.assertEqual(snapshot, render())
                self.assertEqual(notebook_path.read_bytes(), original)
                catalog = json.loads((blog / "_data/experiments.json").read_text())
                self.assertEqual(len(catalog), 2)
                first_page = (blog / "experiments" / first.name / "index.html").read_text()
                second_page = (blog / "experiments" / second.name / "index.html").read_text()
                self.assertIn("SAVED_OUTPUT_42", first_page)
                self.assertIn('id="first-experiment"', first_page)
                self.assertIn('href="#first-experiment"', first_page)
                self.assertIn("saved traceback", first_page)
                self.assertIn('<details class="notebook-code">', first_page)
                self.assertIn("/project/experiments/2026-10-19-second/", first_page)
                self.assertIn("/assets/experiments/2026-10-19-first/plot.svg", first_page)
                self.assertIn("https://github.com/example/project/blob/", first_page)
                self.assertIn("No committed Jupyter notebook", second_page)
                manifest = json.loads((blog / "experiments/manifest.json").read_text())
                self.assertEqual(manifest["missing_links"][0]["target"], "gone.csv")
                self.assertEqual(len(catalog[1]["notebooks"]), 1)

    def test_unmarked_directory_is_preserved(self):
        with tempfile.TemporaryDirectory() as scratch:
            output = Path(scratch) / "blog/experiments"
            output.mkdir(parents=True)
            sentinel = output / "keep.txt"
            sentinel.write_text("existing file")
            with patch.multiple(experiments, ROOT=Path(scratch), BLOG=output.parent, OUTPUT=output), \
                    patch.object(experiments, "git", return_value=""):
                with self.assertRaisesRegex(RuntimeError, "unmarked"):
                    experiments.build("", "")
            self.assertEqual(sentinel.read_text(), "existing file")


if __name__ == "__main__":
    unittest.main()
