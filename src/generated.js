// Generated from content/game.toml by tools/gen. Do not edit: change the
// content file and run ./build.sh. Imported through src/data.js.

/** Part ids by name. A name is also the key for an image override in assets/parts.json. */
export const PART_IDS = {
  ellipse: 0,
  player_body: 1,
  player_head: 2,
  scythe: 3,
  blot: 4,
  wisp: 5,
  brute: 6,
  gem: 7,
  slash: 8,
  talisman: 9,
  bead: 10,
  splat: 11,
  grass: 12,
  stone: 13,
  heart: 14,
  ring: 15,
  scrap: 16,
  ground_shadow: 18,
};

/** Upgrades, indexed by id. `glyph` is the brush mark shown on cards and HUD chips.
 * Names and descriptions are in src/i18n.js as upgrade.<id>.name and .desc. */
export const UPGRADES = [
  { name: 'scythe', glyph: '鐮', max: 6 },
  { name: 'talisman', glyph: '符', max: 6 },
  { name: 'beads', glyph: '珠', max: 6 },
  { name: 'pulse', glyph: '波', max: 6 },
  { name: 'vitality', glyph: '命', max: 5 },
  { name: 'swift', glyph: '風', max: 5 },
  { name: 'lure', glyph: '引', max: 5 },
  { name: 'might', glyph: '力', max: 5 },
  { name: 'haste', glyph: '速', max: 5 },
  { name: 'mending', glyph: '癒', max: 5 },
];

/** Event types reported by the core. */
export const EVENT = {
  HIT: 1,
  KILL: 2,
  PICKUP: 3,
  LEVELUP: 4,
  HURT: 5,
  SLASH: 6,
  SHOOT: 7,
  PULSE: 8,
  DEATH: 9,
  HEAL: 10,
  ELITE: 11,
  WIN: 12,
};

/** Missions, indexed by id. `goal` is what the mission asks for (seconds for survive, enemies for kills).
 * Names and descriptions are in src/i18n.js as mission.<name>.name and .desc. */
export const MISSIONS = [
  { id: 0, name: 'endless', goal: 0 },
  { id: 1, name: 'survive', goal: 300 },
  { id: 2, name: 'kills', goal: 1500 },
  { id: 3, name: 'formation', goal: 5 },
  { id: 4, name: 'arena', goal: 10 },
  { id: 5, name: 'retrieval', goal: 3 },
  { id: 6, name: 'pursuit', goal: 3000 },
  { id: 7, name: 'escort', goal: 21000 },
  { id: 8, name: 'hold', goal: 300 },
  { id: 9, name: 'escape', goal: 18000 },
  { id: 10, name: 'boss', goal: 12000 },
  { id: 11, name: 'vendetta', goal: 6 },
];

/** Mission ids by name. */
export const MISSION_IDS = {
  endless: 0,
  survive: 1,
  kills: 2,
  formation: 3,
  arena: 4,
  retrieval: 5,
  pursuit: 6,
  escort: 7,
  hold: 8,
  escape: 9,
  boss: 10,
  vendetta: 11,
};

/** Tunes, indexed by id: numbers the setup screen can change for a test run. `mission` is the
 * mission that shows the tune, or 'any'. Labels are in src/i18n.js as tune.<name>. */
