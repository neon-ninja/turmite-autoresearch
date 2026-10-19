"""Publish tracked experiment documents and saved notebook outputs; never execute cells."""
from __future__ import annotations

import argparse
import hashlib
import html
import json
import re
import shutil
import subprocess
from pathlib import Path
from urllib.parse import quote, unquote, urlsplit

import nbformat
from bs4 import BeautifulSoup
from nbconvert import HTMLExporter

ROOT = Path(__file__).resolve().parents[2]
BLOG = ROOT / "blog"
OUTPUT = BLOG / "experiments"


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def notebook_anchor(path: Path, experiment: Path) -> str:
    return "notebook-" + path.relative_to(experiment).as_posix().replace("/", "-").replace(".", "-")


def heading_slug(text: str) -> str:
    return re.sub(r"[^\w -]", "", unquote(text).lower()).replace(" ", "-")


def build(baseurl: str, repository: str) -> None:
    tracked = {ROOT / path for path in git("ls-files").splitlines()}
    readmes = sorted((p for p in tracked if p.parent.parent == ROOT / "experiments"
                      and p.name == "README.md" and not p.parent.name.startswith("_")), reverse=True)
    experiments = {p.parent for p in readmes}
    commit = git("rev-parse", "HEAD")
    # This fixed directory contains generated website files, never research data.
    assert OUTPUT.resolve() == ROOT.resolve() / "blog" / "experiments"
    marker = OUTPUT / ".generated-by-turmite-blog"
    if OUTPUT.exists():
        if not marker.is_file():
            raise RuntimeError(f"Refusing to replace an unmarked directory: {OUTPUT}")
        shutil.rmtree(OUTPUT)
    OUTPUT.mkdir()
    marker.touch()
    exporter = HTMLExporter(template_name="basic")
    exporter.exclude_anchor_links = True
    catalog = []
    manifest = {"commit": commit, "inputs": {}, "missing_links": []}

    def site_url(path: str) -> str:
        return baseurl + "/experiments/" + path

    def repo_url(path: Path, fragment: str = "") -> str:
        return repository + "/blob/" + commit + "/" + quote(path.relative_to(ROOT).as_posix()) + fragment

    def rewrite(fragment: str, source: Path, experiment: Path) -> str:
        soup = BeautifulSoup(fragment, "html.parser")
        namespace = notebook_anchor(source, experiment) + "-" if source.suffix == ".ipynb" else "log-" if source.name == "NOTEBOOK.md" else ""
        heading_counts = {}
        for heading in soup.find_all(re.compile(r"^h[1-6]$")):
            slug = heading_slug(heading.get_text())
            count = heading_counts.get(slug, 0)
            heading_counts[slug] = count + 1
            heading["id"] = namespace + slug + (f"-{count}" if count else "")
        for element in soup.find_all(["a", "img", "source"]):
            attr = "href" if element.name == "a" else "src"
            url = element.get(attr)
            if not url or url.startswith("//"):
                continue
            if url.startswith("#"):
                element[attr] = "#" + namespace + heading_slug(url[1:])
                continue
            parsed = urlsplit(url)
            if parsed.scheme:
                continue
            target = (source.parent / unquote(parsed.path)).resolve()
            suffix = "#" + heading_slug(parsed.fragment) if parsed.fragment else ""
            parent_experiment = next((p for p in [target, *target.parents] if p in experiments), None)
            if target.name == "README.md" and target.parent in experiments:
                element[attr] = site_url(target.parent.name + "/") + suffix
            elif target.suffix == ".ipynb" and target in tracked and parent_experiment:
                anchor = notebook_anchor(target, parent_experiment)
                if parsed.fragment:
                    anchor += "-" + heading_slug(parsed.fragment)
                element[attr] = site_url(parent_experiment.name + "/") + "#" + anchor
            elif attr == "src" and target in tracked and target.is_file():
                # Copy only referenced, committed images, preserving their repository path.
                relative = target.relative_to(ROOT)
                dest = OUTPUT / experiment.name / "assets" / relative
                dest.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(target, dest)
                manifest["inputs"][relative.as_posix()] = digest(target)
                element[attr] = site_url(experiment.name + "/assets/" + quote(relative.as_posix()))
            elif target in tracked or (target.is_dir() and ROOT in target.parents):
                if repository:
                    link = repo_url(target, suffix)
                    if target.is_dir():
                        link = link.replace("/blob/", "/tree/", 1)
                    element[attr] = link
                else:
                    del element[attr]
                    element["title"] = "Repository source: " + target.relative_to(ROOT).as_posix()
                    element["class"] = ["source-reference"]
            else:
                # Older READMEs contain migration-era links; keep their label, not a 404.
                manifest["missing_links"].append({"source": source.relative_to(ROOT).as_posix(), "target": url})
                del element[attr]
                element["title"] = "Source reference unavailable: " + url
                element["class"] = ["source-reference"]
        for code_input in soup.select(".input"):
            details = soup.new_tag("details", attrs={"class": "notebook-code"})
            summary = soup.new_tag("summary")
            summary.string = "Code"
            code_input.wrap(details)
            details.insert(0, summary)
        return str(soup)

    def render_markdown(path: Path, experiment: Path) -> str:
        manifest["inputs"][path.relative_to(ROOT).as_posix()] = digest(path)
        notebook = nbformat.v4.new_notebook(cells=[nbformat.v4.new_markdown_cell(path.read_text(), id="document")])
        notebook.metadata.language_info = {"name": "python", "pygments_lexer": "python3"}
        body, _ = exporter.from_notebook_node(notebook, resources={"metadata": {"path": str(path.parent)}})
        return rewrite(body, path, experiment)

    for readme in readmes:
        experiment = readme.parent
        notebooks = sorted(p for p in tracked if p.suffix == ".ipynb"
                           and (p.parent == experiment or experiment / "notebooks" in p.parents))
        text = readme.read_text()
        title_match = re.search(r"^#\s+(.+)$", text, re.MULTILINE)
        title = title_match.group(1).replace("`", "") if title_match else experiment.name
        overview = render_markdown(readme, experiment)
        excerpt = BeautifulSoup(overview, "html.parser").find("p")
        description = re.sub(r"\s+", " ", excerpt.get_text(" ", strip=True))[:250] if excerpt else "Experiment record."
        sections = ['<section id="overview" class="prose experiment-overview">' + overview + "</section>"]
        notebook_info = []
        for path in notebooks:
            manifest["inputs"][path.relative_to(ROOT).as_posix()] = digest(path)
            data = json.loads(path.read_text())
            # Older notebooks lack cell IDs. Assign stable IDs in memory only;
            # nbformat otherwise introduces random IDs when repairing them.
            for index, cell in enumerate(data.get("cells", [])):
                cell.setdefault("id", f"cell-{index}")
            notebook = nbformat.reads(json.dumps(data), as_version=4)
            language = notebook.metadata.setdefault("language_info", {})
            language.setdefault("name", "python")
            if language.get("name", "python").startswith("python"):
                # Static syntax highlighting does not need an interactive kernel.
                language["pygments_lexer"] = "python3"
            code_cells = [c for c in notebook.cells if c.cell_type == "code"]
            outputs = sum(len(c.get("outputs", [])) for c in code_cells)
            errors = sum(o.output_type == "error" for c in code_cells for o in c.get("outputs", []))
            body, _ = exporter.from_notebook_node(notebook, resources={"metadata": {"path": str(path.parent)}})
            anchor = notebook_anchor(path, experiment)
            name = path.relative_to(experiment).as_posix()
            download = experiment.name + "/notebooks/" + name
            dest = OUTPUT / download
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(path, dest)
            message = f"{outputs} saved outputs · {len(code_cells)} code cells. Cells are not executed during publication."
            if not outputs:
                message = "No saved outputs. This render shows the saved narrative and code; cells have not been executed for publication."
            if errors:
                message += f" {errors} saved error outputs are preserved below."
            sections.append(f'<section id="{anchor}" class="notebook-section"><h2>{html.escape(name)}</h2>'
                            f'<p class="render-note">{message} '
                            f'<a href="{site_url(download)}" download>Download notebook</a></p>'
                            f'<div class="notebook-render">{rewrite(body, path, experiment)}</div></section>')
            notebook_info.append({"name": name, "anchor": anchor, "outputs": outputs, "errors": errors})
        if not notebooks:
            sections.append('<section class="notebook-section"><h2>Notebook</h2><p class="render-note">'
                            'No committed Jupyter notebook is available for this experiment yet.</p></section>')
        log = experiment / "NOTEBOOK.md"
        if log in tracked:
            sections.append('<details class="lab-log"><summary>Read the dated lab log</summary>'
                            '<div class="prose">' + render_markdown(log, experiment) + '</div></details>')
        metadata = {"layout": "experiment", "title": title, "description": description,
                    "slug": experiment.name, "source_commit": commit, "notebooks": notebook_info,
                    "source_url": repo_url(readme) if repository else ""}
        destination = OUTPUT / experiment.name / "index.html"
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text("---\n" + json.dumps(metadata, ensure_ascii=False) + "\n---\n{% raw %}\n"
                               + "\n".join(sections) + "\n{% endraw %}\n")
        catalog.append({**metadata, "url": "/experiments/" + experiment.name + "/"})
    data_dir = BLOG / "_data"
    data_dir.mkdir(exist_ok=True)
    (data_dir / "experiments.json").write_text(json.dumps(catalog, ensure_ascii=False, indent=2) + "\n")
    (OUTPUT / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, sort_keys=True, indent=2) + "\n")
    print(f"Published {len(catalog)} experiment pages, {sum(len(p['notebooks']) for p in catalog)} notebooks; "
          f"{len(manifest['missing_links'])} unavailable source references recorded in experiments/manifest.json.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseurl", default="", help="Pages project path, e.g. /turmite")
    parser.add_argument("--repository-url", default="", help="GitHub repository URL for pinned source links")
    args = parser.parse_args()
    build(args.baseurl.rstrip("/"), args.repository_url.rstrip("/"))
