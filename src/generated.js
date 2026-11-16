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

/** Missions, indexed by id. `goal` is what the mission asks for (seconds for survive).
 * Names and descriptions are in src/i18n.js as mission.<name>.name and .desc. */
export const MISSIONS = [
  { id: 0, name: 'endless', goal: 0 },
  { id: 1, name: 'survive', goal: 300 },
];

/** Mission ids by name. */
export const MISSION_IDS = {
  endless: 0,
  survive: 1,
};

/** Size of the state array written by the core, in floats. */
export const STATE_FLOATS = 32;

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
};

export const INK = [0.17, 0.15, 0.13];
