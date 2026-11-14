# Handoff

## In progress

### integrate/own-version

- Task: start the user's own version of the game on the fork `skymiku39/jadereaper` (remote `fork`), instead of contributing to `pomodorozhong/jadereaper` (remote `origin`). The user decided this on 2026-11-15. This branch starts from `origin/main` at a0d53d1 and adds three commits cherry-picked from earlier local branches: the game design document and roadmap steps 1 and 4 (from `docs/game-design`, the closed #6), the weapon and upgrade behaviour tests (from `test/weapon-behaviour`, the closed #13), and the content file with its id generator (from `feat/content-generator`, never pushed). Not linked to any issue.
- PR: none. On 2026-11-15 the user approved pushing this branch to the fork as its `main` by fast-forward (the old P-13). `fork/main` and the local `main` now point at this branch's tip, and the local `main` tracks `fork/main`.
- Code checkpoint: 8fe6834 (`feat(build): generate ids from a single content file`), on `origin/main` at a0d53d1.
- Conflicts resolved in the last cherry-pick: `README.md` (kept the upstream `tools/` row and added the `tools/gen/` row) and `docs/README.md` (kept the upstream gotchas and replaced "Ids are mirrored by hand" with "Ids are generated").
- Verification at 8fe6834 (Rust 1.100, Linux x86-64 under WSL): `cargo test --release --manifest-path core/Cargo.toml` passed, 42 unit tests and 16 content tests; `./build.sh` passed and left the committed wasm and generated files unchanged; `node --check` passed on every `src/*.js`. Not run: `tools/check-fonts.sh` and `npm run smoke` (`node_modules/` is not installed here).
- Still to decide for the user's own version: whether `CONTRIBUTING.md`, the issue forms, the label files and the Proposal rule in `AGENTS.md` and below still apply on the fork (they describe the upstream maintainer's process); whether to rename the remotes so `origin` is the fork; and whether new work on the fork goes through PRs there. The fork has issues turned off, and its Pages deploy has not been set up, so the `Pages` workflow may fail on the fork until it is. `ROADMAP.md` "Current state" still says ids are mirrored by hand.
- Upstream: the Proposals #28 and #29 on `pomodorozhong/jadereaper` stay open by the user's choice. They have no labels, because this account cannot set labels there.
- Run git inside WSL only. Running Windows git on the `\\wsl.localhost` path writes `.git` files as root and breaks later commits.
- Note: the `ci/smoke-comment` subsection below is stale; its PR merged upstream as #26. The local branches `docs/game-design`, `test/weapon-behaviour` and `feat/content-generator` and their worktrees are now contained in this branch, apart from their own handoff commits.
- Next action: ask the user about the open choices above, then pick new work from `Ranked work`. Take `gh` commands to the fork with `--repo skymiku39/jadereaper`; without it `gh` targets the upstream.

### ci/smoke-comment

- Task: correct the comments at the top of `.github/workflows/smoke.yml` and `tools/smoke.mjs`, which said CI usually has no WebGPU and checks the "Cannot start" screen. CI runs showed the opposite: Chromium finds a software adapter on the Ubuntu runner and plays a few seconds of a run (answered on #17, now closed). A comment-only correction, which `CONTRIBUTING.md` lets skip an issue. Local branch only, not linked to any issue. It also carries this handoff refresh after #20, #23 and #24 merged.
- PR: none yet.
- Code checkpoint: 9c01200 (`ci: say which path the smoke test takes on the runner`), on `main` at f739727.
- Verification at 9c01200: `node --check tools/smoke.mjs` passes and `smoke.yml` parses (PyYAML via uvx). Not run: `npm run smoke`, `cargo test`, `./build.sh` (comments only, no behaviour change); the `Smoke` job runs on the PR.
- Next action: P-12, then open the PR with the `open-pr` skill.
- Pending: **P-12** push this branch and open a PR for it. Why: pushing and opening a PR need approval. Blocks: the corrected comments and this handoff reaching `main`.

## Ranked work

Roadmap steps are detailed in `ROADMAP.md`. They become issues once P-02 is approved.

1. Roadmap step 5, the rest of CI: the wasm drift check, cargo fmt and clippy, and a JavaScript check. The Pages deploy (#5) and the smoke test (#24) are done. Planned on the local branch `ci/core-checks`, which needs an `accepted` Proposal first (see that branch's own `HANDOFF.md`)
2. Decide which areas need the maintainer's sign-off before contributors build on them, made on an issue and recorded in `docs/decisions/`. Blocks #14 (`blocked`)
3. #3: pick English UI fonts to match the Chinese set (`accepted`; can run alongside item 1)
4. #16: run the Fonts workflow only when font tools change (`accepted`, small)
5. #18: smoke test connects to 127.0.0.1 instead of localhost (`accepted`, small)
6. Roadmap step 1: data-driven content and puppet rigs (unblocks most later steps)
7. Roadmap step 2: game feel pass (can run alongside step 3)
8. Roadmap step 3: fixed time step, replays and a headless balance tool
9. Roadmap step 4: run structure and meta progression (after step 1)
10. Roadmap step 6: touch controls and a WebGL2 fallback
11. Roadmap step 7: scale to thousands of enemies

## Pending

- **P-02** File one GitHub issue per roadmap step (seven issues, titles as in `ROADMAP.md`, body copied from each step's section, step 1 to 4 in a "Playable game" milestone). Under `CONTRIBUTING.md` they would go in as Proposals, labelled `enhancement` and `proposal`. Why: filing issues needs approval. Blocks: nothing else; `ROADMAP.md` covers the details until then.
- **P-10** Delete the local backup branches `backup/contribution-flow-v1` to `-v7`, earlier rebuilds of the merged #20. Why: they hold older drafts not in any surviving branch. Blocks: nothing.
