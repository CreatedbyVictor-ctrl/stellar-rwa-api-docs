# Verifying documented API examples

`scripts/verify-doc-examples.mjs` (repo root) extracts each `curl` + `json`
example pair from the `.mdx` pages and compares the documented response with a
live response by shape (keys and value types).

```bash
API_BASE_URL=https://<testnet-api-host> node scripts/verify-doc-examples.mjs
```

Point `API_BASE_URL` at a local instance (`http://localhost:8080`) or a mock to
run it offline. The script exits 1 when drift is found.

A scheduled workflow, `.github/workflows/doc-examples-drift.yml`, runs it weekly
against the URL in the repository variable `DOC_EXAMPLES_API_URL`. It has no
pull request trigger, so drift is reported without blocking merges. The Rust
tests in `api/tests/` continue to validate examples against the response models.
