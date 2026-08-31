# Security Policy

**Tiếng Việt:** báo cáo lỗ hổng theo mục *Reporting a vulnerability* bên dưới. Đừng mở issue công khai.

## Supported versions

| Version | Supported |
|---|---|
| `@manhquy/dory` on npm (`next` and `latest` tags) | Yes |
| Desk built from `main` | Yes |
| Unpublished / local forks | Best effort |

There is no long-term support line yet. If a fix lands on `main`, it ships in the next lamp prerelease (`0.1.0-next.N`) and in the next desk source snapshot.

## Reporting a vulnerability

Please **do not** file a public GitHub issue or discussion for a security problem.

1. Use [GitHub Private vulnerability reporting](https://github.com/manhquydev/dory/security/advisories/new) on this repository, or
2. Email **manhquy.mqy@gmail.com** with a description, impact, and a way to reproduce.

We will acknowledge the report and work on a fix before any public disclosure.

Do not attach tokens, cookies, private keys, or personal data you do not need to prove the issue.

## What this project will not do

- Dory does not install Node, Rust, or a version manager on your machine.
- The npm package `@manhquy/dory` ships the journal **lamp** (`dory-serve` only). It is not the desk binary `dory`. The unscoped npm name `dory` is a different product; do not install it as this project.
- Do not send maintainers npm tokens or GitHub PATs in issues, PRs, or chat.

## Desk vs lamp

A report against the Rust desk (PTY, socket, occupant CLI) and a report against the Node lamp (HTTP journal on `:7380`) are different surfaces. Say which engine you hit.
