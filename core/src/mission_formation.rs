//! The `formation` mission: destroy every formation node on the field.
//!
//! Each node has health, is hurt by the player's own weapons and keeps
//! spawning enemies around itself while the player is near. The ordinary
//! spawning from the screen edge stays on at a reduced rate, so the run still
//! feels like a horde game. The mission is won when the last node falls.
//!
//! The state lives in `Formation`, held as `Game::formation`. The `impl Game`
//! block below holds the hooks the rest of `game.rs` calls; all of them do
//! nothing outside this mission.

use crate::defs::*;
use crate::fmath;
use crate::game::Game;
use core::f32::consts::TAU;

/// Health of a node. The scythe alone takes about 90 seconds to bring one down.
pub const NODE_HP: f32 = 2000.0;
/// Radius of a node. A weapon hits it when it reaches this far past the centre.
pub const NODE_R: f32 = 44.0;
/// Seconds between two spawn waves of an active node.
const SPAWN_EVERY: f32 = 1.6;
/// Enemies in a wave.
const SPAWN_COUNT: usize = 3;
/// Enemies appear this far from the node's centre (min, max).
const SPAWN_AT: (f32, f32) = (80.0, 160.0);
/// A node only spawns while the player is within this many spawn rings.
const ACTIVE_RINGS: f32 = 1.3;
/// Nodes stop spawning while this many enemies are alive.
const ENEMY_CAP: usize = 380;
/// Share of the usual edge-of-screen spawn rate that stays on.
const EDGE_RATE: f32 = 0.4;
/// Distance of the nodes from the start (min, max).
const LAYOUT_RADIUS: (f32, f32) = (1300.0, 2000.0);
/// Seconds a destroyed node's shockwave ring takes to fade.
const RUIN_FADE: f32 = 0.8;

#[derive(Clone, Copy)]
pub struct Node {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    flash: f32,
    timer: f32,
    bead_cd: f32,
    fade: f32,
}

impl Node {
    pub fn alive(&self) -> bool {
        self.hp > 0.0
    }
}

#[derive(Default)]
pub struct Formation {
    pub nodes: Vec<Node>,
    /// Enemies the nodes have spawned so far.
    pub spawned: u32,
}

impl Formation {
    /// Lay out `n` nodes around the origin from the run's seed state: evenly
    /// spread in angle with some jitter, at varying distances.
    pub fn start(&mut self, state: u32, n: usize) {
        let mut rng = Rng((state ^ 0x7f4a_7c15) | 1);
        let phase = rng.range(0.0, TAU);
        self.nodes.clear();
        self.spawned = 0;
        for k in 0..n {
            let a = phase + (k as f32 + rng.range(-0.25, 0.25)) * TAU / n as f32;
            let r = rng.range(LAYOUT_RADIUS.0, LAYOUT_RADIUS.1);
            self.nodes.push(Node {
                x: fmath::cos(a) * r,
                y: fmath::sin(a) * r,
                hp: NODE_HP,
                flash: 0.0,
                timer: rng.range(0.0, SPAWN_EVERY),
                bead_cd: 0.0,
                fade: 0.0,
            });
        }
    }

    /// Nodes destroyed so far.
    pub fn destroyed(&self) -> usize {
        self.nodes.iter().filter(|n| !n.alive()).count()
    }

    /// True once there are nodes and none is left standing.
    pub fn cleared(&self) -> bool {
        !self.nodes.is_empty() && self.nodes.iter().all(|n| !n.alive())
    }

    /// The live node closest to a point.
    pub fn nearest(&self, x: f32, y: f32) -> Option<&Node> {
        let mut best: Option<(&Node, f32)> = None;
        for n in self.nodes.iter().filter(|n| n.alive()) {
            let d = (n.x - x) * (n.x - x) + (n.y - y) * (n.y - y);
            if best.is_none_or(|(_, b)| d < b) {
                best = Some((n, d));
            }
        }
        best.map(|(n, _)| n)
    }

    /// Feed the state into the run's checksum. Adds nothing outside this mission.
    pub fn mix_into(&self, mix: &mut dyn FnMut(u32)) {
        for n in &self.nodes {
            for v in [n.x, n.y, n.hp, n.timer] {
                mix(v.to_bits());
            }
        }
        if !self.nodes.is_empty() {
            mix(self.spawned);
        }
    }
}

