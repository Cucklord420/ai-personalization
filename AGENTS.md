**OWNER SECURITY-FIRST DIRECTIVE (10-30-2026): SECURITY IS PRIORITY #1.** For every task, repository, agent, PR, CI workflow, and release, security takes precedence over features, speed, performance, cost, convenience, Rust migration, and backlog closure. Triage vulnerabilities, exposed secrets, privacy/access risks, supply-chain threats, CI/release integrity, and recovery hazards first. Never bypass security checks, required reviews, least privilege, privacy boundaries, or safe rollback to finish other work. Block unsafe merges, publication, deployments, and false security-issue closures. Preserve stricter local rules; this policy does not authorize destructive operations or public disclosure of private findings.

# Repository instructions

Read AI.md and PROFILE.md before helping Justin. In research, actively find negative reviews, credible failures and edge cases, count independent complaints where defensible, and reevaluate earlier research. Load relevant context/ modules; START_HERE.md is the complete import. Follow current intent and consult CONFLICTS.md / UNKNOWNS.md when needed.

When editing:
- profile.json is canonical. Follow LEARN.md, MAINTAINING.md and render-spec.json; synchronize derived files.
- Preserve user edits, history and the existing license. This repo is public: no raw chats, private records, credentials, diagnostics or private source locators.
- Validate provenance, scope, dates, schema/IDs, links, privacy and consistency. Report checks actually performed.
- Profile loading does not authorize other-repo changes, new tasks, messages or production actions. Rust-first applies to suitable executable software; Markdown/JSON remain intentional.

## Separation of responsibilities

This repository is the canonical source for **user personalization**: enduring preferences, user-reported context, provenance, corrections, and portable AI import/export. `profile.json` is authoritative; generated Markdown is a projection. Do not turn preferences into autonomous permissions or assert a preference is a deployed technical policy. Global assistant operating rules, validation duties, and instruction hierarchy belong in the repository that owns those rules; project-specific decisions, equipment truth and deployments belong in the relevant project repository. Cross-link rather than duplicating competing canonical rules. The mature-open preference is a user preference here: favor open standards, but recommend polished, supported, low-maintenance solutions over unfinished DIY. Never infer that technical experience implies willingness to tinker. A merged GitHub file does not automatically change ChatGPT account personalization.
