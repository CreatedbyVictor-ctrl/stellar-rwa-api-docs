# Contributing to the docs site

The repository-wide rules are in [CONTRIBUTING.md](../CONTRIBUTING.md). This guide covers the docs site in `docs/`.

## Local development

```bash
cd docs
npm ci
npm run dev      # http://localhost:3000
npm run lint
npm run build    # must pass before you open a PR
```

## Sample checker

`npm run check:mdx-samples` extracts fenced `ts`, `tsx`, `js` and `jsx` blocks from `.mdx` files under `docs/app` and verifies that they parse (`docs/scripts/check-mdx-code-samples.mjs`). It checks syntax only: it does not run samples or resolve types, and it skips `bash`, `json` and `rust` fences. Run it before pushing.

## Adding a page

1. Create `docs/app/docs/<section>/<page>/page.mdx`, following an existing page in the same section.
2. Add an entry to the matching section in `docs/components/nav.ts`. Order in that array is the sidebar order, and it also drives previous/next links and search metadata. Place the page where a reader would expect to find it in the section.
3. Run `npm run check:mdx-samples` and `npm run build`.

## Style

Follow the [documentation style guide](./STYLE_GUIDE.md).
