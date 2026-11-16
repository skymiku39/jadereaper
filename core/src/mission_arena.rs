//! The `arena` mission: clear a fixed number of waves. A wave is a fixed batch
//! of enemies that appears around the player; the next one only comes after
//! the current one is completely dead and a short pause has passed. The normal
//! time-based horde and elite spawning is off. Lose on death, win when the
//! last wave is cleared.
//!
//! Enemies the player has outrun are not despawned: `Game::move_enemies` puts
//! them back on the spawn ring, so a wave can always be finished.

use crate::defs::*;
use crate::game::Game;

/// Seconds between a wave being cleared and the next one appearing.
pub const PAUSE: f32 = 2.5;
/// Seconds before the first wave.
const FIRST_DELAY: f32 = 1.5;
/// Extra enemy health per wave after the first, as a fraction of base health.
/// This replaces the time-based scaling, which would punish a slow player.
const HP_PER_WAVE: f32 = 0.1;

/// One wave: how many of each kind, `[blots, wisps, brutes, elites]`. The
/// number of waves is the length of this table and the mission's goal in
/// `content/game.toml`.
pub const WAVES: [[u8; 4]; 10] = [
    [10, 0, 0, 0],
    [14, 4, 0, 0],
    [16, 8, 0, 0],
    [18, 8, 2, 0],
    [20, 10, 4, 0],
    [22, 12, 6, 0],
    [24, 14, 8, 0],
    [26, 16, 10, 0],
    [28, 18, 12, 0],
    [30, 20, 14, 1],
];

/// Kind ids in the order of a `WAVES` row.
const ORDER: [u8; 4] = [K_BLOT, K_WISP, K_BRUTE, K_ELITE];

/// Progress through the waves.
pub struct Arena {
    /// Waves that have appeared so far.
    pub spawned: u32,
    /// Waves that have been fully cleared.
    pub cleared: u32,
    /// Seconds left until the next wave appears.
    pub pause: f32,
}

impl Arena {
    pub fn new() -> Arena {
        Arena { spawned: 0, cleared: 0, pause: FIRST_DELAY }
    }

    pub fn total() -> u32 {
        WAVES.len() as u32
    }

    /// True once every wave is cleared.
    pub fn won(&self) -> bool {
        self.cleared >= Self::total()
    }

    /// Health multiplier of the wave being spawned (`spawned` already counts it).
    pub fn hp_scale(&self) -> f32 {
        1.0 + HP_PER_WAVE * self.spawned.saturating_sub(1) as f32
    }
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new()
    }
}

impl Game {
    /// The arena's replacement for the time-based spawning.
    pub(crate) fn arena_step(&mut self, dt: f32) {
        if !self.enemies.is_empty() {
            return;
        }
        if self.arena.spawned > self.arena.cleared {
            self.arena.cleared = self.arena.spawned;
            self.arena.pause = PAUSE;
        } else if self.arena.spawned < Arena::total() {
            self.arena.pause -= dt;
            if self.arena.pause <= 0.0 {
                self.arena_spawn_wave();
            }
        }
    }

