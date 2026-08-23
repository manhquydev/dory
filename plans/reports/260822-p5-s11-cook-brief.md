---
type: cook-brief
date: 2026-08-22
status: unheld
unheld_by: 260823-0856-brainstorm-s11.md
held_by: 260822-1942-brainstorm-occupant-lock.md # occupancy prerequisite met 2026-08-23
advice: persist
authority:
  - CHARTER.md
  - plans/reports/260822-skill-cli-socket-contract.md
  - plans/reports/260822-1923-brainstorm-next-direction.md
  - skills/dory/SKILL.md
---

# Cook brief — 1a close contract §11

Do **not** cook 1b (self-report hook), 1c (`omp` allowlist), remaining §5 verbs, or deep Flow in this wave.

Kill the wave if rust classifier, `--kind`, `flow-skill` bytes, or new §5 verbs appear without a proven miss.

## Contract

Outcome: a named occupant **inside** a Dory pane (`DORY_ENV=1`) splits, starts, prompts, waits, and gets a Flow verdict on a real external repo.  
Constraints: three houses; no `herdr`/`dsh` at runtime; no Xia `--copy`; no `--kind` farm; classifier table unchanged.  
Non-goals: LLM occupant, §5 verb fill, Session OS fold, phase-file flip.  
Acceptance: a stranger can mark every §11 clause PASS on the new test’s world-state, not on a header.

## Six rules

1. Test process may: start server, `workspace create`, parse `.result.root_pane.id`, `agent start` the **named driver**, then only `pane read` / `wait-output` / `agent get` for asserts. It must **not** call `pane split`, `agent start` (peer), `agent prompt`, `agent wait` (peer), `pane run`, or `dory flow`.
2. Named driver argv is **`/bin/bash <fixture>`** (script, not a sitting interactive shell). Root pane is already `bash --norc`; a bare `/bin/bash` occupant cannot type the loop under the harness ban. Fixture may print `DORY_OCC_READY` so the driver’s own start can settle. **No** `rust/src` classifier edit.
3. That occupant types the loop in `skills/dory/SKILL.md`: gate `test "${DORY_ENV:-}" = 1` → `dory --help` → `pane split --current --direction right --no-focus` → parse `.result.pane.id` (reject invented `w1`) → `agent start` a peer → `prompt --wait` → `agent wait` → `cd` the temp clone → `dory flow -- status`.
4. “Loads SKILL.md” is mechanical: fixture **opens** `skills/dory/SKILL.md` (absolute path from the dory crate, not a `rust/` relative). Issued verb texts must occur in that file **and** on the named occupant’s pane transcript. Not an LLM. Not `cat` the skill then replay the old slave strings.
5. Judged tree: **lock original to** `/home/manhquy/Downloads/spec-kit` (git, ~30 MB, no `flow/`, no `.dory`). Clone into temp. Do **not** clone `affiliate-partner-finder` (~2.7 GB — timeout/disk). Do not mutate spec-kit. Foreign `FLOW_BIN` = `flow-skill/.../flow.sh`. `DORY_WORKSPACE_DIR` is **not** injected at PTY spawn; fixture must `cd` the clone and export `FLOW_PROJECT_ROOT=<clone>`. Journal `{clone}/.dory/sessions/s1.jsonl`; journal cwd equals the clone; original spec-kit gains no `.dory`. Nonzero `status` on a tree with no `flow/` is an allowed verdict — do **not** mint `flow/` or copy `flow-skill/flow/` into the clone. Forbidden: `eval/phase5-project`, `/bin/true` as judge, cwd/`FLOW_PROJECT_ROOT` into `flow-skill`.
6. Do not write `.dory/` into `flow-skill` or `flow-deck`. Do not use those houses, `flow-deck-win`, or the dory crate as the judged project.

Peer argv after split: existing `DORY_OCC_READY` reader (`sh`/`bash`), or `prompt --wait` will not settle.

## Asserts

- Authorship: split/start/prompt/wait/flow appear on the **named occupant’s** pane transcript / were issued with `$DORY_BIN` by that occupant.
- `printenv DORY_ENV` → `1`.
- Journal `flow/invoke` + `flow/result`; journal cwd equals the spec-kit **clone**; judge exit preserved.
- After the test: `rg -i dory` on `flow-skill` is 0; no `flow-skill/.dory`; original `/home/manhquy/Downloads/spec-kit` has no new `.dory`.
- Path of judged cwd ≠ dory crate, ≠ `flow-skill`, not under `flow-skill`, ≠ `flow-deck`, ≠ `eval/phase5-project`, and the clone was not created by copying `flow-skill/flow/`.

## Keep as history, do not call §11

`p5_inside.rs`, `p5_live_loop.rs`, `p5_real_repo.rs`.

## Do not

- Flip `phase-05-occupant-wait.md` or `plan.md`.
- Add `tab get` / `pane layout|focus|send-keys|input` unless this cook hits a hard wall; then **exactly one** verb.
- Start `omp`, allowlist a coding-agent comm, or implement the self-report hook.
- Teach `flow-skill` the word `dory`.
- Grow `next` / `card` / `check` inside Dory.

## Done when

`cargo test --offline --locked` is green **and** the new test’s world-state fills the §11 table (Driver / `DORY_ENV` / verbs / foreign verdict / real external repo / not-header) as PASS. Score from files, not from this brief’s title.
