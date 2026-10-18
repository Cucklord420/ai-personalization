# Native adapters

Provider-independent profile content must fit the destination's actual capabilities, permissions and context limits. These conventions were checked on 10-18-2026; this is not proof of installation.

| Destination | Entry | Apply | Limit |
|---|---|---|---|
| Chat or other AI | PROMPT.md / START_HERE.md | Read or paste the content | Persistence requires that platform's supported mechanism |
| Codex / compatible agents | AGENTS.md | Read this checkout; merge the agent export into the supported broader scope if requested | Unrelated checkouts do not inherit it automatically |
| Claude Code | CLAUDE.md | Local AI/PROFILE imports; load relevant modules | Preserve existing global/project instructions |
| Gemini CLI | GEMINI.md | Local context imports; load relevant modules | A pointer does not synchronize files |
| Muse / file-based agent | adapters/MUSE.md | Map content to actual live files | Preserve identity, jobs and alert history |

Criteria: instruction loading, persistence scope, access and honest limits.

For an existing workspace inspect and version its files before merging. Use exports/agent-instructions.md for a self-contained import, or exports/custom-instructions.txt for limited space. No adapter here writes home-directory settings, connects accounts, deploys services or creates jobs by itself.

Sources: OpenAI — 10-18-2026 — https://learn.chatgpt.com/docs/agent-configuration/agents-md.
Sources: Anthropic — 10-18-2026 — https://code.claude.com/docs/en/memory.
Sources: Google Gemini CLI — 10-18-2026 — https://google-gemini.github.io/gemini-cli/docs/cli/gemini-md.html.

Muse mapping comes from the saved supplied workspace exports, not an assumed universal API.
