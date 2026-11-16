//! Accepting a mission: before a mission starts, the player walks to a spot
//! and stands on it for a while.
//!
//! The spot is the origin, where every mission lays itself out, and the player
//! starts `T_ACCEPT_DIST` away from it. Nothing else happens until the mission
//! is accepted: no enemies, and the run's clock stands still. Standing inside
//! the circle for `T_ACCEPT_TIME` seconds in one go starts the mission;
//! stepping out starts the count again. A time of 0 skips all of this.

use crate::defs::*;
use crate::fmath;
use crate::game::Game;
use core::f32::consts::TAU;

/// Where the mission is accepted.
pub const SPOT: (f32, f32) = (0.0, 0.0);
/// The player accepts while he is this close to the spot.
pub const RADIUS: f32 = 70.0;
/// How far inside the screen edge the pointing marker sits.
const EDGE: f32 = 44.0;

const JADE: [f32; 4] = [0.25, 0.62, 0.48, 1.0];

/// What the accept phase keeps between steps.
#[derive(Default)]
pub struct Accept {
    /// True from `Game::brief` until the mission is accepted.
    pub active: bool,
    /// The mission that starts once it is accepted, an `M_*` id.
    pub mission: usize,
    /// Seconds stood in the circle without leaving it.
    pub held: f32,
    /// Seconds to stand there, `T_ACCEPT_TIME`.
    pub need: f32,
}

impl Game {
    /// Begin a run of the given mission with its accept phase. The mission
    /// itself starts, through `Game::start`, once the player has accepted it,
    /// or at once when the time to stand is 0.
    pub fn brief(&mut self, mission: usize) {
        let need = self.tune[T_ACCEPT_TIME];
        if need <= 0.0 {
            self.start(mission);
            return;
        }
        let a = self.rng.range(0.0, TAU);
        let d = self.tune[T_ACCEPT_DIST];
        self.player.x = SPOT.0 + fmath::cos(a) * d;
        self.player.y = SPOT.1 + fmath::sin(a) * d;
        self.accept = Accept { active: true, mission, held: 0.0, need };
        self.running = true;
    }

    /// One step of the accept phase: the player walks, and the time he has
    /// stood in the circle is counted.
    pub(crate) fn accept_step(&mut self, dt: f32) {
        self.walk(dt);
        let (dx, dy) = (self.player.x - SPOT.0, self.player.y - SPOT.1);
        if dx * dx + dy * dy <= RADIUS * RADIUS {
            self.accept.held += dt;
        } else {
            self.accept.held = 0.0;
        }
        if self.accept.held >= self.accept.need {
            self.accept.active = false;
            self.event(EV_LEVELUP, self.player.x, self.player.y, 0.0);
            self.start(self.accept.mission);
        }
    }

    /// The circle to stand in. It fills from the middle as the time is served.
    pub(crate) fn accept_draw_ground(&mut self) {
        if !self.accept.active {
            return;
        }
        let pose = (self.anim * 3.0).floor() as i32 & 1;
        let swell = if pose == 0 { 1.0 } else { 1.08 };
        let done = (self.accept.held / self.accept.need.max(0.001)).clamp(0.0, 1.0);
        let (x, y) = SPOT;
        self.put(x, y, RADIUS * 1.1, RADIUS * 0.85, 0.0, P_ELLIPSE, 0.0, 0.1, [JADE[0], JADE[1], JADE[2], 0.18]);
        if done > 0.0 {
            let r = RADIUS * done;
            self.put(x, y, r * 1.1, r * 0.85, 0.0, P_ELLIPSE, 0.0, 0.3, [JADE[0], JADE[1], JADE[2], 0.55]);
        }
        self.put(x, y, RADIUS * 1.15 * swell, RADIUS * 0.9 * swell, 0.0, P_RING, 0.0, 0.5, JADE);
    }

