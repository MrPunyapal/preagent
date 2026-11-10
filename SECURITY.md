# Security policy

Preagent is local-first software and does not provide a hosted inference API.

- Prompt classification works offline without credentials.
- Opt-in Ollama inference is restricted to the IPv4 loopback interface.
- Prompts and model replies are not persisted or uploaded by Preagent.
- Claude Code, Codex and OpenCode follow their own configured authentication,
  permission, sandbox, and provider policies.
- Always review untrusted repository content before sending it to any agent.

For security vulnerabilities, use GitHub's private vulnerability reporting
feature in this repository when available. Do not publish exploit details in
a public issue before coordinating a fix.
