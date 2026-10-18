# AI Personalization

A portable set of instructions and user-approved context for AI assistants. Point an assistant at this repository at the start of a conversation; it should load the core files, adapt to the task, and keep the profile current when new durable preferences become clear.

> **Privacy:** this repository is public. Keep raw chat exports, personal records, credentials, and sensitive details out of it. Store only concise, low-risk preferences here. Use a private repository before adding private context.

## For an assistant

Read [AI.md](AI.md), then [PROFILE.md](PROFILE.md). Use [LEARN.md](LEARN.md) when the user asks you to learn from chat history or update this profile. If the platform supports persistent instructions, link or copy the short instruction in [PROMPT.md](PROMPT.md).

When available, use the user's directly supplied chat export or memory source. Do not imply you inspected a source you could not access. Separate confirmed preferences from hypotheses, keep provenance and dates, and ask only when ambiguity affects the outcome.

## Quick start

Tell an AI:

> Read https://github.com/Cucklord420/ai-personalization and follow AI.md. Apply the profile to this task. If you have access to my chat history or memory, follow LEARN.md and update the profile only with durable, useful preferences. Do not copy raw chats or sensitive details into this public repository.

For an agent that can clone repositories, clone this repository and read the same files. A public link cannot grant access to private chats or account memory; those sources must be available through that platform or provided separately.

## Files

- [AI.md](AI.md): behavior expected from any assistant using this repo.
- [PROFILE.md](PROFILE.md): the concise current profile, with confidence and provenance.
- [LEARN.md](LEARN.md): process for extracting preferences from conversation history without overfitting.
- [PROMPT.md](PROMPT.md): short reusable setup prompt for platforms with limited instruction space.

## Profile status

This is a starter profile. The current session establishes that you want a reusable personalization repo for ChatGPT and other agents, and that “Muse” is an example of prior work. It does not expose your ChatGPT memory or past conversation contents, so the detailed profile has not yet been inferred from them.
