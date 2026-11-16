# Handoff

The repository is the user's one-person version of the game (decided 2026-11-15 and 2026-11-16). The goal is to build the user's own design in `docs/design.md`; nobody else signs work off. Remotes: `origin` is the user's repository `skymiku39/jadereaper`, `upstream` is `pomodorozhong/jadereaper` (read-only for this account; do not open issues or PRs there unless asked).

Run git, `cargo` and `./build.sh` inside WSL only. Windows git on the `\\wsl.localhost` path writes `.git` files as root and breaks later commits.

## In progress

Environment: `gh` is not installed inside WSL, so use the Windows one with `--repo skymiku39/jadereaper`. Node is 24 locally against the 26 the repository asks for; everything passes on 24. The smoke test runs locally.

### docs/plan-refresh

Cut from `main` at 1221228 on 2026-11-16 to bring the plan files up to date after PR 6 (the user said "更新文件"). It holds this file's edits since PR 6 merged and one corrected line in `ROADMAP.md`'s working order. Both are committed here (the user said "提交") and not pushed; pushing and opening a PR need the user's word. No code changed, so no checks were run.

The `handoff` skill that `AGENTS.md` points to (`references/working-rules.md`) is not in the repository, its history, or the user's skill folders, so this branch followed only the rules written in `AGENTS.md`.

Pushed to `origin` on 2026-11-16 (the user said "推"); no PR is open.

### feat/mission-tuning

A setup screen before every mission, where a tester changes the mission's numbers (the user asked for it on 2026-11-16 after playing: distances too far, counts too high or low). Cut from `docs/plan-refresh` at 771b494, so it carries that branch's two commits; merging this one brings them along. Committed, pushed and opened as a PR on 2026-11-16 (the user chose "Create PR"); merging it needs the user's word.

- **How it works:** a number that can be changed is a `[[tune]]` in `content/game.toml` (42 of them: 3 to 5 per mission, and `spawn_rate`, `enemy_speed` and `enemy_hp` for every mission). The generator writes `T_*`, `TUNE_DEFAULT`, `TUNE_MIN`, `TUNE_MAX` and `MISSION_GOAL_TUNE` for Rust and `TUNES` for JavaScript. `Game.tune` holds a run's values; `jr_tune` and `jr_tune_reset` set them for the next `jr_start`. The mission files read `self.tune[...]` where they had constants. A title button now opens `#setup` (`setup` in `src/ui.js`, `openSetup`, `stepTune` and `startRun` in `src/main.js`); values are kept in the save as `tuning`.
- **Decided here, not by the user:** a run with any tune off its default is a test run and is not recorded (no win, try or endless record). Which numbers are tunes and their ranges are a first pick.
- **Checked:** `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` on `core` and `tools/gen`, `cargo test --release` (169 passed, 5 ignored; 18 content tests), `./build.sh` (a second run changes nothing), `npm run fonts:subset` and `fonts:check`, `npm run determinism`, `npm run smoke` (it now goes through the setup screen). The recorded whole-run hash test passes, so default runs play as before. A throwaway script in headless Chromium opened the `escape` setup screen in Chinese at 1280x800, changed two values, started the run (the HUD state showed the changed goal), reopened the screen (the values were kept), reset them and went back with Esc: no console errors.
- **Not checked:** nobody has played a run with changed values, so whether each tune does what its label says in play is only covered by the unit tests. Gamepad navigation of the steppers, the English labels on screen, narrow screens, and a setup screen taller than the window were not tried. The mission files were changed by a subagent; the non-test part of its diff was read, its new tests were not.

Other work starts on a new branch cut from `main`.

Merged on 2026-11-16 by rebase as PR 6 (the user said "合併部署，並且我要試玩"): `feat/missions-all`, all nine remaining mission types from `docs/design.md` 4.2.1. Its three CI checks passed before the merge. The Pages deploy of `main` succeeded and https://skymiku39.github.io/jadereaper/ serves the twelve missions; the Core, Smoke and Fonts runs on `main` at 1221228 passed as well. The branch is deleted locally and on `origin`. The user is about to play the missions; their findings feed P-29 and P-30. What follows describes that work.

