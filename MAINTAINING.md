# Maintain the profile

This is a text/data repository with no required installer, application, API, scheduler or executable dependency.

profile.json is canonical; schema/profile.schema.json describes its shape. render-spec.json identifies each rendering and selection. PROFILE/topic files and complete exports are derived views, not independent sources of truth.

## Rendering recipe

1. Preserve canonical record and group order. For each output select its listed groups or explicit record IDs.
2. Emit the heading/version/review banner. Group selected records by canonical group order.
3. Preserve each record's exact text and stable ID anchor. Detailed files include status, evidence/confidence, source IDs and actual last-supported date (or unknown).
4. Complete START_HERE/agent exports contain the current AI.md contract plus all records. Replace supporting relative links with repository-absolute links so a copied file stands alone.
5. Compact exports explicitly say they are partial. Keep custom-instructions.txt within the declared repository character budget; this is our budget, not an asserted platform limit.
6. Update README counts, manifest inventory/checksums, changelog and coverage/conflict/unknown notes when underlying evidence changes.

Any capable editing agent can render this using its own tools; reading the profile needs no script. Rust-first applies to suitable executable projects, not these appropriate Markdown/JSON formats.

## Validation

Check JSON/schema shape, unique IDs, valid dates, resolved source/group references, allowed statuses and nonempty sourced text. Review contradictory active requirements, unsupported claims, privacy and stale dates.

Verify exact canonical record text appears once in its assigned core/topic file and each complete export. Check local links/anchors, compact character budget and manifest hashes. Behavior cases in validation/acceptance-cases.json support review/future model evaluation; static file checks do not prove live AI behavior.

Read the latest branch head, preserve unrelated edits/history/license, make one coherent update and verify remote commit/files. Never force-push a stale head or publish private source material merely to improve traceability.

TCO: $0 new paid runtime/dependency is required; AI usage, hosting/account costs and maintenance effort remain environment-dependent.
