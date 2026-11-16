//! Prints the state hash of a scripted run, one line per seed:
//!
//!     cargo run --release --manifest-path core/Cargo.toml --example scripted_run -- <ticks> <seed>...
//!
//! `tools/determinism.mjs` compares the output with the wasm build.

use jadereaper_core::game::scripted_run;

fn main() {
    let mut args = std::env::args().skip(1);
    let ticks: u32 = args.next().and_then(|a| a.parse().ok()).expect("usage: scripted_run <ticks> <seed>...");
    for arg in args {
        let seed: u32 = arg.parse().expect("seed must be a number");
        println!("{seed} {}", scripted_run(seed, ticks));
    }
}
