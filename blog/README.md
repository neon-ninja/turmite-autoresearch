# Turmite research devlog

A small Jekyll site for researcher-written Markdown entries. The blog is
independent of the simulator in `web/` and does not publish the repository's
notes, raw data, or scratch files automatically. Ruby dependencies are pinned
in `Gemfile.lock`; Python and Rust dependencies are unchanged.

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
bundle exec jekyll serve --host 0.0.0.0 --baseurl ''
# Include work in progress only when previewing:
bundle exec jekyll serve --host 0.0.0.0 --baseurl '' --drafts --future
```

Open <http://localhost:4000>. No Node or Python build step is needed.
To build and check a project site locally:

```bash
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

Only blog changes and workflow changes trigger automatic builds. This does
not run simulations, publish all experiment files, or require a deployment
secret. The workflow needs Pages enabled and the normal `GITHUB_TOKEN`
permissions for Pages deployment.

References: [Jekyll authoring](https://jekyllrb.com/docs/posts/) and
[GitHub's custom Pages workflows](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages).
