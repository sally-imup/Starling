// Copyright (c) 2026 Sally. Saturn Client, bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// Saturn Client's GUI animation: each menu and screen eases in (grows and fades)
// when it opens. It works on the frame's whole GUI batch, so every menu gets it.
// Only the opening is animated; a closing screen is simply gone.
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::time::Duration;

/// How long the open animation takes, in seconds.
const DURATION: f32 = 0.2;
/// The screen's scale at the start of the animation (1.0 when it is done).
const START_SCALE: f32 = 0.92;

#[derive(Default)]
struct State {
    /// Which screen was open last frame (a hash of it, and of the title).
    key: Option<u64>,
    /// Seconds since that screen opened.
    age: f32,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// Whether a menu or screen is up (the title and every screen but playing).
fn gui_open(game: &crate::game::Game<'_>) -> bool {
    game.title.is_some() || !matches!(game.screen, crate::game::Screen::Playing)
}

/// Which screen is up, as a number: a change of it restarts the animation.
fn screen_key(game: &crate::game::Game<'_>) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    std::mem::discriminant(&game.screen).hash(&mut h);
    game.title.is_some().hash(&mut h);
    h.finish()
}

/// Once a frame: restart the animation when the screen changes.
pub(super) fn frame(game: &crate::game::Game<'_>, elapsed: Duration) {
    use crate::game::saturn_client as saturn;
    let on = game.native_on(saturn::ID)
        && saturn::settings(|key| game.native_setting(saturn::ID, key)).gui_anim;
    let key = screen_key(game);
    STATE.with_borrow_mut(|s| {
        if !on || !gui_open(game) {
            s.key = None;
            s.age = 0.0;
            return;
        }
        if s.key != Some(key) {
            s.key = Some(key);
            s.age = 0.0;
        }
        s.age += elapsed.as_secs_f32();
    });
}

/// Scale the frame's GUI about its centre and fade it in, while the screen is young.
pub(super) fn apply(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client_render::hud::HudBatch,
) {
    use crate::game::saturn_client as saturn;
    if !(game.native_on(saturn::ID)
        && saturn::settings(|key| game.native_setting(saturn::ID, key)).gui_anim
        && gui_open(game))
    {
        return;
    }
    let age = STATE.with_borrow(|s| s.age);
    if age >= DURATION {
        return;
    }
    // Ease out: fast at first, settling at the end.
    let t = (age / DURATION).clamp(0.0, 1.0);
    let ease = 1.0 - (1.0 - t).powi(3);
    let scale = START_SCALE + (1.0 - START_SCALE) * ease;
    let alpha = ease;
    let centre = [batch.width * 0.5, batch.height * 0.5];
    for v in &mut batch.vertices {
        for (axis, c) in centre.iter().enumerate() {
            v.pos[axis] = c + (v.pos[axis] - c) * scale;
        }
        // Colours are ARGB: the alpha byte is the top one.
        let a = ((v.color >> 24) as f32 * alpha) as u32;
        v.color = (v.color & 0x00ff_ffff) | (a << 24);
    }
}
