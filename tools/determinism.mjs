// Determinism check: the same scripted run must end in the same state hash in
// the native build and in the committed wasm.
//
//     npm run determinism
//
// Needs Rust (for the native side). A mismatch means some float operation
// gives different results on the two targets; see docs/README.md.

import { execFileSync } from 'node:child_process';
import { readFile } from 'node:fs/promises';

const TICKS = 60 * 240; // four minutes of play, past the first two elites
const SEEDS = [1, 7, 12345, 987654321];

const root = new URL('..', import.meta.url);
const bytes = await readFile(new URL('wasm/jadereaper_core.wasm', root));
const core = (await WebAssembly.instantiate(bytes, {})).instance.exports;

const native = execFileSync(
  'cargo',
  ['run', '--quiet', '--release', '--manifest-path', 'core/Cargo.toml', '--example', 'scripted_run', '--', String(TICKS), ...SEEDS.map(String)],
  { cwd: root, encoding: 'utf8' },
);
const nativeHash = new Map(native.trim().split('\n').map((line) => line.split(' ').map(Number)));

let failed = 0;
for (const seed of SEEDS) {
  const wasm = core.jr_scripted_run(seed, TICKS) >>> 0;
  const same = wasm === nativeHash.get(seed);
  if (!same) failed++;
  console.log(`determinism: seed ${seed}: wasm ${wasm}, native ${nativeHash.get(seed)}${same ? '' : '  MISMATCH'}`);
}
if (failed) {
  console.error(`determinism: failed, ${failed} of ${SEEDS.length} seeds differ`);
  process.exitCode = 1;
} else {
  console.log('determinism: ok');
}