impl Game {
    /// Lay out the nodes. Called when the run starts.
    pub(crate) fn formation_start(&mut self) {
        let n = self.goal() as usize;
        self.formation.start(self.rng.0, n);
    }

    /// Scale of the edge-of-screen spawn rate.
    pub(crate) fn formation_edge_scale(&self) -> f32 {
        if self.mission == M_FORMATION {
            EDGE_RATE
        } else {
            1.0
        }
    }

    /// Run the nodes' timers and spawn their enemies. Called once per step.
    pub(crate) fn formation_step(&mut self, dt: f32) {
        if self.mission != M_FORMATION {
            return;
        }
        let (px, py) = (self.player.x, self.player.y);
        let ring = (self.view_w * self.view_w + self.view_h * self.view_h).sqrt() + 70.0;
        let active = ring * ACTIVE_RINGS;
        for i in 0..self.formation.nodes.len() {
            let n = &mut self.formation.nodes[i];
            n.flash = (n.flash - dt * 6.0).max(0.0);
            n.bead_cd = (n.bead_cd - dt).max(0.0);
            if !n.alive() {
                n.fade = (n.fade - dt).max(0.0);
                continue;
            }
            let (x, y) = (n.x, n.y);
            if (x - px) * (x - px) + (y - py) * (y - py) > active * active {
                continue;
            }
            n.timer -= dt;
            if n.timer > 0.0 {
                continue;
            }
            n.timer += SPAWN_EVERY;
            for _ in 0..SPAWN_COUNT {
                if self.enemies.len() >= ENEMY_CAP {
                    break;
                }
                let t = self.time;
                let roll = self.rng.f();
                let brute = if t > 60.0 { ((t - 60.0) / 400.0).min(0.25) } else { 0.0 };
                let kind = if roll < brute {
                    K_BRUTE
                } else if roll < brute + 0.3 {
                    K_WISP
                } else {
                    K_BLOT
                };
                let a = self.rng.range(0.0, TAU);
                let r = self.rng.range(SPAWN_AT.0, SPAWN_AT.1);
                self.spawn_enemy_at(kind, x + fmath::cos(a) * r, y + fmath::sin(a) * r);
                self.formation.spawned += 1;
            }
        }
    }

    /// Hurt a node, and destroy it when its health runs out.
    pub(crate) fn damage_node(&mut self, i: usize, dmg: f32) {
        let n = &mut self.formation.nodes[i];
        if !n.alive() {
            return;
        }
        n.hp -= dmg;
        n.flash = 1.0;
        let (x, y) = (n.x, n.y);
        let kind = if n.alive() {
            EV_HIT
        } else {
            n.hp = 0.0;
            n.fade = RUIN_FADE;
            self.shake = self.shake.max(0.6);
            EV_KILL
        };
        if self.events.len() < 256 {
            self.events.push([kind, x, y, if kind == EV_HIT { dmg } else { K_ELITE as f32 }]);
        }
    }

