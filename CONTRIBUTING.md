# Contributing

This repository is @skymiku39's own version of Jade Reaper. For now it is a one-person project: the goal is to build the maintainer's game design, recorded in [docs/design.md](docs/design.md), and the maintainer alone decides what gets built. It applies the same way to AI coding agents working for the maintainer.

## The flow

1. **Pick work from the plan.** The order is in [ROADMAP.md](ROADMAP.md) and the current state is in `HANDOFF.md`. No issue or proposal is needed; issues are turned off on this repository.
2. **One branch per piece of work,** cut from `main`. Keep it to one reason to change. If the work turns out bigger than planned, split it rather than growing the branch.
3. **Open a PR on this repository** so the CI checks run, then merge it once they pass. Pushing and merging need the maintainer's go-ahead.

Design questions go to the maintainer directly. Do not settle them yourself, and do not hold work for anyone else's sign-off.

## Decisions

Choices that shape later work, such as the toolchain or a design pillar, are made by the maintainer and recorded in [docs/decisions/](docs/decisions/README.md). What the game should be is recorded in [docs/design.md](docs/design.md). A choice in neither place is still open, so ask.

## Before you open a PR

Run the checks in [AGENTS.md](AGENTS.md) and list in the PR which ones you ran, what they showed, and which ones you skipped and why. Read [docs/README.md](docs/README.md) for the gotchas, for example that the built wasm and the bundled fonts are committed.

## Unused for now

The issue forms in `.github/ISSUE_TEMPLATE/` and the labels in [.github/labels.md](.github/labels.md) describe an issue-first process with `accepted` proposals. They are kept for when the project opens to other contributors, and nothing here depends on them today.
