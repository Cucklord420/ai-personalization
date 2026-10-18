# Learn from chat history and maintain the profile

Use this when the user asks you to inspect past chats, memory, or other personal context, or asks to update their preferences.

## 1. Check what is actually available

- Look for a connected memory/history feature, files the user provided, or a source they explicitly point to.
- If none is available, say so plainly. Do not silently substitute assumptions or pretend the archive was inspected.
- For exports, first inspect the export's own handling and privacy instructions. Keep raw exports in their intended private location; this repository is public.

## 2. Extract durable signals

Review only material relevant to personalization. Look for patterns across different tasks, especially explicit corrections and repeated requests:

- How the user likes answers structured and how much detail they want.
- How they prefer collaboration, clarifying questions, and execution.
- Technical environment, tools, and recurring project context that helps future work.
- Stable constraints, priorities, and recurring dislikes.

A one-off task instruction is not automatically a standing preference. A single example is weak evidence. Do not infer sensitive traits or make personality diagnoses.

## 3. Record a compact profile

For each preference, keep:
- The preference in plain language.
- Evidence type: `explicit`, `repeated`, or `inferred`.
- Confidence: `high`, `medium`, or `low`.
- Date last supported, when available.

Use PROFILE.md for the resulting concise profile. Keep it useful to an assistant that has not seen the original conversations. Do not paste chat excerpts or conversation summaries. Prefer generalizable patterns over detailed personal history.

## 4. Protect privacy

This repository is public. Do not commit raw chats, attachments, export files, sensitive facts, secrets, or identifying details. If useful personalization depends on private information, keep it in a private store and link to it only if the user has explicitly configured a safe mechanism. A private source does not become public-safe just because the user asked you to read it.

## 5. Validate and report

After editing, review the diff and ensure every statement is supported and appropriate for this repository's visibility. Tell the user what source you inspected, what durable patterns you added, and what remains uncertain. Never say you reviewed all chats if you only saw a subset.