How it came about (2026-11-16): the user said to build every mission in parallel ("每一個任務都要做，但是要記得幫我分工並且同時做", then "五種也一起做"), so nine subagents each built one mission in its own worktree and branch, and on "提交並合併" each was committed there and all were combined here. The combination is one commit rather than nine merges, because the branches collided in the same lines and the merges in between would not have built.

- **Missions and ids:** `formation` 3, `arena` 4, `retrieval` 5, `pursuit` 6, `escort` 7, `hold` 8, `escape` 9, `boss` 10, `vendetta` 11. Each mission's rules, numbers, drawing and tests are in `core/src/mission_<name>.rs`.
- **Changed while combining:** one `spawn_enemy_at` instead of three variants; `move_enemies` takes the escort, hold, boss and vendetta steering in one pass; `hud` and `gameOver` in `src/ui.js` rewritten to take all missions (`gameOver` now takes the text key of the reason); `state_floats` is 60; the generator marks `MISSION_GOAL` `#[rustfmt::skip]` because the line outgrew rustfmt's width; the title screen shows the mission buttons in two columns and scrolls, because twelve buttons no longer fitted and the lower ones could not be clicked.
- **Left uneven:** `formation`, `arena`, `retrieval`, `hold` and `escape` are declared in `core/src/lib.rs`; `pursuit`, `escort`, `boss` and `vendetta` from `game.rs` with `#[path]`.
- **Checked on this branch:** `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` on `core`, `cargo test --release` (151 passed, 5 ignored bot reports; 16 content tests), fmt and clippy on `tools/gen`, `./build.sh` (a second run changes nothing), `npm run fonts:subset` and `fonts:check`, `npm run determinism`, `npm run smoke`. The recorded whole-run hash test still passes, so endless, survive and kills play as before. A throwaway script started each of the twelve missions from the title screen in headless Chromium for four seconds: no console or page errors, and each HUD showed its own text.
- **Not checked:** nobody has played any of the nine or seen the game view (headless screenshots show a blank canvas; only the title screen was looked at, in Chinese at 1280x720). Gamepad and keyboard navigation of the two-column title, the English text, and narrow screens were not tried. `npm run determinism` and `npm run smoke` still only play the default run. No win or loss screen of the new missions was reached in a browser.
- **Bot findings, from the subagents and not re-run:** `escape` is won 9 times in 12 by a bot that only dodges, and no spawn tuning stopped it; `retrieval` is won 7 in 8 by a dodging bot; `pursuit` lasts about 3 minutes, not 5 to 10; `vendetta`'s fight was lost 12 in 12 by a bot that does not level up on the way; `arena` wave 3 to 4 killed the simple fighter bot; `hold` and `escort` sit on the edge (5 of 8 and 3 of 6 for bots that guard); `boss` health was doubled to 12000 after its last bot run.
- **The nine source branches** `feat/mission-<name>` and their worktrees `.worktrees/mission-<name>` still exist, one commit each on top of 9bde2d0. Their content is all in this branch; they will never show as merged.

Merged on 2026-11-16, both by rebase: `feat/mission-survive` as PR 4 and `feat/mission-kills` as PR 5 (the user said "推送並合併").

- PR 5's first CI run failed in the smoke test: the HUD never appeared. The likely cause is that the test clicked a start button inside the 300 ms input lock a screen has when it appears, so the click was dropped; the test now waits 500 ms first, and the next run passed. One pass does not prove the cause, so watch for the same failure.
- Not verified for either mission: a person playing it through (five minutes, or 1500 kills), the game view itself (headless screenshots show a blank canvas), gamepad navigation of the mission buttons, and the English text on screen. Death in the kill-count mission was not played; it shares the survive mission's path.
- The kill-count goal of 1500 comes from a throwaway simulation that is not in the repository (40 seeds per bot, always the first card, 15 minutes at most): a bot that closes in and backs off when hurt won 15 of 40 and reaches 1500 kills at about five minutes; one that only backs away from near enemies won 4; one walking in circles won 0.
- The user asked on 2026-11-16 whether each way of playing should be its own repository. The answer given was to keep one repository (the missions share the core, renderer, UI and CI, and the story chapter chains them); the user did not reply and went on with this repository.

## Ranked work

