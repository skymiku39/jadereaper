# Docs

Project overview, controls, layout and how to extend the game are in the root [README.md](../README.md). The plan is in [ROADMAP.md](../ROADMAP.md). Settled design, plan and tooling choices are in [decisions/](decisions/README.md), and how to propose work is in [CONTRIBUTING.md](../CONTRIBUTING.md).

## Gotchas

- **WebGPU needs a secure origin.** Open the game through `http://localhost` or `https`. Opening `index.html` from disk fails, and the game shows its "Cannot start" screen.
- **No `strip = true` in `core/Cargo.toml`.** On this machine's Rust toolchain, `rust-objcopy` cannot find `libLLVM.dylib`, so stripping fails during the build. The release profile leaves it out; the wasm is 69 KB without it.
- **The built wasm is committed.** After changing anything in `core/`, run `./build.sh` and commit `wasm/jadereaper_core.wasm` with the source change, or the browser keeps running the old core.
- **Bundled fonts only hold the characters the game shows.** After adding or changing Chinese text in `src/` or `index.html`, run `tools/subset-fonts.sh` and commit `assets/fonts/`. Otherwise the font check in CI fails, and new characters fall back to a system font in the middle of a line.
- **The numbers font is picked by character.** `JR Numerals` in `style.css` applies Caveat Brush to digits and `+ % : , . ×` through `unicode-range`, which must match `NUMERALS` in `tools/fonts.mjs`. Caveat Brush has no `→`, so arrows use the text font.
- **The Pages site only holds listed paths.** `.github/workflows/pages.yml` copies `index.html`, `style.css`, `src/`, `wasm/` and `assets/`. A new top-level file or folder the game loads must be added to its copy step, or it works locally and 404s on https://pomodorozhong.github.io/jadereaper/. `PUBLISHED` in `tools/serve.mjs` must list the same paths, so `npm start` and the smoke test 404 on a file the deploy would miss. The site is served from `/jadereaper/`, so keep every path the game loads relative.
- **Ids are generated, not hand-written.** Part, upgrade and event ids, upgrade max levels, state buffer indices and the ink colour live in `content/game.toml`. `tools/gen` writes them to `core/src/generated.rs`, `src/generated.js` and the marked block at the top of `src/shader.wgsl`; `./build.sh` runs it first. Never edit those outputs by hand, and commit them with the content file, because the Pages deploy builds nothing. `cargo test` fails if they are stale or if the content has a duplicate or undefined id. The generator is its own crate so it still runs when the core does not compile yet.
- **Wasm memory views go stale.** `src/main.js` rebuilds its `Float32Array` views of wasm memory on every read, because memory growth detaches old views. Keep that pattern when reading new buffers.
- **Balance numbers** in `core/src/defs.rs` (`KINDS`) and `core/src/game.rs` (`xp_for`, spawn rate, weapon stats) were tuned only against a bot that walks in circles. Treat them as placeholders.
