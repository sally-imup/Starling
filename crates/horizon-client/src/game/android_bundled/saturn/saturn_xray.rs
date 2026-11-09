// Copyright (c) 2026 Sally. Saturn Client, bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// Saturn Client's X-Ray: outlines of ores in the loaded chunks around the player,
// drawn over the walls. A scan finds them every few ticks; the list is kept until the next.
use horizon_client::gizmos::Line;
use std::cell::RefCell;

/// Blocks each way from the player the scan covers (horizontally).
const RADIUS: i32 = 16;
/// Heights the scan covers: where ores generate.
const Y_MIN: i32 = -64;
const Y_MAX: i32 = 72;
/// Ticks between scans.
const SCAN_EVERY: u32 = 40;
/// With Optimization on: a smaller area, scanned less often.
const RADIUS_OPTIMIZED: i32 = 8;
const SCAN_EVERY_OPTIMIZED: u32 = 120;
/// Most ores kept, so a dense area cannot flood the frame.
const MAX_TARGETS: usize = 512;
/// Outline width in pixels.
const WIDTH: f32 = 2.0;

/// Ore names (a part of the block's name) and their outline colours, ARGB.
const ORES: [(&str, u32); 10] = [
    ("diamond", 0xff55_ffff),
    ("emerald", 0xff55_ff55),
    ("gold", 0xffff_ff55),
    ("redstone", 0xffff_5555),
    ("lapis", 0xff55_77ff),
    ("iron", 0xffdd_dddd),
    ("copper", 0xffcc_7744),
    ("coal", 0xff99_9999),
    ("quartz", 0xffee_eeee),
    ("debris", 0xffaa_55ff),
];

#[derive(Default)]
struct State {
    tick: u32,
    /// Each ore's block position and outline colour.
    targets: Vec<([i32; 3], u32)>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn colour(name: &str) -> Option<u32> {
    if !(name.ends_with("_ore") || name.ends_with("ancient_debris")) {
        return None;
    }
    ORES.iter().find(|(key, _)| name.contains(key)).map(|&(_, c)| c)
}

pub(super) fn run(game: &mut crate::game::Game<'_>) {
    use crate::game::saturn_client as saturn;
    let settings = saturn::settings(|key| game.native_setting(saturn::ID, key));
    let on = game.native_on(saturn::ID) && settings.xray_on;
    if !on {
        STATE.with_borrow_mut(|s| s.targets.clear());
        return;
    }
    let (radius, every) = if settings.optimize {
        (RADIUS_OPTIMIZED, SCAN_EVERY_OPTIMIZED)
    } else {
        (RADIUS, SCAN_EVERY)
    };
    let tick = STATE.with_borrow_mut(|s| {
        s.tick = s.tick.wrapping_add(1);
        s.tick
    });
    if tick % every != 1 {
        return;
    }
    let Some(player) = game.native_player() else {
        return;
    };
    let (px, pz) = (player.x.floor() as i32, player.z.floor() as i32);
    let mut found = Vec::new();
    'scan: for x in px - radius..=px + radius {
        for z in pz - radius..=pz + radius {
            // Only generated (loaded) chunks.
            if !game
                .world
                .chunks
                .contains_key(&(x.div_euclid(16), z.div_euclid(16)))
            {
                continue;
            }
            for y in Y_MIN..=Y_MAX {
                let state = game.world.block(x, y, z);
                let facts = game.res.facts().get(state);
                if let Some(argb) = colour(&facts.name) {
                    found.push(([x, y, z], argb));
                    if found.len() >= MAX_TARGETS {
                        break 'scan;
                    }
                }
            }
        }
    }
    STATE.with_borrow_mut(|s| s.targets = found);
}

/// The outlines to draw this frame: every kept ore's box, as lines over the walls.
pub(super) fn lines(game: &crate::game::Game<'_>) -> Vec<Line> {
    use crate::game::saturn_client as saturn;
    if !(game.native_on(saturn::ID)
        && saturn::settings(|key| game.native_setting(saturn::ID, key)).xray_on)
    {
        return Vec::new();
    }
    let Some(player) = game.native_player() else {
        return Vec::new();
    };
    STATE.with_borrow(|s| {
        let mut out = Vec::new();
        for (pos, argb) in &s.targets {
            let origin = pos.map(f64::from);
            let near = (origin[0] - player.x).powi(2)
                + (origin[1] - player.y).powi(2)
                + (origin[2] - player.z).powi(2)
                <= f64::from(RADIUS * RADIUS * 4);
            if !near {
                continue;
            }
            for edge in horizon_client::blocks::cube_edges().iter() {
                let point = |end: usize| std::array::from_fn(|i| origin[i] + edge[i + 3 * end]);
                out.push(Line {
                    start: point(0),
                    end: point(1),
                    color: *argb,
                    width: WIDTH,
                });
            }
        }
        out
    })
}
