---
type: docs-journal-list
date: 2026-08-29
time: 12:04
wave: dory-docs-1204
role: dj_list
verdict: LIST_PASS
head: 049e304
p_feat: f1c966c
rust_land: b544f5f
leftover_readme: 68190a5f
land_readme: git show HEAD:README.md
dory: not invoked
t13: not sat
herdr_server_stop: not run
recook_p: no
agentwiki: skipped
---

# Journal list — 2026-08-29 P entry

**Verdict: LIST_PASS**

`ak journal list` from `/home/manhquy/Downloads/flow/dory` now returns the 2026-08-29 P ship journals. Files already existed (`ok:true` on validate). List was empty because this tree was not in `~/.agentkit/projects.json` (only `cmc_edu`). `ak journal show` failed with `journals: entry not found` for the same reason. `ak plan list` already knew `project_slug: dory`.

Did not sit `t13`. Did not invoke `dory`. Did not `herdr server stop`. Did not close `wP`/`w15`/`w16`/`t13`. Did not edit leftover 5. Did not `git add -A`. Did not recook P. Did not claim company Phase 5. Land README cited as `git show HEAD:README.md` (blob `5ac82b10…`); working `README.md` still mint `68190a5f`.

## Cause

| Probe | Result |
|---|---|
| `ak journal validate 2026-08-29-p-unlock-3-shipped-docs-route-opened-3` | `ok: true`, date `2026-08-29` |
| `ak journal list --json` (before) | `kind: journal.list`, `data: []` |
| `ak journal show …-opened-3` (before) | exit 1, `Error: journals: entry not found` |
| `ak journal list --project cmc_edu` | cmc_edu rows only |
| `ak projects list` | `cmc_edu` only |

List/show walk the global registry. Validate reads `plans/journals/` on disk. Creating a fourth identical file would not have filled the list.

## Fix

```
ak projects add . --json
```

Envelope: `kind: projects.add`, `name: dory`, `dir: /home/manhquy/Downloads/flow/dory`, `was_update: false`, `registered_at: 2026-08-29T05:16:28Z`. Writes `~/.agentkit/projects.json` only. Not leftover 5. Not a product cook.

## After

`ak journal list --from 2026-08-29 --to 2026-08-29 --json --no-interactive`:

| project_id | slug | filename | date |
|---|---|---|---|
| dory | `p-unlock-3-shipped-docs-route-opened-2` | `2026-08-29-p-unlock-3-shipped-docs-route-opened-2.md` | 2026-08-29 |
| dory | `p-unlock-3-shipped-docs-route-opened` | `2026-08-29-p-unlock-3-shipped-docs-route-opened.md` | 2026-08-29 |
| dory | `p-unlock-3-shipped-docs-route-opened-3` | `2026-08-29-p-unlock-3-shipped-docs-route-opened-3.md` | 2026-08-29 |

Bare `ak journal list --json` also returns those three as the newest dory rows (`project_id: dory`, title `P unlock-3 shipped; docs route opened`). `ak journal show 2026-08-29-p-unlock-3-shipped-docs-route-opened-3 --json` now `kind: journal.show`.

Collision suffixes `-2` / `-3` are prior `ak journal create` retries against the empty list. File is source of truth; no fourth create.

## Hold

| Check | Evidence |
|---|---|
| HEAD | `049e304 docs(plan): check isolate prd-unlock phases` |
| P feat | `f1c966c feat(isolate): fail-then-pass flow.sh prd` |
| Rust log | `b544f5f fix(attach): do not auto-start server on sit` |
| Leftover 5 still `M` | `README.md` + four rust leftover paths |
| Leftover README mint | `git hash-object README.md` = `68190a5f…` |
| Land README | `git show HEAD:README.md` — operator desk how-to; "Xong tới đâu" has no isolate taxi |

AgentWiki publish skipped. Journals are work history, not WHERE. WHERE: `docs/README.md`.
