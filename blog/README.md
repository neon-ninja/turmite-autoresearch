# Turmite research devlog

A small Jekyll site for researcher-written Markdown entries. The blog is
independent of the simulator in `web/` and does not publish the repository's
notes, raw data, or scratch files automatically. Build dependencies are pinned
in `Gemfile.lock` and `uv.lock`.

## Write an entry

1. Copy `templates/post.md` to `_posts/YYYY-MM-DD-short-title.md`.
2. Set the title, description, date **with a timezone offset**, author, status,
   and tags. Suggested statuses: `Hypothesis`, `Experiment`, `Null result`,
   `Instrument`, and `Project note` (free text).
3. For research results, set `experiment` to its repository-relative folder
   and `commit` to the full Git revision containing the evidence. Link data,
   scripts, commands, seeds, fit windows, calibration, and uncertainty in the
   body. Blog entries summarise the scientific record; they do not replace it.
4. Preview, review the generated page, and commit the source in a PR.

Use `_drafts/short-title.md` while writing (create the directory as needed).
Drafts and future-dated posts are excluded from production. To publish a draft,
move it into `_posts/` with a dated filename and update its date. Drafts committed
to a public repository **are public source**, even though absent from the site.

Store small figures in `assets/posts/<slug>/`. In Markdown, use a base-path-aware
image URL:

```liquid
![Descriptive alt text]({{ '/assets/posts/my-entry/figure.png' | relative_url }})
```

Use fenced code blocks with a language, Markdown tables, and normal headings.
For inline equations use Unicode or HTML (`t<sup>1/3</sup>`); this bootstrap
does not load a remote maths renderer. For cross-post links use Jekyll's
`post_url` tag passed through `relative_url`:

```liquid
{% capture entry_url %}{% post_url 2026-10-19-opening-the-research-log %}{% endcapture %}
[Opening entry]({{ entry_url | relative_url }})
```

Add corrections under a new dated heading; preserve the original prediction.
Avoid numerical claims without a reproducible source.

## Local preview (WSL)

Requires Ruby 3.3 and Bundler. Run in WSL from `/mnt/f/Code/turmite/blog`:

```bash
export UV_PROJECT_ENVIRONMENT=$HOME/.venvs/turmite UV_LINK_MODE=copy
export BUNDLE_PATH=$HOME/.local/share/turmite-blog-gems
bundle install
cd ..
uv run --frozen --only-group blog blog/scripts/experiments.py
cd blog
bundle exec jekyll serve --host 0.0.0.0 --baseurl ''
# Include work in progress only when previewing:
bundle exec jekyll serve --host 0.0.0.0 --baseurl '' --drafts --future
```

Open <http://localhost:4000>. The Python preparation step publishes the
experiment records; Jekyll builds the complete site. Re-run preparation after
changing an experiment, then restart Jekyll or let its watcher rebuild.
To build and check a project site locally:

```bash
# From the repository root, using the WSL environment exports above:
uv run --frozen --only-group blog blog/scripts/experiments.py --baseurl /turmite
cd blog
JEKYLL_ENV=production bundle exec jekyll build --strict_front_matter --baseurl /turmite
bundle exec ruby scripts/check.rb _site /turmite
```

Output is in ignored `_site/`. The checker verifies required pages, local
links and assets, project-site prefixes, and exclusion of authoring files.
Canonical URLs and feed URLs use `url` and `baseurl`; for a production-like
local build, set `url` in a separate config override. `repository_url` enables
source and provenance links. GitHub Actions supplies all three automatically.

## GitHub Pages setup

1. Push this repository to GitHub. In **Settings → Pages → Build and
   deployment → Source**, select **GitHub Actions**.
2. Ensure the default branch is `master` or `main` (or adjust the workflow's
   push branches). The first push containing the blog runs the workflow;
   alternatively run **Research blog** manually on the default branch.
3. The workflow builds and checks PRs, and deploys only from the repository's
   default branch. It uses the `github-pages` environment. If environment
   approvals or branch rules are enabled, satisfy them in GitHub.

The workflow uses the pinned Ruby bundle for both local and CI builds. It
gets the site's origin and base path from `actions/configure-pages`, so both
`https://OWNER.github.io/REPO/` and user/organisation sites work. Custom domains
are configured in Pages settings; add `blog/CNAME` if the domain setup requires
it. Feed links and canonical URLs follow the Pages configuration.

Blog, experiment, Python dependency, and workflow changes trigger automatic
builds. This does not run simulations, publish all experiment files, or require a deployment
secret. The workflow needs Pages enabled and the normal `GITHUB_TOKEN`
permissions for Pages deployment.

## Experiment pages and notebook renders

The preparation script discovers tracked `experiments/*/README.md` files,
excluding internal `_` folders. Every experiment gets an entry in
`/experiments/` and a page at `/experiments/<folder-name>/`. The searchable
index remains usable without JavaScript; JavaScript adds filtering.

Each page includes its README, every tracked `notebook.ipynb` at its root
and every `.ipynb` in its `notebooks/` directory, plus an expandable
`NOTEBOOK.md` lab log when present. Notebook narrative, code, saved figures,
tables, text, and error outputs are rendered with nbconvert. Code inputs are
collapsed individually; outputs stay visible. Original notebooks can be
downloaded. Missing notebooks and notebooks without saved outputs are labelled.
**No cells are executed**, and original scientific files are never rewritten.

The `blog` dependency group in `pyproject.toml` and `uv.lock` pins nbconvert,
nbformat, and Beautiful Soup independently of the analysis dependencies.
Preparation writes only generated `blog/experiments/` and
`blog/_data/experiments.json`, both ignored by Git. The generated directory
has an ownership marker; the script refuses to replace an unmarked directory.
CI regenerates these files before Jekyll runs. Run preparation before a local
build too, with the **same baseurl** as Jekyll.

Relative experiment and notebook links point to the corresponding site page.
Other repository references point to GitHub at the source revision when
`--repository-url https://github.com/OWNER/REPO` is provided (CI sets this).
Without it, repository-only references retain their labels and show their
source path on hover. Referenced, tracked images are copied; bulk dumps and
untracked files are never copied. Unavailable legacy references are left as
labelled text and listed in `/experiments/manifest.json`. This manifest also
records the source revision and SHA-256 of every rendered input and copied
asset. A render reflects the saved document; it does not verify its claims or
refresh stale notebook results.

Publishing checks (from the repository root under WSL):

```bash
uv run --frozen --only-group blog blog/scripts/test_experiments.py
```

The fixtures check repeatable output, preserved saved and error outputs,
nested notebooks, source and image links, heading anchors, missing notebooks,
exclusion of untracked/internal files, and protection of unmarked directories.

References: [Jekyll authoring](https://jekyllrb.com/docs/posts/) and
[GitHub's custom Pages workflows](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages).
