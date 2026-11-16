//! Writes the id constants shared by the core, the host and the shader from
//! `content/game.toml`:
//!
//! - `core/src/generated.rs`
//! - `src/generated.js`
//! - the generated block in `src/shader.wgsl`
//!
//! Run from anywhere: `cargo run --release --manifest-path tools/gen/Cargo.toml`.
//! With `--check` it writes nothing and fails if a file is out of date.

mod content;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let check = std::env::args().skip(1).any(|a| a == "--check");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    match run(&root, check) {
        Ok(stale) if check && stale > 0 => {
            eprintln!("gen: {stale} generated file(s) out of date; run ./build.sh");
            ExitCode::FAILURE
        }
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("gen: {e}");
            ExitCode::FAILURE
        }
    }
}

fn read(root: &Path, rel: &str) -> Result<String, String> {
    std::fs::read_to_string(root.join(rel)).map_err(|e| format!("{rel}: {e}"))
}

/// Returns how many files differ from what was generated.
fn run(root: &Path, check: bool) -> Result<usize, String> {
    let src = read(root, content::CONTENT_PATH)?;
    let c = content::load(&src)
        .map_err(|errs| format!("{} has problems:\n  {}", content::CONTENT_PATH, errs.join("\n  ")))?;
    let shader = read(root, content::WGSL_PATH)?;
    let outputs = [
        (content::RUST_OUT, content::render_rust(&c)),
        (content::JS_OUT, content::render_js(&c)),
        (content::WGSL_PATH, content::splice_wgsl(&shader, &c)?),
    ];
    let mut stale = 0;
    for (rel, text) in outputs {
        let old = std::fs::read_to_string(root.join(rel)).unwrap_or_default();
        if old == text {
            println!("gen: {rel} unchanged");
            continue;
        }
        stale += 1;
        if check {
            println!("gen: {rel} is out of date");
        } else {
            std::fs::write(root.join(rel), text).map_err(|e| format!("{rel}: {e}"))?;
            println!("gen: wrote {rel}");
        }
    }
    Ok(stale)
}
