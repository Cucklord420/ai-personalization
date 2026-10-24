# Maintain the profile

This is a Markdown/JSON profile repository. Reading it needs no installer, API, scheduler or executable. Maintenance validation uses a Rust command-line tool; its committed lockfile pins the dependency versions.

profile.json is canonical; schema/profile.schema.json describes its shape. render-spec.json identifies each rendering and selection. PROFILE/topic files and complete exports are derived views, not independent sources of truth.

## Rendering recipe

1. Preserve canonical record and group order. For each output select its listed groups or explicit record IDs.
2. Emit the heading/version/review banner. Group selected records by canonical group order.
3. Preserve each record's exact text and stable ID anchor. Detailed files include status, evidence/confidence, source IDs and actual last-supported date (or unknown).
4. Complete START_HERE/agent exports contain the current AI.md contract plus all records. Replace supporting relative links with repository-absolute links so a copied file stands alone.
5. Compact exports explicitly say they are partial. Keep custom-instructions.txt within the declared repository character budget; this is our budget, not an asserted platform limit.
6. Update README counts, manifest inventory/checksums, changelog and coverage/conflict/unknown notes when underlying evidence changes.

The profile and its exports remain Markdown/JSON. Rust is used for the repository's executable validator, not to replace those data formats.

## Validation

From the repository root, run:

```sh
cargo fmt --manifest-path validation/Cargo.toml -- --check
cargo test --manifest-path validation/Cargo.toml --locked
cargo run --offline --manifest-path validation/Cargo.toml --locked -- .
```

The validator checks the committed JSON Schema without following external references, Gregorian dates, unique IDs and resolved group/source references, every render selection and exact record coverage, both complete exports and their AI contract, local Markdown links/anchors, README counts, compact instructions length, and the exact manifest inventory, byte counts and SHA-256 hashes. It scans the worktree, staged blobs and reachable Git history for credential, contact, address, account and raw-transcript patterns. Findings report only a category and source path, never matched content.

The validator makes no network requests. Build dependencies may need to be downloaded once; the committed Cargo.lock makes that build reproducible. Git history checks require a Git checkout with the relevant history available. Acceptance cases in validation/acceptance-cases.json are review scenarios, not live model test results; static checks do not prove assistant behavior.

Read the latest branch head, preserve unrelated edits/history/license, make one coherent update and verify remote commit/files. Never force-push a stale head or publish private source material merely to improve traceability.

TCO: $0 new paid runtime/dependency is required; AI usage, hosting/account costs and maintenance effort remain environment-dependent.
