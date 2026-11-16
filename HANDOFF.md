# Handoff

The repository is the user's one-person version of the game (decided 2026-11-15 and 2026-11-16). The goal is to build the user's own design in `docs/design.md`; nobody else signs work off. Remotes: `origin` is the user's repository `skymiku39/jadereaper`, `upstream` is `pomodorozhong/jadereaper` (read-only for this account; do not open issues or PRs there unless asked).

Run git, `cargo` and `./build.sh` inside WSL only. Windows git on the `\\wsl.localhost` path writes `.git` files as root and breaks later commits.

## In progress

Environment: `gh` is not installed inside WSL, so use the Windows one with `--repo skymiku39/jadereaper`. Node is 24 locally against the 26 the repository asks for; everything passes on 24. The smoke test runs locally.

`feat/mission-survive` merged as PR 4 on 2026-11-16 (rebase merge, `main` at 8f804b7). Not verified for PR 4: a person playing the full five minutes of the survive mission, the game view itself (headless screenshots show a blank canvas), gamepad navigation of the mission buttons, and the English text on screen.

### feat/mission-kills

- Task: item 1 of `Ranked work`, the kill-count mission prototype. The user said "開始擊殺數任務" on 2026-11-16.
- PR: none. Not pushed.
- Code checkpoint: see `git log`; the branch is cut from `main` at 8f804b7.
- Changed: `content/game.toml` gains the mission `kills` (id 2, goal 1500, no time limit); `Game::progress` and `Game::mission_won` in the core count kills for it. The HUD's kill tag reads "n / goal" in this mission; the title button and the result screen come from the framework. English name "Cull", Chinese "殺滿數量". The Chinese fonts were re-subset for 滿 and 隻. The duplicate-name content test now edits the state field `kills` by its `index` line, since a mission has the same name.
- Verification on 2026-11-16 (Rust 1.100.0, Node 24.22.3, Linux x86-64 under WSL): format, clippy, `cargo test` (55 unit tests, 16 content tests), `./build.sh` (a second run changes nothing), `npm run fonts:check`, `npm run determinism` and `npm run smoke` passed. A headless play with the goal temporarily set to 15: the title shows three buttons, the HUD reads "1 / 15", the win screen appears at 15 kills with the mission named and one win saved, and the title button then reads "已完成 1 次"; no console errors. The HUD screenshot was looked at. Not verified: a person playing to 1500, death in this mission (it shares the survive mission's path), the game view itself, gamepad navigation, and the English text on screen.
- Simulation behind the goal of 1500 (40 seeds per bot, always the first card, 15 minutes at most): a bot that closes in and backs off when hurt won 15 of 40 and reaches 1500 kills at about five minutes; one that only backs away from near enemies won 4; one walking in circles won 0. The simulation was a throwaway example and is not in the repository.
- The user asked mid-task whether each way of playing should be its own repository. The answer given was to keep one repository (the missions share the core, renderer, UI and CI, and the story chapter chains them); the user had not replied when this was written.
- Next action: P-24, then P-25.
- Pending: **P-24** push this branch to `origin`, open a PR and merge it once CI passes. Why: pushing and merging need the user's go-ahead.
- Pending: **P-25** the user's call on the goal of 1500 and on whether this mission should have a time limit; open in `docs/design.md` 4.2.1. Blocks: nothing.

## Ranked work

The working order agreed with the user on 2026-11-16, with missions moved ahead of replays the same day. Details of each roadmap step are in `ROADMAP.md`; mission types are in `docs/design.md`, section 4.2.1.

1. One or two more mission prototypes: kill a count (on `feat/mission-kills`), then the user's pick.
2. The rest of roadmap step 3: input recording with replay playback, then the headless balance tool. The fixed time step is done (PR 3).
3. Data-driven content rewrite (roadmap step 1), then martial arts × magic elements and reactions. Needs the user's answers first: what each of the four elements does, which pairs react, and how many martial arts can be held at once (design document, open items 2 and 4).
4. Remaining mission prototypes, the game feel pass (roadmap step 2), then the rest of roadmap step 4.
5. English UI fonts to match the Chinese set.
6. Touch controls and a WebGL2 fallback (roadmap step 6); scaling to thousands of enemies (roadmap step 7).

Done: the rest of CI (roadmap step 5), merged as PR 2, and the fixed time step, merged as PR 3, and the mission framework with the survive mission, merged as PR 4, all on 2026-11-16.

## Pending

- **P-16** Delete the old local branches `docs/game-design`, `feat/content-generator`, `test/weapon-behaviour`, `integrate/own-version` and `docs/own-pages-url`, the worktrees under `.worktrees/`, and the remote branches `docs/game-design`, `test/weapon-behaviour` and `docs/own-pages-url` on `skymiku39/jadereaper`. Their code is on `main` (cherry-picked, so the first three do not show as merged); only their own handoff commits would be lost. Why: deleting branches needs approval. Blocks: nothing.
- **P-17** The user's answer on the four suggested mission types in `docs/design.md` 4.2.1 (break the formation, pursuit, retrieval, arena waves). Blocks: nothing until item 4 of `Ranked work`.
- **P-21** The user's call on the survive mission: it can be won by only running away. In a native simulation of 40 seeds, a bot that moves away from nearby enemies and always takes the first card won 40 of 40 at a mean level of 1.4; a bot walking in circles won 0 of 40 and lasted 102 s on average. The arena has no edge and enemies are slower than the player. It needs pressure that running cannot escape (a bounded arena, spawns that surround, faster chasers); open in `docs/design.md` 4.2.1. Blocks: nothing, but it shapes the next mission prototypes.
- **P-22** The user has not yet said whether the game looks right at the fixed step (PR 3). Blocks: nothing.
- **P-23** Delete the merged branch `feat/mission-survive`, locally and on `origin`. Why: deleting branches needs approval. Blocks: nothing.

Closed on 2026-11-16: P-20 is done, PR 4 merged at the user's word ("通過後合併"); P-02 (roadmap issues) is withdrawn, since issues are off and `ROADMAP.md` is the plan; P-10 is obsolete, the backup branches no longer exist; P-12 is obsolete, that change merged upstream as #26; P-14 is done, `main` on the user's repository is at 82e268d; P-15 is done, `chore/solo-housekeeping` merged as PR 1.
