//! Generated from `content/game.toml` by `tools/gen`. Do not edit: change the
//! content file and run `./build.sh`. Re-exported by `crate::defs`.

// Part ids.
pub const P_ELLIPSE: f32 = 0.0;
pub const P_PLAYER_BODY: f32 = 1.0;
pub const P_PLAYER_HEAD: f32 = 2.0;
pub const P_SCYTHE: f32 = 3.0;
pub const P_BLOT: f32 = 4.0;
pub const P_WISP: f32 = 5.0;
pub const P_BRUTE: f32 = 6.0;
pub const P_GEM: f32 = 7.0;
pub const P_SLASH: f32 = 8.0;
pub const P_TALISMAN: f32 = 9.0;
pub const P_BEAD: f32 = 10.0;
pub const P_SPLAT: f32 = 11.0;
pub const P_GRASS: f32 = 12.0;
pub const P_STONE: f32 = 13.0;
pub const P_HEART: f32 = 14.0;
pub const P_RING: f32 = 15.0;
pub const P_SCRAP: f32 = 16.0;
pub const P_GROUND_SHADOW: f32 = 18.0;

// Upgrade ids.
pub const U_SCYTHE: usize = 0;
pub const U_TALISMAN: usize = 1;
pub const U_BEADS: usize = 2;
pub const U_PULSE: usize = 3;
pub const U_VITALITY: usize = 4;
pub const U_SWIFT: usize = 5;
pub const U_LURE: usize = 6;
pub const U_MIGHT: usize = 7;
pub const U_HASTE: usize = 8;
pub const U_MENDING: usize = 9;
pub const N_UPGRADES: usize = 10;
pub const UPGRADE_MAX: [u8; N_UPGRADES] = [6, 6, 6, 6, 5, 5, 5, 5, 5, 5];

// Event types, read by the host for sound.
pub const EV_HIT: f32 = 1.0;
pub const EV_KILL: f32 = 2.0;
pub const EV_PICKUP: f32 = 3.0;
pub const EV_LEVELUP: f32 = 4.0;
pub const EV_HURT: f32 = 5.0;
pub const EV_SLASH: f32 = 6.0;
pub const EV_SHOOT: f32 = 7.0;
pub const EV_PULSE: f32 = 8.0;
pub const EV_DEATH: f32 = 9.0;
pub const EV_HEAL: f32 = 10.0;
pub const EV_ELITE: f32 = 11.0;

// HUD state buffer: size and the index of each field.
pub const STATE_FLOATS: usize = 32;
pub const S_HP: usize = 0;
pub const S_MAX_HP: usize = 1;
pub const S_XP: usize = 2;
pub const S_XP_NEXT: usize = 3;
pub const S_LEVEL: usize = 4;
pub const S_TIME: usize = 5;
pub const S_KILLS: usize = 6;
pub const S_CHOOSING: usize = 7;
pub const S_CHOICE0: usize = 8;
pub const S_OVER: usize = 11;
pub const S_CAM_X: usize = 12;
pub const S_CAM_Y: usize = 13;
pub const S_ENEMIES: usize = 14;
pub const S_RUNNING: usize = 15;
pub const S_UPGRADE0: usize = 16;

pub const INK: [f32; 3] = [0.17, 0.15, 0.13];