    /// The scythe's arc, as `weapons` swings it.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn formation_hit_arc(&mut self, px: f32, py: f32, ax: f32, ay: f32, reach: f32, cosw: f32, dmg: f32) {
        for i in 0..self.formation.nodes.len() {
            let n = &self.formation.nodes[i];
            let (dx, dy) = (n.x - px, n.y - py);
            let d = (dx * dx + dy * dy).sqrt().max(0.001);
            if n.alive() && d < reach + NODE_R && (d < 30.0 || (dx * ax + dy * ay) / d > cosw) {
                self.damage_node(i, dmg);
            }
        }
    }

    /// The spirit pulse.
    pub(crate) fn formation_hit_circle(&mut self, x: f32, y: f32, radius: f32, dmg: f32) {
        for i in 0..self.formation.nodes.len() {
            let n = &self.formation.nodes[i];
            let (dx, dy) = (n.x - x, n.y - y);
            let r = radius + NODE_R;
            if n.alive() && dx * dx + dy * dy < r * r {
                self.damage_node(i, dmg);
            }
        }
    }

    /// A jade bead at `(x, y)`. A node takes a bead's hit once per cooldown.
    pub(crate) fn formation_hit_bead(&mut self, x: f32, y: f32, dmg: f32) {
        for i in 0..self.formation.nodes.len() {
            let n = &mut self.formation.nodes[i];
            let (dx, dy) = (n.x - x, n.y - y);
            let r = NODE_R + 12.0;
            if n.alive() && n.bead_cd <= 0.0 && dx * dx + dy * dy < r * r {
                n.bead_cd = 0.45;
                self.damage_node(i, dmg);
            }
        }
    }

    /// A talisman at `(x, y)`. True when it hit a node and is spent.
    pub(crate) fn formation_hit_shot(&mut self, x: f32, y: f32, dmg: f32) -> bool {
        for i in 0..self.formation.nodes.len() {
            let n = &self.formation.nodes[i];
            let (dx, dy) = (n.x - x, n.y - y);
            let r = NODE_R + 8.0;
            if n.alive() && dx * dx + dy * dy < r * r {
                self.damage_node(i, dmg);
                return true;
            }
        }
        false
    }

    /// Draw the nodes on the ground, below the characters.
    pub(crate) fn draw_formation(&mut self) {
        let pose = ((self.anim * 3.0).floor() as i32) & 1;
        for i in 0..self.formation.nodes.len() {
            let n = self.formation.nodes[i];
            let shadow = [INK[0], INK[1], INK[2], 0.25];
            self.put(n.x, n.y, 66.0, 22.0, 0.0, P_GROUND_SHADOW, 0.0, 0.0, shadow);
            if n.alive() {
                let left = (n.hp / NODE_HP).clamp(0.0, 1.0);
                let vermilion = [0.80, 0.27, 0.18, 1.0];
                // The area where it spawns, and the stones of the formation.
                self.put(n.x, n.y, 84.0, 34.0, 0.0, P_RING, 0.0, 0.2, [0.80, 0.27, 0.18, 0.5]);
                self.put(n.x, n.y - 28.0, 42.0, 28.0, 0.0, P_STONE, n.flash, 0.3, [0.36, 0.22, 0.19, 1.0]);
                let tilt = if pose == 0 { 0.06 } else { -0.06 };
                self.put(n.x, n.y - 68.0, 28.0, 24.0, tilt, P_STONE, n.flash, 0.6, [0.42, 0.25, 0.21, 1.0]);
                // The eye burns smaller as the node weakens.
                let eye = 9.0 + 9.0 * left;
                let bob = if pose == 0 { 0.0 } else { -3.0 };
                self.put(n.x, n.y - 108.0 + bob, eye, eye, 0.0, P_ELLIPSE, n.flash, 0.4, vermilion);
            } else {
                self.put(n.x, n.y - 14.0, 42.0, 14.0, 0.0, P_STONE, 0.0, 0.3, [0.40, 0.38, 0.35, 0.5]);
                if n.fade > 0.0 {
                    let t = n.fade / RUIN_FADE;
                    let r = 40.0 + (1.0 - t) * 320.0;
                    self.put(n.x, n.y - 20.0, r, r * 0.5, 0.0, P_RING, 0.0, 0.5, [0.80, 0.27, 0.18, 0.8 * t]);
                }
            }
        }
    }

    /// Draw a marker at the screen edge pointing at the nearest live node,
    /// while that node is off screen.
    pub(crate) fn draw_formation_marker(&mut self) {
        let Some(n) = self.formation.nearest(self.player.x, self.player.y) else {
            return;
        };
        let (dx, dy) = (n.x - self.cam.0, n.y - 40.0 - self.cam.1);
        if dx.abs() < self.view_w - 20.0 && dy.abs() < self.view_h - 20.0 {
            return;
        }
        // Slide along the line from the centre to the node until it meets the
        // edge of the view, kept inside by a margin.
        let m = 52.0;
        let tx = if dx.abs() > 0.001 { (self.view_w - m) / dx.abs() } else { f32::MAX };
        let ty = if dy.abs() > 0.001 { (self.view_h - m) / dy.abs() } else { f32::MAX };
        let t = tx.min(ty);
        let (x, y) = (self.cam.0 + dx * t, self.cam.1 + dy * t);
        let pulse = if ((self.anim * 3.0).floor() as i32) & 1 == 0 { 1.0 } else { 0.85 };
        self.put(x, y, 24.0 * pulse, 24.0 * pulse, 0.0, P_RING, 0.0, 0.1, [0.80, 0.27, 0.18, 0.9]);
        self.put(x, y, 8.0, 15.0, fmath::atan2(dy, dx) + TAU * 0.25, P_TALISMAN, 0.0, 0.5, [0.80, 0.27, 0.18, 1.0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn started(seed: u32) -> Game {
        let mut g = Game::new(seed);
        g.set_view(640.0, 360.0);
        g.start(M_FORMATION);
        g
    }

    /// Put the player on a node, so it counts as near.
    fn go_to(g: &mut Game, i: usize) {
        g.player.x = g.formation.nodes[i].x - 60.0;
        g.player.y = g.formation.nodes[i].y;
    }

    #[test]
    fn layout_comes_from_the_seed() {
        let a = started(7);
        let b = started(7);
        let c = started(8);
        assert_eq!(a.formation.nodes.len(), a.goal() as usize);
        assert!(a.formation.nodes.len() >= 2);
        let same = |p: &Game, q: &Game| {
            p.formation.nodes.iter().zip(&q.formation.nodes).all(|(u, v)| u.x == v.x && u.y == v.y)
        };
        assert!(same(&a, &b));
        assert!(!same(&a, &c));
        for seed in 1..40 {
            let g = started(seed);
            let nodes = &g.formation.nodes;
            for (i, n) in nodes.iter().enumerate() {
                let d = (n.x * n.x + n.y * n.y).sqrt();
                assert!((1290.0..2010.0).contains(&d), "seed {seed}: node {i} at {d}");
                assert!(n.alive() && n.hp == NODE_HP);
                for m in &nodes[i + 1..] {
                    let sep = ((n.x - m.x).powi(2) + (n.y - m.y).powi(2)).sqrt();
                    // Further apart than one node's activation range at this view size.
                    assert!(sep > 1100.0, "seed {seed}: nodes {sep} apart");
                }
            }
        }
    }

    #[test]
    fn other_missions_have_no_nodes() {
        for m in [M_ENDLESS, M_SURVIVE] {
            let mut g = Game::new(3);
            g.start(m);
            for _ in 0..600 {
                g.tick();
            }
            assert!(g.formation.nodes.is_empty());
            assert_eq!(g.formation_edge_scale(), 1.0);
        }
    }

    #[test]
    fn won_when_the_last_node_falls() {
        let mut g = started(5);
        let total = g.formation.nodes.len();
        assert_eq!(g.goal(), total as f32);
        for i in 0..total {
            assert!(!g.won);
            assert_eq!(g.progress(), i as f32);
            g.damage_node(i, NODE_HP - 1.0);
            g.tick();
            assert!(g.formation.nodes[i].alive() && !g.won && g.progress() == i as f32);
            g.damage_node(i, 1.0);
            g.tick();
            assert_eq!(g.progress(), (i + 1) as f32);
        }
        assert!(g.won && !g.over);
        assert!(g.events.iter().any(|e| e[0] == EV_WIN));
        g.update(crate::game::TICK);
        assert_eq!(g.state[S_WON], 1.0);
        assert_eq!(g.state[S_MISSION], M_FORMATION as f32);
        assert_eq!(g.state[S_GOAL], total as f32);
        assert_eq!(g.state[S_PROGRESS], total as f32);
    }

    #[test]
    fn nodes_spawn_while_alive_and_stop_when_destroyed() {
        let mut g = started(9);
        // Far from every node: nothing spawns.
        for _ in 0..600 {
            g.player.hp = g.player.max_hp;
            g.tick();
        }
        assert_eq!(g.formation.spawned, 0);

        go_to(&mut g, 0);
        let (nx, ny) = (g.formation.nodes[0].x, g.formation.nodes[0].y);
        let mut near_node = false;
        for _ in 0..600 {
            g.player.hp = g.player.max_hp;
            let before = g.formation.spawned;
            g.tick();
            if g.formation.spawned > before {
                near_node |= g.enemies.iter().any(|e| (e.x - nx).hypot(e.y - ny) < SPAWN_AT.1 + 40.0);
            }
            if g.choosing {
                g.choose(0);
            }
        }
        assert!(g.formation.spawned >= 3 * 4, "{} spawned", g.formation.spawned);
        assert!(near_node, "no enemy appeared next to the node");

        g.damage_node(0, NODE_HP);
        let spawned = g.formation.spawned;
        for _ in 0..600 {
            g.player.hp = g.player.max_hp;
            g.tick();
            if g.choosing {
                g.choose(0);
            }
        }
        assert_eq!(g.formation.spawned, spawned);
    }

    #[test]
    fn weapons_hurt_a_node_in_reach_only() {
        let mut g = started(4);
        go_to(&mut g, 0);
        g.player.aim = (1.0, 0.0);
        for _ in 0..120 {
            g.player.hp = g.player.max_hp;
            g.tick();
            if g.choosing {
                g.choose(0);
            }
        }
        assert!(g.formation.nodes[0].hp < NODE_HP, "the scythe did not reach the node");
        for n in &g.formation.nodes[1..] {
            assert_eq!(n.hp, NODE_HP);
        }
    }

    #[test]
    fn the_mission_is_lost_on_death() {
        let mut g = started(6);
        go_to(&mut g, 0);
        g.formation.nodes[0].hp = 1.0e9;
        g.player.hp = 1.0;
        for _ in 0..(60 * 120) {
            g.tick();
            if g.choosing {
                g.choose(0);
            }
            if g.over {
                break;
            }
        }
        assert!(g.over && !g.won);
        assert!(g.events.iter().any(|e| e[0] == EV_DEATH));
    }

    /// Run a bot for `seconds`, healing it every step when `unhurt`. Returns the
    /// time of the win.
    fn play(g: &mut Game, seconds: u32, unhurt: bool, mut bot: impl FnMut(&Game) -> (f32, f32)) -> Option<f32> {
        for _ in 0..(60 * seconds) {
            if unhurt {
                g.player.hp = g.player.max_hp;
            }
            g.input = bot(g);
            g.events.clear();
            g.tick();
            if g.choosing {
                g.choose(0);
            }
            if g.won {
                return Some(g.time);
            }
            if g.over {
                return None;
            }
        }
        None
    }

    /// Walks straight at the nearest live node and stands on it.
    fn seeker(g: &Game) -> (f32, f32) {
        let p = &g.player;
        match g.formation.nearest(p.x, p.y) {
            Some(n) => {
                let (dx, dy) = (n.x - p.x, n.y - p.y);
                let d = (dx * dx + dy * dy).sqrt().max(0.001);
                if d < 80.0 {
                    (0.0, 0.0)
                } else {
                    (dx / d, dy / d)
                }
            }
            None => (0.0, 0.0),
        }
    }

    #[test]
    fn a_player_who_walks_up_to_the_nodes_can_win() {
        // Indicative only: a seeker that is never hurt, so it measures the
        // pace of the nodes, not the survival of the bot.
        for seed in [1, 3, 5] {
            let mut g = started(seed);
            let t = play(&mut g, 600, true, seeker).unwrap_or_else(|| panic!("seed {seed}: no win in ten minutes"));
            assert!(t > 90.0, "seed {seed}: won after only {t} s");
            println!("seed {seed}: seeker won at {t:.0} s, {} kills, level {}", g.kills, g.player.level);
        }
    }

    #[test]
    fn running_away_cannot_win() {
        for seed in [1, 3, 5] {
            let mut g = started(seed);
            // Keeps away from whatever is close, never heads for a node.
            let run_away = |g: &Game| {
                let p = &g.player;
                let (mut sx, mut sy) = (0.0, 0.0);
                for e in &g.enemies {
                    let (dx, dy) = (p.x - e.x, p.y - e.y);
                    let d2 = dx * dx + dy * dy;
                    if d2 < 300.0 * 300.0 {
                        let d = d2.sqrt().max(1.0);
                        sx += dx / d / d;
                        sy += dy / d / d;
                    }
                }
                (sx * 40.0, sy * 40.0)
            };
            // Healed, so that it keeps running for the whole ten minutes.
            let won = play(&mut g, 600, true, run_away);
            assert!(won.is_none(), "seed {seed}: won by running away");
            assert_eq!(g.progress(), 0.0, "seed {seed}: a node fell");
            assert!(g.time > 590.0);
        }
    }
}
