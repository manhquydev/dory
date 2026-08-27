OPS_PASS

when=2026-08-27
cwd=/home/manhquy/Downloads/flow/dory
wave_tabs=w13:t1W t1X t1Y t1Z
factory=w13:t13 / w13:p2R
ship=paper pathspec only; no leftover 5; no push

== leftover 5 mint ==
68190a5ffa073c082aa318aad5ed032e13cc90e3 README.md
602479094e84d31ad6f017775a3d55aeb485c644 rust/src/attach.rs
373d688636ff7315ccd665f450069d8284eb47ff rust/src/main.rs
4de1554ad56e248cdcf42f02111b7389b08dae82 rust/src/server.rs
9c28fc3e0f3666498a8952411242d5301f7911de rust/tests/p5_attach.rs

== PATH ==
~/.local/bin/dory gone
type -a dory empty
leftover ELF kept sha256=3ba0e3bc3e9630c8f0b544523610b2f4e784c6f47ad049ab0b67f10704f1bc14

== sock ==
$XDG_RUNTIME_DIR/dory/default/dory.sock not connectable
DORY_* unset on factory

== ship allowlist ==
plans/260827-0940-unlink-leftover-path-dory/
plans/reports/260827-0940-unlink-roster.*
plans/reports/260827-0927-brainstorm-eval-team.md
plans/reports/260827-unlink-path-leftover-*