    fn arena_spawn_wave(&mut self) {
        let wave = WAVES[self.arena.spawned as usize];
        self.arena.spawned += 1;
        for (kind, &count) in ORDER.iter().zip(wave.iter()) {
            for _ in 0..count {
                self.spawn_enemy(*kind);
            }
        }
        if wave[3] > 0 {
            self.event(EV_ELITE, self.player.x, self.player.y, 0.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arena_game(seed: u32) -> Game {
        let mut game = Game::new(seed);
        game.start(M_ARENA);
        game
    }

    /// One step with the player kept alive and level-ups answered.
    fn step_unhurt(game: &mut Game) {
        game.player.hp = game.player.max_hp;
        // `tick` does not clear the event list the way `update` does.
        game.events.clear();
        game.tick();
        if game.choosing {
            game.choose(0);
        }
    }

    /// Run until the first wave is on the field.
    fn to_first_wave(game: &mut Game) {
        for _ in 0..600 {
            step_unhurt(game);
            if !game.enemies.is_empty() {
                return;
            }
        }
        panic!("the first wave never came");
    }

    fn kill_all(game: &mut Game) {
        for e in game.enemies.iter_mut() {
            e.hp = 0.0;
        }
    }

    #[test]
    fn the_wave_table_matches_the_mission_goal() {
        assert_eq!(MISSION_GOAL[M_ARENA], WAVES.len() as f32);
        assert_eq!(arena_game(1).goal(), WAVES.len() as f32);
        for (i, w) in WAVES.iter().enumerate() {
            assert!(w.iter().map(|&n| n as u32).sum::<u32>() > 0, "wave {i} is empty");
        }
        // Only the last wave has the elite.
        assert!(WAVES[..WAVES.len() - 1].iter().all(|w| w[3] == 0));
        assert!(WAVES[WAVES.len() - 1][3] > 0);
    }

    #[test]
    fn no_enemy_spawns_outside_a_wave() {
        let mut game = arena_game(3);
        // Before the first wave nothing is there.
        for _ in 0..60 {
            step_unhurt(&mut game);
        }
        assert!(game.enemies.is_empty());
        assert_eq!(game.arena.spawned, 0);
        to_first_wave(&mut game);
        let n = game.enemies.len();
        assert_eq!(n, WAVES[0].iter().map(|&c| c as usize).sum::<usize>());
        // While the wave lives, the field only ever shrinks; the time-based
        // horde would have added more by now.
        let mut last = n;
        for _ in 0..(60 * 60) {
            step_unhurt(&mut game);
            assert!(game.enemies.len() <= last, "an enemy spawned mid-wave");
            last = game.enemies.len();
            if last == 0 {
                break;
            }
        }
    }

    #[test]
    fn the_next_wave_waits_for_the_last_enemy_and_a_pause() {
        let mut game = arena_game(4);
        to_first_wave(&mut game);
        // Leave one enemy alive, far away and harmless.
        let keep = game.enemies.len() - 1;
        for e in game.enemies.iter_mut().take(keep) {
            e.hp = 0.0;
        }
        for _ in 0..(60 * 20) {
            game.player.hp = game.player.max_hp;
            game.tick();
            if game.choosing {
                game.choose(0);
            }
            if let Some(e) = game.enemies.first_mut() {
                e.hp = 1.0e9; // the straggler survives the player's weapons
            }
        }
        assert_eq!(game.enemies.len(), 1);
        assert_eq!(game.arena.spawned, 1);
        assert_eq!(game.arena.cleared, 0);
        assert_eq!(game.progress(), 0.0);

        // Kill it: the pause starts, and only then does wave two appear.
        kill_all(&mut game);
        let mut ticks = 0;
        while game.arena.spawned < 2 {
            step_unhurt(&mut game);
            ticks += 1;
            assert!(ticks < 60 * 10, "wave two never came");
        }
        assert_eq!(game.arena.cleared, 1);
        assert_eq!(game.progress(), 1.0);
        assert!(ticks as f32 / 60.0 >= PAUSE - 0.1, "no pause: {ticks} steps");
        assert_eq!(game.enemies.len(), WAVES[1].iter().map(|&c| c as usize).sum::<usize>());
    }

    #[test]
    fn progress_counts_cleared_waves_and_the_last_clear_wins() {
        let mut game = arena_game(5);
        let total = Arena::total();
        for w in 0..total {
            let mut ticks = 0;
            while game.arena.spawned <= w {
                step_unhurt(&mut game);
                ticks += 1;
                assert!(ticks < 60 * 10, "wave {w} never came");
            }
            assert_eq!(game.progress(), w as f32, "while wave {w} is up");
            assert!(!game.won, "won with wave {w} still alive");
            kill_all(&mut game);
            step_unhurt(&mut game); // reap
            step_unhurt(&mut game); // the clear is noticed
            assert_eq!(game.progress(), (w + 1) as f32);
            assert_eq!(game.won, w + 1 == total, "after wave {w}");
        }
        assert!(game.won && !game.over);
        assert!(game.events.iter().any(|e| e[0] == EV_WIN));
        game.update(crate::game::TICK);
        assert_eq!(game.state[S_WON], 1.0);
        assert_eq!(game.state[S_MISSION], M_ARENA as f32);
        assert_eq!(game.state[S_GOAL], total as f32);
        assert_eq!(game.state[S_PROGRESS], total as f32);
        // The final wave has the elite.
        assert!(game.kills >= WAVES.iter().map(|w| w.iter().map(|&c| c as u32).sum::<u32>()).sum::<u32>());
    }

    #[test]
    fn the_mission_is_lost_on_death() {
        let mut game = arena_game(6);
        to_first_wave(&mut game);
        // Stand still without healing until the crowd kills the player.
        game.player.hp = 1.0;
        for _ in 0..(60 * 60) {
            game.tick();
            if game.choosing {
                game.choose(0);
            }
            if game.over {
                break;
            }
        }
        assert!(game.over && !game.won);
        assert_eq!(game.progress(), 0.0);
    }

    #[test]
    fn the_same_seed_gives_the_same_waves() {
        let layout = |seed: u32| {
            let mut game = arena_game(seed);
            to_first_wave(&mut game);
            game.enemies.iter().map(|e| (e.x.to_bits(), e.y.to_bits(), e.kind)).collect::<Vec<_>>()
        };
        assert_eq!(layout(9), layout(9));
        assert_ne!(layout(9), layout(10));
    }

    #[test]
    fn a_wave_surrounds_the_player_at_the_spawn_ring() {
        let mut game = arena_game(8);
        to_first_wave(&mut game);
        let ring = game.view_w.hypot(game.view_h) + 70.0;
        for e in &game.enemies {
            let d = (e.x - game.player.x).hypot(e.y - game.player.y);
            // One tick of walking may already have happened.
            assert!((d - ring).abs() < 5.0, "enemy at {d}, ring {ring}");
        }
    }

    #[test]
    fn an_enemy_left_far_behind_comes_back() {
        let mut game = arena_game(2);
        to_first_wave(&mut game);
        let ring = game.view_w.hypot(game.view_h) + 70.0;
        game.player.x += 100_000.0;
        game.tick();
        let max = game.enemies.iter().map(|e| (e.x - game.player.x).hypot(e.y - game.player.y)).fold(0.0, f32::max);
        assert!(max < ring * 1.8, "stragglers stay far away: {max}");
        assert_eq!(game.enemies.len(), WAVES[0].iter().map(|&c| c as usize).sum::<usize>());
    }

    #[test]
    fn other_missions_do_not_use_the_arena() {
        let mut game = Game::new(1);
        game.start(M_ENDLESS);
        for _ in 0..(60 * 5) {
            step_unhurt(&mut game);
        }
        assert!(!game.enemies.is_empty() || game.kills > 0);
        assert_eq!(game.arena.spawned, 0);
        assert!(!game.won);
    }

    // -------------------------------------------------------- bot checks

    /// Closest enemy to the player: (dx, dy, distance).
    fn nearest(game: &Game) -> Option<(f32, f32, f32)> {
        let (px, py) = (game.player.x, game.player.y);
        game.enemies
            .iter()
            .map(|e| (e.x - px, e.y - py))
            .map(|(dx, dy)| (dx, dy, dx.hypot(dy)))
            .min_by(|a, b| a.2.total_cmp(&b.2))
    }

    /// Play up to ten simulated minutes. `bot` picks the input from the game.
    /// Returns (won, died, waves cleared, seconds).
    fn play(seed: u32, bot: fn(&Game) -> (f32, f32)) -> (bool, bool, u32, f32) {
        let mut game = arena_game(seed);
        for t in 0..(60 * 600) {
            if t % 6 == 0 {
                game.input = bot(&game);
            }
            game.tick();
            if game.choosing {
                game.choose(0);
            }
            if game.won || game.over {
                break;
            }
        }
        eprintln!("  level {}, kills {}, enemies left {}", game.player.level, game.kills, game.enemies.len());
        (game.won, game.over, game.arena.cleared, game.time)
    }

    /// Only moves away from enemies that are close.
    fn runner(game: &Game) -> (f32, f32) {
        match nearest(game) {
            Some((dx, dy, d)) if d < 450.0 => (-dx / d, -dy / d),
            _ => (0.0, 0.0),
        }
    }

    /// Closes in on the nearest enemy, backs off when hurt.
    fn fighter(game: &Game) -> (f32, f32) {
        let Some((dx, dy, d)) = nearest(game) else { return (0.0, 0.0) };
        // Health does not come back by itself, so back off only just after a hit.
        let hurt = game.player.iframe > 0.0;
        let s = if hurt {
            -1.0
        } else if d > 60.0 {
            1.0
        } else {
            // Keep nudging when close so the scythe stays aimed at the crowd.
            0.3
        };
        (dx / d * s, dy / d * s)
    }

    #[test]
    fn a_bot_that_only_runs_away_does_not_win() {
        for seed in 1..=5 {
            let (won, _, cleared, secs) = play(seed, runner);
            eprintln!("runner seed {seed}: won {won}, waves {cleared}, {secs:.0}s");
            assert!(!won, "seed {seed} was won by running away");
        }
    }

    #[test]
    fn a_fighting_bot_makes_progress() {
        for seed in 1..=6 {
            let (won, died, cleared, secs) = play(seed, fighter);
            eprintln!("fighter seed {seed}: won {won}, died {died}, waves {cleared}, {secs:.0}s");
            assert!(cleared >= 1, "seed {seed} cleared nothing");
        }
    }
}
