# Handoff

The repository is the user's one-person version of the game (decided 2026-11-15 and 2026-11-16). The goal is to build the user's own design in `docs/design.md`; nobody else signs work off. Remotes: `origin` is the user's repository `skymiku39/jadereaper`, `upstream` is `pomodorozhong/jadereaper` (read-only for this account; do not open issues or PRs there unless asked).

Run git, `cargo` and `./build.sh` inside WSL only. Windows git on the `\\wsl.localhost` path writes `.git` files as root and breaks later commits.

## In progress

### chore/solo-housekeeping

- Task: phase 0 of the plan agreed on 2026-11-16. Record the one-person process and the mission list, and fix stale documents, before feature work starts.
- PR: none. Not pushed.
- Code checkpoint: see `git log`; the branch is cut from `main` at 82e268d.
- Changed: `CONTRIBUTING.md`, `AGENTS.md` and the PR template drop the issue-first Proposal process; `docs/design.md` gains section 4.2.1, the mission type list (seven settled, four suggested by the agent and waiting for the user's answer); `ROADMAP.md` gets a current "Current state" and the agreed working order; the generator command in the `content/game.toml` comment is corrected; the remotes were renamed (`fork` to `origin`, the old `origin` to `upstream`).
- Verification on 2026-11-16 (Rust 1.100, Node 24.22.3, Linux x86-64 under WSL): `cargo test --release --manifest-path core/Cargo.toml` passed (42 unit tests, 16 content tests); `./build.sh` passed and left the committed wasm and generated files unchanged; `node --check` passed on every `src/*.js` and `tools/*.mjs`. `npm run smoke` could not run here: Chromium fails to start because the system library `libnspr4.so` is missing. Fixing it needs `sudo npx playwright install-deps chromium`, which the user has to run.
- Environment gaps found: `rustfmt` and `clippy` are not installed (`rustup component add rustfmt clippy`); `gh` is not installed inside WSL (the Windows one works for read-only calls); Node is 24 while `.node-version` and `package.json` ask for 26.
- Next action: P-15, then start item 1 of `Ranked work`.
- Pending: **P-15** push this branch to `origin`, open a PR on `skymiku39/jadereaper` and merge it once CI passes. Why: pushing and merging need the user's go-ahead. Blocks: the new process and mission list reaching `main`.

### ci/core-checks

- Task: item 1 of `Ranked work`, the rest of CI. Stacked on `chore/solo-housekeeping`, so that branch merges first.
- PR: none. Not pushed.
- Code checkpoint: see `git log`; the branch is cut from `chore/solo-housekeeping`.
- Changed: a new `Core` workflow (`.github/workflows/core.yml`) runs format, clippy, the tests of `core/` and `tools/gen/`, `./build.sh`, a check that the build left no committed file changed, and `node --check` on every script. `rust-toolchain.toml` pins Rust 1.100.0 with rustfmt, clippy and the wasm target. `rustfmt.toml` sets `max_width = 120` and `use_small_heuristics = "Max"`, chosen because they change the existing compact code least. Both crates were formatted once and three clippy findings fixed (`Rng::next` is now `Rng::next_u32`). The Fonts workflow now triggers on the three font tool files instead of all of `tools/`. The smoke test connects to 127.0.0.1.
- Verification on 2026-11-16 (Rust 1.100.0, Node 24.22.3, Linux x86-64 under WSL): format check, clippy with `-D warnings` (native and wasm targets), `cargo test` (42 and 16 passed), `./build.sh` and `npm run smoke` (WebGPU available, played 3 s) all passed. A clean rebuild gave the same wasm bytes twice on this machine. Not verified: the `Core` workflow itself has never run, and whether the runner builds the same wasm bytes as this machine is only known once it does. If it does not, the stale-build step fails on the first PR and needs a look.
- This supersedes the "Environment gaps" line in the `chore/solo-housekeeping` subsection: rustfmt and clippy come with the pinned toolchain, and the smoke test runs locally since the user installed Chromium's system libraries. Node is still 24 locally against the 26 the repository asks for; everything passed on 24.
- Next action: P-18, then item 2 of `Ranked work`.
- Pending: **P-18** push this branch to `origin` after P-15, open a PR on `skymiku39/jadereaper` and merge it once CI passes. Why: pushing and merging need the user's go-ahead. Blocks: CI coverage for all later work.

## Ranked work

The working order agreed with the user on 2026-11-16. Details of each roadmap step are in `ROADMAP.md`; mission types are in `docs/design.md`, section 4.2.1.

1. The rest of CI (roadmap step 5): run `cargo test` in CI, fail on a stale wasm or stale generated files, `cargo fmt --check`, `cargo clippy`, `node --check`. With it, two small fixes: run the Fonts workflow only when font tools or text change, and make the smoke test connect to 127.0.0.1 instead of localhost.
2. Fixed time step, replays and the headless balance tool (roadmap step 3).
3. Mission framework: objective, win condition, result screen, mission picker on the title screen. First mission: survive a time.
4. One or two more mission prototypes (kill a count, then the user's pick).
5. Data-driven content rewrite (roadmap step 1), then martial arts × magic elements and reactions. Needs the user's answers first: what each of the four elements does, which pairs react, and how many martial arts can be held at once (design document, open items 2 and 4).
6. Remaining mission prototypes, the game feel pass (roadmap step 2), then the rest of roadmap step 4.
7. English UI fonts to match the Chinese set.
8. Touch controls and a WebGL2 fallback (roadmap step 6); scaling to thousands of enemies (roadmap step 7).

## Pending

- **P-16** Delete the old local branches `docs/game-design`, `feat/content-generator`, `test/weapon-behaviour`, `integrate/own-version` and `docs/own-pages-url`, the worktrees under `.worktrees/`, and the remote branches `docs/game-design`, `test/weapon-behaviour` and `docs/own-pages-url` on `skymiku39/jadereaper`. Their code is on `main` (cherry-picked, so the first three do not show as merged); only their own handoff commits would be lost. Why: deleting branches needs approval. Blocks: nothing.
- **P-17** The user's answer on the four suggested mission types in `docs/design.md` 4.2.1 (break the formation, pursuit, retrieval, arena waves). Blocks: nothing until item 6 of `Ranked work`.

Closed on 2026-11-16: P-02 (roadmap issues) is withdrawn, since issues are off and `ROADMAP.md` is the plan; P-10 is obsolete, the backup branches no longer exist; P-12 is obsolete, that change merged upstream as #26; P-14 is done, `main` on the user's repository is at 82e268d.
