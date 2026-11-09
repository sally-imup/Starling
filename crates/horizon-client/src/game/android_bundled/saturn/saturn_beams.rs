// Copyright (c) 2026 Sally. Saturn Client, bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// Saturn Client's loot beams: a column of particles above each item on the ground.
// The particles live in their own list, so No Particle leaves them alone.
use horizon_client::particles::ParticleOptions;
use std::sync::atomic::{AtomicU32, Ordering};

/// Items farther than this (in blocks, across the ground) get no beam.
const RANGE: f64 = 48.0;
/// With Optimization on: a shorter range, fewer beams, and a slower spawn.
const RANGE_OPTIMIZED: f64 = 24.0;
const MAX_ITEMS_OPTIMIZED: usize = 32;
/// A beam's particles: this many, this far apart (blocks), above the item.
const COUNT: usize = 12;
const STEP: f64 = 0.35;
/// Ticks a particle lives. Spawned every other tick, so the column stays unbroken.
const LIFETIME: i32 = 6;

static TICK: AtomicU32 = AtomicU32::new(0);

pub(super) fn run(game: &mut crate::game::Game<'_>) {
    use crate::game::saturn_client as saturn;
    if !game.native_on(saturn::ID) {
        return;
    }
    let settings = saturn::settings(|key| game.native_setting(saturn::ID, key));
    if !settings.beams_on {
        return;
    }
    let (range, every, max_items) = if settings.optimize {
        (RANGE_OPTIMIZED, 4, MAX_ITEMS_OPTIMIZED)
    } else {
        (RANGE, 2, usize::MAX)
    };
    if TICK.fetch_add(1, Ordering::Relaxed) % every != 0 {
        return;
    }
    let Some(player) = game.native_player() else {
        return;
    };
    let argb = saturn::COLORS[settings.beam_color].1;
    let channel = |shift: u32| ((argb >> shift) & 0xff) as f32 / 255.0;
    let options = ParticleOptions::dust([channel(16), channel(8), channel(0)], 1.0);
    let items = game
        .dropped
        .iter()
        .filter(|e| e.stack.count > 0)
        .filter(|item| {
            let (dx, dz) = (item.position.x - player.x, item.position.z - player.z);
            dx * dx + dz * dz <= range * range
        })
        .take(max_items);
    for item in items {
        let (x, y, z) = (item.position.x, item.position.y, item.position.z);
        for i in 0..COUNT {
            let dy = 0.25 + i as f64 * STEP;
            game.particles.add_beam(&options, [x, y + dy, z], LIFETIME);
        }
    }
}
