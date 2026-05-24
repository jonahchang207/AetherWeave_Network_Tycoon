---
layout: doc
title: GitHub Pages
nav: dev
section: dev
---

# GitHub Pages — Publish This Site

This documentation is a **Jekyll** site in the `/docs` folder.

## One-time setup

1. Push the repository to GitHub.
2. Open **Settings → Pages**.
3. Under **Build and deployment**:
   - **Source:** Deploy from a branch
   - **Branch:** `main` (or `master`) → folder **`/docs`**
4. Save. GitHub builds in 1–3 minutes.

Your site URL:

```
https://<username>.github.io/AetherWeave_Network_Tycoon/
```

## Configure `baseurl`

If you rename the repository, edit `docs/_config.yml`:

```yaml
baseurl: "/YourRepoName"
```

Mismatch causes broken CSS/links.

## Optional: set GitHub remote in footer

Edit `docs/_config.yml`:

```yaml
repository: your-username/AetherWeave_Network_Tycoon
```

Then update `docs/_includes/footer.html` to use `https://github.com/{{ site.repository }}` if you enable the `jekyll-github-metadata` plugin, or hardcode your URL.

## Local preview

```bash
cd docs
bundle install
bundle exec jekyll serve --livereload
```

Open `http://localhost:4000` plus your `baseurl` path (Jekyll prints the correct URL).

### Gemfile

The repo includes `docs/Gemfile` pinning `github-pages` gem for parity with GitHub’s build.

## Editing docs

| Section | Path |
|---------|------|
| Home | `docs/index.md` |
| Game design | `docs/game-design/*.md` |
| Tutorial | `docs/tutorial/*.md` |
| Development | `docs/development/*.md` |
| Styles | `docs/assets/css/main.css` |
| Layout | `docs/_layouts/default.html` |

Use front matter:

```yaml
---
layout: doc
title: Page Title
nav: home   # home | design | tutorial | dev
---
```

## CI (optional Pages workflow)

The default **branch /docs** build needs no workflow. For custom plugins, add `.github/workflows/pages.yml` using `actions/configure-pages` — not required for this site.

## Troubleshooting

| Issue | Fix |
|-------|-----|
| 404 on CSS | Wrong `baseurl` in `_config.yml` |
| Build fails on GitHub | Invalid YAML front matter; test with `bundle exec jekyll build` |
| Old content cached | Hard refresh; check Pages build log in Actions tab |
