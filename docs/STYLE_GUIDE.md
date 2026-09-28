# Documentation style guide

## Voice
- Concise, technical and honest about limitations. Second person ("you"), present tense, active voice.
- No placeholders ("TODO", "coming soon") and no marketing language.

## Terminology
- "Stellar RWA" for the platform; "asset token", "compliance contract", "holder" for contract concepts.
- Use "API" for the REST service and "contract" for on-chain Soroban code. Do not mix them.
- Use the exact names of endpoints, fields and error codes as they appear in the API.
- Spell out an acronym on first use per page.

## Headings
- One `#` title per page (from `DocHeader`); sections use `##`, subsections `###`. Do not skip levels.
- Sentence case ("Adding a page", not "Adding A Page"). No trailing punctuation.

## Code samples
- Every sample must be real and correct against the deployed contracts and current API.
- Always set a language on the fence (`ts`, `bash`, `json`, `rust`). Fenced `ts`/`tsx`/`js` samples must parse: run `npm run check:mdx-samples`.
- Show a `curl` request followed by the `json` response it returns; keep the response complete enough to match the API (see `docs/DOC_EXAMPLES_VERIFICATION.md`).
- Use `http://localhost:8080` for local API examples and placeholder values that are obviously fake.
- Use `CalloutBox` for notes and warnings and `ApiEndpoint` for endpoint headers.
