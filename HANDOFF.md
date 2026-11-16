# Handoff

The repository is the user's one-person version of the game (decided 2026-11-15 and 2026-11-16). The goal is to build the user's own design in `docs/design.md`; nobody else signs work off. Remotes: `origin` is the user's repository `skymiku39/jadereaper`, `upstream` is `pomodorozhong/jadereaper` (read-only for this account; do not open issues or PRs there unless asked).

Run git, `cargo` and `./build.sh` inside WSL only. Windows git on the `\\wsl.localhost` path writes `.git` files as root and breaks later commits.

## In progress

Environment: `gh` is not installed inside WSL, so use the Windows one with `--repo skymiku39/jadereaper`. Node is 24 locally against the 26 the repository asks for; everything passes on 24. The smoke test runs locally.

### feat/fixed-step

- Task: the first slice of item 1 of `Ranked work`: a fixed time step and a deterministic core. Replays and the balance tool are later slices.
- PR: https://github.com/skymiku39/jadereaper/pull/3, based on `main`. The user approved pushing it and a rebase merge once CI passes, on 2026-11-16 (P-19).
- Code checkpoint: see `git log`; the branch is cut from `main` at ef354f1 (PR 1 and PR 2 merged, all four workflows green there).
- Changed: `Game::update(dt)` now collects real time and advances in whole steps of `TICK` (1/60 s) through the new public `Game::tick()`, then `Game::present()` rebuilds the instances and HUD state. A frame time within 12% of one step counts as exactly one. New `Game::hash()`, `scripted_run()` and the exports `jr_ticks`, `jr_state_hash` and `jr_scripted_run`. New `core/src/fmath.rs` replaces every standard `sin`, `cos`, `exp`, `powf`, `powi` and `atan2` in the core, because the native and wasm maths libraries disagreed and the same run diverged. New `tools/determinism.mjs` (`npm run determinism`) and `core/examples/scripted_run.rs` compare a four-minute scripted run on both targets for four seeds; the `Core` workflow runs it.
- Verification on 2026-11-16 (Rust 1.100.0, Node 24.22.3, Linux x86-64 under WSL): format, clippy (native and wasm), `cargo test` (49 unit tests, 16 content tests), `./build.sh` (wasm is now 63,017 bytes), `npm run determinism` (all four seeds match) and `npm run smoke` passed. A ten-second headless play measured 60.0 steps per second, the run clock matched real time, and there were no console errors. Not verified: how it looks. Headless screenshots show the HUD but a blank canvas, so nobody has watched the game move since this change; the user should play it once. Also not tried: displays faster than 60 Hz, where motion advances on some frames only.
- Next action: merge PR 3 once CI passes, then the replay slice. The user has still to play the game once and say whether it looks right.

## Ranked work

The working order agreed with the user on 2026-11-16. Details of each roadmap step are in `ROADMAP.md`; mission types are in `docs/design.md`, section 4.2.1.

1. Roadmap step 3 in three slices: the fixed time step (on `feat/fixed-step`), then input recording with replay playback, then the headless balance tool.
2. Mission framework: objective, win condition, result screen, mission picker on the title screen. First mission: survive a time.
3. One or two more mission prototypes (kill a count, then the user's pick).
4. Data-driven content rewrite (roadmap step 1), then martial arts × magic elements and reactions. Needs the user's answers first: what each of the four elements does, which pairs react, and how many martial arts can be held at once (design document, open items 2 and 4).
5. Remaining mission prototypes, the game feel pass (roadmap step 2), then the rest of roadmap step 4.
6. English UI fonts to match the Chinese set.
7. Touch controls and a WebGL2 fallback (roadmap step 6); scaling to thousands of enemies (roadmap step 7).

Done: the rest of CI (roadmap step 5), merged as PR 2 on 2026-11-16.

## Pending

- **P-16** Delete the old local branches `docs/game-design`, `feat/content-generator`, `test/weapon-behaviour`, `integrate/own-version` and `docs/own-pages-url`, the worktrees under `.worktrees/`, and the remote branches `docs/game-design`, `test/weapon-behaviour` and `docs/own-pages-url` on `skymiku39/jadereaper`. Their code is on `main` (cherry-picked, so the first three do not show as merged); only their own handoff commits would be lost. Why: deleting branches needs approval. Blocks: nothing.
- **P-17** The user's answer on the four suggested mission types in `docs/design.md` 4.2.1 (break the formation, pursuit, retrieval, arena waves). Blocks: nothing until item 5 of `Ranked work`.

Closed on 2026-11-16: P-02 (roadmap issues) is withdrawn, since issues are off and `ROADMAP.md` is the plan; P-10 is obsolete, the backup branches no longer exist; P-12 is obsolete, that change merged upstream as #26; P-14 is done, `main` on the user's repository is at 82e268d; P-15 is done, `chore/solo-housekeeping` merged as PR 1.
