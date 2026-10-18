# Software development

Profile version: 1.0.0 · Reviewed: 10-18-2026 · America/New_York

Rendered from [../profile.json](../profile.json). Dates are evidence dates, not live verification. See [../SOURCES.md](../SOURCES.md) and [../UNKNOWNS.md](../UNKNOWNS.md).

## Software development

<a id="record-code-rust"></a>

- **code-rust** — Rust is the default for new personally maintained executable software. Prefer Rust migrations where practical; preserve required Markdown, JSON, configuration, platform interfaces and other suitable non-Rust formats.
  
  required; active; retrieved_explicit/high; last supported: 10-15-2026; sources: S02, S03

<a id="record-code-parity"></a>

- **code-parity** — For an existing project, preserve functionality and interfaces. Keep the existing implementation available until behavioral parity, relevant build/test checks and recovery are verified.
  
  required; active; retrieved_explicit/high; last supported: 10-15-2026; sources: S02, S03

<a id="record-code-no-blind-rewrite"></a>

- **code-no-blind-rewrite** — Do not force Rust when platform constraints or a clearly better alternative make it unsuitable. Do not rewrite unrelated projects merely because this profile was loaded.
  
  required; active; retrieved_explicit/high; last supported: unknown; sources: S02, S03

<a id="record-code-verification"></a>

- **code-verification** — Inspect repository rules, dependencies and the official Rust guide/documentation where relevant. Test meaningful behavior, failure cases and migration parity; do not present queued, unrun or static checks as runtime success.
  
  required; active; retrieved_explicit/high; last supported: 10-15-2026; sources: S03

<a id="record-code-open"></a>

- **code-open** — Prefer open-source, portable implementations and ordinary Git. Justin has requested licenses that preserve openness; inspect the existing license and task before selecting or changing one.
  
  preferred; active; retrieved_explicit/high; last supported: unknown; sources: S02, S03

<a id="record-ai-use-cases"></a>

- **ai-use-cases** — Prioritize AI for research, reasoning, troubleshooting, automation, computer use, agent workflows and local inference. Do not default to creative-generation pitches.
  
  preferred; active; documented/high; last supported: unknown; sources: S02, S05, S07

<a id="record-ai-portability"></a>

- **ai-portability** — Keep prompts, memory, sessions, configuration, files and recovery portable across assistants. Prefer local/private operation, replaceable model/hosting providers and documented exports; GitHub may host the data but must not be indispensable.
  
  required; active; retrieved_explicit/high; last supported: unknown; sources: S03, S05, S07

<a id="record-ai-local-context"></a>

- **ai-local-context** — On-device AI was required for current work on 10-10-2026; this is a task-specific requirement, not a universal constraint on every laptop or every AI service.
  
  required; active; retrieved_explicit/high; last supported: 10-10-2026; sources: S04

<a id="record-ai-capabilities"></a>

- **ai-capabilities** — Verify agent persistence, background execution, permissions, privacy, reliability, export and actual costs. Marketing, a saved prompt or a schedule written in a file is not proof of runtime capability.
  
  required; active; retrieved_explicit/high; last supported: unknown; sources: S03, S05, S06

<a id="record-code-guide-first"></a>

- **code-guide-first** — Prefer practical complete setup instructions and verified behavior over undocumented claims. A source file, passing format check or CI definition does not prove a real deployment, migration or restore.
  
  required; active; documented/high; last supported: unknown; sources: S03, S11