export const TUNES = [
  { id: 0, name: 'spawn_rate', mission: 'any', def: 1.0, min: 0.0, max: 4.0, step: 0.25 },
  { id: 1, name: 'enemy_speed', mission: 'any', def: 1.0, min: 0.5, max: 4.0, step: 0.25 },
  { id: 2, name: 'enemy_hp', mission: 'any', def: 1.0, min: 0.25, max: 4.0, step: 0.25 },
  { id: 3, name: 'survive_goal', mission: 'survive', def: 300, min: 30, max: 900, step: 30 },
  { id: 4, name: 'kills_goal', mission: 'kills', def: 1500, min: 100, max: 5000, step: 100 },
  { id: 5, name: 'formation_goal', mission: 'formation', def: 5, min: 1, max: 8, step: 1 },
  { id: 6, name: 'formation_node_hp', mission: 'formation', def: 2000, min: 250, max: 6000, step: 250 },
  { id: 7, name: 'formation_dist_min', mission: 'formation', def: 1300, min: 300, max: 6000, step: 100 },
  { id: 8, name: 'formation_dist_max', mission: 'formation', def: 2000, min: 300, max: 6000, step: 100 },
  { id: 9, name: 'formation_spawn_count', mission: 'formation', def: 3, min: 0, max: 8, step: 1 },
  { id: 10, name: 'arena_goal', mission: 'arena', def: 10, min: 1, max: 10, step: 1 },
  { id: 11, name: 'arena_size', mission: 'arena', def: 1.0, min: 0.25, max: 3.0, step: 0.25 },
  { id: 12, name: 'arena_hp_per_wave', mission: 'arena', def: 0.1, min: 0.0, max: 0.5, step: 0.05 },
  { id: 13, name: 'retrieval_goal', mission: 'retrieval', def: 3, min: 1, max: 6, step: 1 },
  { id: 14, name: 'retrieval_dist_min', mission: 'retrieval', def: 5000, min: 500, max: 12000, step: 500 },
  { id: 15, name: 'retrieval_dist_max', mission: 'retrieval', def: 8000, min: 500, max: 12000, step: 500 },
  { id: 16, name: 'retrieval_slow', mission: 'retrieval', def: 0.12, min: 0.0, max: 0.3, step: 0.02 },
  { id: 17, name: 'pursuit_goal', mission: 'pursuit', def: 3000, min: 500, max: 12000, step: 500 },
  { id: 18, name: 'pursuit_leg_len', mission: 'pursuit', def: 3600, min: 1200, max: 9600, step: 600 },
  { id: 19, name: 'pursuit_speed', mission: 'pursuit', def: 170, min: 100, max: 220, step: 10 },
  { id: 20, name: 'pursuit_start_dist', mission: 'pursuit', def: 800, min: 300, max: 2000, step: 100 },
  { id: 21, name: 'pursuit_pack', mission: 'pursuit', def: 8, min: 0, max: 24, step: 2 },
  { id: 22, name: 'escort_leg_len', mission: 'escort', def: 3000, min: 500, max: 5000, step: 250 },
  { id: 23, name: 'escort_speed', mission: 'escort', def: 70, min: 30, max: 160, step: 10 },
  { id: 24, name: 'escort_hp', mission: 'escort', def: 360, min: 60, max: 1200, step: 60 },
  { id: 25, name: 'escort_follow', mission: 'escort', def: 240, min: 100, max: 500, step: 20 },
  { id: 26, name: 'hold_goal', mission: 'hold', def: 300, min: 30, max: 600, step: 30 },
  { id: 27, name: 'hold_radius', mission: 'hold', def: 160, min: 60, max: 400, step: 20 },
  { id: 28, name: 'hold_point_hp', mission: 'hold', def: 800, min: 100, max: 3000, step: 100 },
  { id: 29, name: 'hold_share', mission: 'hold', def: 3, min: 1, max: 6, step: 1 },
  { id: 30, name: 'escape_goal', mission: 'escape', def: 18000, min: 2000, max: 30000, step: 2000 },
  { id: 31, name: 'escape_ring_count', mission: 'escape', def: 36, min: 0, max: 120, step: 6 },
  { id: 32, name: 'escape_ahead_rate', mission: 'escape', def: 20, min: 0, max: 80, step: 5 },
  { id: 33, name: 'escape_brute_share', mission: 'escape', def: 0.25, min: 0.0, max: 0.5, step: 0.05 },
  { id: 34, name: 'boss_goal', mission: 'boss', def: 12000, min: 2000, max: 30000, step: 2000 },
  { id: 35, name: 'boss_warmup', mission: 'boss', def: 90, min: 0, max: 180, step: 15 },
  { id: 36, name: 'boss_walk_speed', mission: 'boss', def: 85, min: 40, max: 160, step: 5 },
  { id: 37, name: 'boss_spawn_scale', mission: 'boss', def: 0.4, min: 0.0, max: 1.0, step: 0.1 },
  { id: 38, name: 'vendetta_goal', mission: 'vendetta', def: 6, min: 1, max: 12, step: 1 },
  { id: 39, name: 'vendetta_dist_min', mission: 'vendetta', def: 16000, min: 2000, max: 30000, step: 2000 },
  { id: 40, name: 'vendetta_dist_max', mission: 'vendetta', def: 20000, min: 2000, max: 30000, step: 2000 },
  { id: 41, name: 'vendetta_group_hp', mission: 'vendetta', def: 240, min: 60, max: 1200, step: 60 },
];

/** Size of the state array written by the core, in floats. */
export const STATE_FLOATS = 60;

/** Indices into the state array. A run of floats has a 0 suffix (CHOICE0). */
export const STATE = {
  HP: 0,
  MAX_HP: 1,
  XP: 2,
  XP_NEXT: 3,
  LEVEL: 4,
  TIME: 5,
  KILLS: 6,
  CHOOSING: 7,
  CHOICE0: 8,
  OVER: 11,
  CAM_X: 12,
  CAM_Y: 13,
  ENEMIES: 14,
  RUNNING: 15,
  UPGRADE0: 16,
  MISSION: 26,
  GOAL: 27,
  PROGRESS: 28,
  WON: 29,
  ESCAPE: 32,
  ESCAPED: 33,
  CARRIED: 36,
  ESCORT_HP: 44,
  ESCORT_MAX_HP: 45,
  ESCORT_FELL: 46,
  POINT_HP: 48,
  POINT_MAX: 49,
  POINT_FELL: 50,
  EXIT_X: 52,
  EXIT_Y: 53,
  EXIT_DIST: 54,
  BOSS_WAIT: 56,
};

export const INK = [0.17, 0.15, 0.13];
