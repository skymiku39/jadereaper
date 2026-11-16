# Handoff

The repository is the user's one-person version of the game (decided 2026-11-15 and 2026-11-16). The goal is to build the user's own design in `docs/design.md`; nobody else signs work off. Remotes: `origin` is the user's repository `skymiku39/jadereaper`, `upstream` is `pomodorozhong/jadereaper` (read-only for this account; do not open issues or PRs there unless asked).

Run git, `cargo` and `./build.sh` inside WSL only. Windows git on the `\\wsl.localhost` path writes `.git` files as root and breaks later commits.

## In progress

Environment: `gh` is not installed inside WSL, so use the Windows one with `--repo skymiku39/jadereaper`. Node is 24 locally against the 26 the repository asks for; everything passes on 24. The smoke test runs locally.

### feat/mission-survive

- Task: item 1 of `Ranked work`: the mission framework and the first mission, "survive a time". The user asked on 2026-11-16 to do this before replays and the balance tool.
- PR: none. Not pushed.
- Code checkpoint: see `git log`; the branch is cut from `main` at 802359b (PR 3, the fixed time step, merged there).
- Changed: `content/game.toml` gains a `[[mission]]` table (`endless`, and `survive` with a goal of 300 s), the event `win` and the state fields `mission`, `goal`, `progress` and `won`; `tools/gen` reads the table and writes `M_*`, `N_MISSIONS` and `MISSION_GOAL` for Rust and `MISSIONS` and `MISSION_IDS` for JavaScript. In the core, `jr_start(seed, mission)` starts a mission, `Game::mission_won` holds each mission's win rule and the run stops when `won` is set. In the host, the title screen builds one start button per mission, the HUD counts down in a timed mission, a new "mission complete" screen shows the result, and the save keeps tries and wins per mission (endless keeps its best time, kills and level). The Chinese fonts were re-subset for the new text.
- Verification on 2026-11-16 (Rust 1.100.0, Node 24.22.3, Linux x86-64 under WSL): format, clippy, `cargo test` (53 unit tests, 16 content tests), `./build.sh`, `tools/check-fonts.sh`, `npm run determinism` and `npm run smoke` passed. A headless play of the survive mission checked both endings: death shows the game-over screen with the mission named and records a try; with the goal temporarily set to 10 s, the win screen appears at 0:10, a win is saved and the title button then reads "已完成 1 次". Screenshots of the title and win screens were looked at. Not verified: a person playing the full five minutes, the game view itself (headless screenshots show a blank canvas), gamepad navigation of the new buttons, and the English text on screen.
- Finding for the user: the survive mission can be won by only running away. In a native simulation of 40 seeds, a bot that moves away from nearby enemies and always takes the first card won 40 of 40 at a mean level of 1.4; a bot walking in circles won 0 of 40 and lasted 102 s on average. The arena has no edge and enemies are slower than the player. The mission needs pressure that running cannot escape (a bounded arena, spawns that surround, faster chasers); recorded in `docs/design.md` 4.2.1 as open.
- Still open from PR 3: the user has not yet said whether the game looks right at the fixed step.
- Next action: P-20, then the user's call on the finding above, then item 2 of `Ranked work`.
- Pending: **P-20** push this branch to `origin`, open a PR and merge it once CI passes. Why: pushing and merging need the user's go-ahead.

## Ranked work

The working order agreed with the user on 2026-11-16, with missions moved ahead of replays the same day. Details of each roadmap step are in `ROADMAP.md`; mission types are in `docs/design.md`, section 4.2.1.

1. Mission framework and the first mission, survive a time (on `feat/mission-survive`).
2. One or two more mission prototypes (kill a count, then the user's pick).
3. The rest of roadmap step 3: input recording with replay playback, then the headless balance tool. The fixed time step is done (PR 3).
4. Data-driven content rewrite (roadmap step 1), then martial arts × magic elements and reactions. Needs the user's answers first: what each of the four elements does, which pairs react, and how many martial arts can be held at once (design document, open items 2 and 4).
5. Remaining mission prototypes, the game feel pass (roadmap step 2), then the rest of roadmap step 4.
6. English UI fonts to match the Chinese set.
7. Touch controls and a WebGL2 fallback (roadmap step 6); scaling to thousands of enemies (roadmap step 7).

Done: the rest of CI (roadmap step 5), merged as PR 2, and the fixed time step, merged as PR 3, both on 2026-11-16.

## Pending

- **P-16** Delete the old local branches `docs/game-design`, `feat/content-generator`, `test/weapon-behaviour`, `integrate/own-version` and `docs/own-pages-url`, the worktrees under `.worktrees/`, and the remote branches `docs/game-design`, `test/weapon-behaviour` and `docs/own-pages-url` on `skymiku39/jadereaper`. Their code is on `main` (cherry-picked, so the first three do not show as merged); only their own handoff commits would be lost. Why: deleting branches needs approval. Blocks: nothing.
- **P-17** The user's answer on the four suggested mission types in `docs/design.md` 4.2.1 (break the formation, pursuit, retrieval, arena waves). Blocks: nothing until item 5 of `Ranked work`.

Closed on 2026-11-16: P-02 (roadmap issues) is withdrawn, since issues are off and `ROADMAP.md` is the plan; P-10 is obsolete, the backup branches no longer exist; P-12 is obsolete, that change merged upstream as #26; P-14 is done, `main` on the user's repository is at 82e268d; P-15 is done, `chore/solo-housekeeping` merged as PR 1.