    /// An arrow at the screen edge pointing to the spot while it is off screen.
    pub(crate) fn accept_draw_marker(&mut self) {
        if !self.accept.active {
            return;
        }
        let (dx, dy) = (SPOT.0 - self.cam.0, SPOT.1 - self.cam.1);
        let (hw, hh) = ((self.view_w - EDGE).max(1.0), (self.view_h - EDGE).max(1.0));
        if dx.abs() <= hw && dy.abs() <= hh {
            return;
        }
        self.point(SPOT.0, SPOT.1);
        let k = (hw / dx.abs().max(0.001)).min(hh / dy.abs().max(0.001));
        let (mx, my) = (self.cam.0 + dx * k, self.cam.1 + dy * k);
        let rot = fmath::atan2(dy, dx) + TAU * 0.25;
        self.put(mx, my, 22.0, 22.0, 0.0, P_RING, 0.0, 0.5, [JADE[0], JADE[1], JADE[2], 0.6]);
        self.put(mx, my, 9.0, 16.0, rot, P_TALISMAN, 0.0, 0.5, JADE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::TICK;

    /// A run of `mission` in its accept phase, with the given distance and time.
    fn briefed(seed: u32, mission: usize, dist: f32, time: f32) -> Game {
        let mut game = Game::new(seed);
        game.set_tune(T_ACCEPT_DIST, dist);
        game.set_tune(T_ACCEPT_TIME, time);
        game.brief(mission);
        game
    }

    /// Walk straight at the spot for `ticks` steps.
    fn walk_in(game: &mut Game, ticks: u32) {
        for _ in 0..ticks {
            game.input = (SPOT.0 - game.player.x, SPOT.1 - game.player.y);
            game.tick();
        }
    }

    #[test]
    fn the_run_waits_at_a_distance_until_the_mission_is_accepted() {
        let mut game = briefed(7, M_ESCAPE, 600.0, 3.0);
        assert!(game.accept.active && game.running);
        let d = (game.player.x.powi(2) + game.player.y.powi(2)).sqrt();
        assert!((d - 600.0).abs() < 1.0, "started {d} away");
        // The mission has not begun: no ring of enemies, no goal, a still clock.
        assert_eq!(game.mission, M_ENDLESS);
        for _ in 0..600 {
            game.tick();
        }
        assert!(game.accept.active && game.enemies.is_empty());
        assert_eq!(game.time, 0.0);
        assert_eq!(game.goal(), 0.0);
    }

    #[test]
    fn standing_in_the_circle_long_enough_starts_the_mission() {
        let mut game = briefed(7, M_ESCAPE, 600.0, 3.0);
        // 600 units at 190 a second is a little over three seconds.
        walk_in(&mut game, 60 * 4);
        assert!(game.accept.active, "accepted before the time was served");
        assert!(game.accept.held > 0.0);
        walk_in(&mut game, 60 * 3);
        assert!(!game.accept.active);
        assert_eq!(game.mission, M_ESCAPE);
        assert!(!game.enemies.is_empty(), "the escape ring should be there");
        assert!(game.events.iter().any(|e| e[0] == EV_LEVELUP) || game.time > 0.0);
    }

    #[test]
    fn leaving_the_circle_starts_the_count_again() {
        let mut game = briefed(3, M_SURVIVE, 0.0, 5.0);
        for _ in 0..120 {
            game.tick();
        }
        assert!((game.accept.held - 2.0).abs() < 0.05);
        game.player.x = RADIUS + 50.0;
        game.tick();
        assert_eq!(game.accept.held, 0.0);
        assert!(game.accept.active);
    }

    #[test]
    fn a_time_of_zero_skips_the_phase() {
        let game = briefed(5, M_SURVIVE, 600.0, 0.0);
        assert!(!game.accept.active);
        assert_eq!(game.mission, M_SURVIVE);
        assert_eq!((game.player.x, game.player.y), (0.0, 0.0));
    }

    #[test]
    fn the_spot_is_drawn_pointed_at_and_reported() {
        let mut game = briefed(9, M_HOLD, 2000.0, 3.0);
        game.update(TICK);
        // Far away: an arrow with its distance, and the state says so.
        assert_eq!(game.pointers.len(), 1);
        assert!((game.pointers[0][2] - 2000.0).abs() < 5.0);
        assert_eq!(game.state[S_ACCEPTING], 1.0);
        assert_eq!(game.state[S_ACCEPT_NEED], 3.0);
        // On the spot: the circle is on screen and nothing points at it.
        game.player.x = 0.0;
        game.player.y = 0.0;
        game.update(TICK);
        assert!(game.pointers.is_empty());
        assert!(game.inst.iter().any(|i| i.part == P_RING));
        assert!(game.state[S_ACCEPT_HELD] > 0.0);
        // A run started without the phase reports none.
        let mut plain = Game::new(9);
        plain.start(M_HOLD);
        plain.update(TICK);
        assert_eq!(plain.state[S_ACCEPTING], 0.0);
    }
}