The working order agreed with the user on 2026-11-16, with missions moved ahead of replays the same day. Details of each roadmap step are in `ROADMAP.md`; mission types are in `docs/design.md`, section 4.2.1.

1. Mission prototypes from `docs/design.md` 4.2.1: all built and merged as PR 6. Next for them is the user playing them and the decisions in `Pending` (P-29, P-30).
2. The rest of roadmap step 3: input recording with replay playback, then the headless balance tool. The fixed time step is done (PR 3).
3. Data-driven content rewrite (roadmap step 1), then martial arts × magic elements and reactions. Needs the user's answers first: what each of the four elements does, which pairs react, and how many martial arts can be held at once (design document, open items 2 and 4).
4. The game feel pass (roadmap step 2), then the rest of roadmap step 4. A determinism check and a smoke test that play the new missions belong here or earlier.
5. English UI fonts to match the Chinese set.
6. Touch controls and a WebGL2 fallback (roadmap step 6); scaling to thousands of enemies (roadmap step 7).

Done: the rest of CI (roadmap step 5), merged as PR 2, and the fixed time step, merged as PR 3, and the mission framework with the survive mission, merged as PR 4, and the kill-count mission, merged as PR 5, and the nine other mission prototypes, merged as PR 6, all on 2026-11-16.

## Pending

- **P-16** Delete the old local branches `docs/game-design`, `feat/content-generator`, `test/weapon-behaviour`, `integrate/own-version` and `docs/own-pages-url`, the worktrees under `.worktrees/`, and the remote branches `docs/game-design`, `test/weapon-behaviour` and `docs/own-pages-url` on `skymiku39/jadereaper`. Their code is on `main` (cherry-picked, so the first three do not show as merged); only their own handoff commits would be lost. Why: deleting branches needs approval. Blocks: nothing.
- **P-21** The user's call on the survive mission: it can be won by only running away. In a native simulation of 40 seeds, a bot that moves away from nearby enemies and always takes the first card won 40 of 40 at a mean level of 1.4; a bot walking in circles won 0 of 40 and lasted 102 s on average. The arena has no edge and enemies are slower than the player. It needs pressure that running cannot escape (a bounded arena, spawns that surround, faster chasers); open in `docs/design.md` 4.2.1. Blocks: nothing, but it shapes the next mission prototypes.
- **P-22** The user has not yet said whether the game looks right at the fixed step (PR 3). Blocks: nothing.
- **P-32** Delete the nine local branches `feat/mission-<name>` and their worktrees, now that PR 6 is merged. They were never pushed. They were combined by hand, so git will not report them as merged and `git branch -d` will refuse. Why: deleting unmerged-looking branches needs approval. Blocks: nothing.
- **P-29** What makes the encirclement in `escape` real, since enemies neither block nor outrun the player: guards standing at the exit, a slowdown inside a crowd, or a higher enemy cap. Same root as P-21. Blocks: `escape` being a mission at all.
- **P-30** The user's call on the prototype numbers of the nine missions, above all `pursuit`'s three-minute length and whether `retrieval` may be won without fighting. Blocks: nothing.
- **P-25** The user's call on the kill-count goal of 1500 and on whether that mission should have a time limit; open in `docs/design.md` 4.2.1. Blocks: nothing.

Closed on 2026-11-16: P-31 is done, PR 6 merged at the user's word; P-28 is done, the user said "提交並合併" and the nine missions are committed and combined on `feat/missions-all`; P-27 is answered, the user said the five decided types are built too; P-17 and P-26 are answered, the user said to build every mission, so the four suggested types are accepted and all are being built; P-23 is done, the user said to delete the merged branches, so `chore/solo-housekeeping`, `ci/core-checks`, `feat/fixed-step`, `feat/mission-survive` and `feat/mission-kills` are gone locally and on `origin` (the user's standing preference is to delete a branch once it merges, without asking); P-24 is done, PR 5 merged at the user's word; P-20 is done, PR 4 merged at the user's word ("通過後合併"); P-02 (roadmap issues) is withdrawn, since issues are off and `ROADMAP.md` is the plan; P-10 is obsolete, the backup branches no longer exist; P-12 is obsolete, that change merged upstream as #26; P-14 is done, `main` on the user's repository is at 82e268d; P-15 is done, `chore/solo-housekeeping` merged as PR 1.
