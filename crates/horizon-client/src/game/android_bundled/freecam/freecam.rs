// Copyright (c) 2026 Sally. Freecam, a native mod bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// V turns the camera free: WASD, Space and Shift move it, and the mouse turns it. The player
// stays where it is, because its own movement keys are held back while this is on. Its body
// glows while it is on. It is for looking around while building: it does not change which
// block a click hits, and it does not load the chunks around the camera.
use crate::game::native_hooks::NativeMod;
use std::cell::RefCell;
use std::time::Duration;
use winit::keyboard::KeyCode;

pub const ID: &str = "freecam";
pub const VERSION: &str = "1.0.0";
const DESCRIPTION: &str = "Freecam: V turns the camera free, for building. WASD, Space and Shift move it.";
const KEY_TOGGLE: &str = "key.keyboard.v";
/// The movement keys: their vanilla names, and the keys the game reads them from.
const MOVE_NAMES: [&str; 7] = [
    "key.keyboard.w",
    "key.keyboard.a",
    "key.keyboard.s",
    "key.keyboard.d",
    "key.keyboard.space",
    "key.keyboard.left.shift",
    "key.keyboard.left.control",
];
const MOVE_KEYS: [KeyCode; 7] = [
    KeyCode::KeyW,
    KeyCode::KeyA,
    KeyCode::KeyS,
    KeyCode::KeyD,
    KeyCode::Space,
    KeyCode::ShiftLeft,
    KeyCode::ControlLeft,
];
/// Blocks per second, and the sprint multiplier.
const SPEED: f64 = 12.0;
const SPRINT: f64 = 2.5;
/// Degrees per mouse count.
const SENSITIVITY: f32 = 0.15;
/// Seconds the camera takes to fly back to the player when freecam turns off.
const RETURN_TIME: f32 = 0.35;

/// A flight back to the player: where it started, and how long it has run.
#[derive(Clone, Copy)]
struct Return {
    from_pos: [f64; 3],
    from_yaw: f32,
    from_pitch: f32,
    t: f32,
}

#[derive(Default)]
struct State {
    active: bool,
    /// Set while the camera flies back to the player.
    returning: Option<Return>,
    pos: [f64; 3],
    /// Degrees, as the camera has them.
    yaw: f32,
    pitch: f32,
    /// Whether each movement key is held, in `MOVE_NAMES` order.
    held: [bool; 7],
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// Its row in Options > Mods.
pub(super) fn list(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: ID.into(),
        title: format!("Freecam {VERSION}"),
        description: DESCRIPTION.into(),
        keys: Vec::new(),
    });
}

pub(super) fn active() -> bool {
    STATE.with_borrow(|s| s.active)
}

/// The camera's (position, yaw, pitch) while freecam is on.
pub(super) fn view() -> Option<(glam::DVec3, f32, f32)> {
    STATE.with_borrow(|s| s.active.then(|| (glam::DVec3::from_array(s.pos), s.yaw, s.pitch)))
}

/// While freecam is on, the world gets no mouse button: no attack, no use, no place, no break.
pub(super) fn takes_mouse() -> bool {
    active()
}

/// A mouse move turns the camera while freecam is on, and is not given to the player.
pub(super) fn look(dx: f64, dy: f64) -> bool {
    STATE.with_borrow_mut(|s| {
        if !s.active {
            return false;
        }
        if s.returning.is_some() {
            // Still flying back: the mouse is not the player's, and does not turn the camera.
            return true;
        }
        s.yaw += dx as f32 * SENSITIVITY;
        s.pitch = (s.pitch + dy as f32 * SENSITIVITY).clamp(-90.0, 90.0);
        true
    })
}

/// A key press or release: the movement keys are tracked, and V turns freecam on or off.
pub(super) fn key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    if !game.native_on(ID) {
        return;
    }
    if let Some(i) = MOVE_NAMES.iter().position(|name| *name == key) {
        STATE.with_borrow_mut(|s| s.held[i] = pressed);
        return;
    }
    if key != KEY_TOGGLE || !pressed {
        return;
    }
    if active() {
        // Fly back to the player, rather than cutting to it.
        STATE.with_borrow_mut(|s| {
            if s.returning.is_none() {
                s.held = [false; 7];
                s.returning = Some(Return {
                    from_pos: s.pos,
                    from_yaw: s.yaw,
                    from_pitch: s.pitch,
                    t: 0.0,
                });
            }
        });
        return;
    }
    // Only in play: V is a letter in the chat and on signs.
    if !matches!(game.screen, crate::game::Screen::Playing) || game.title.is_some() {
        return;
    }
    let Some(eye) = game.eye_position() else {
        return;
    };
    let (yaw, pitch) = (game.yaw, game.pitch);
    STATE.with_borrow_mut(|s| {
        s.active = true;
        s.pos = eye;
        s.yaw = yaw;
        s.pitch = pitch;
        s.held = [false; 7];
    });
}

fn lerp3(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    std::array::from_fn(|i| a[i] + (b[i] - a[i]) * t)
}

/// The shorter way round from `from` to `to`, `t` of the way.
fn lerp_angle(from: f32, to: f32, t: f32) -> f32 {
    let d = (to - from + 540.0).rem_euclid(360.0) - 180.0;
    from + d * t
}

fn add(dir: &mut [f64; 3], v: [f64; 3], k: f64) {
    for i in 0..3 {
        dir[i] += v[i] * k;
    }
}

/// Each frame: move the camera with the held keys, and hold the player's own keys back.
pub(super) fn frame(game: &mut crate::game::Game<'_>, elapsed: Duration) {
    if !game.native_on(ID) {
        STATE.with_borrow_mut(|s| s.active = false);
        return;
    }
    if !active() {
        return;
    }
    for key in MOVE_KEYS {
        game.keys.remove(&key);
    }
    let dt = elapsed.as_secs_f64();
    if STATE.with_borrow(|s| s.returning.is_some()) {
        // The player's eye, and its look, where the camera is flying to now.
        let target = game.eye_position();
        let (to_yaw, to_pitch) = (game.yaw, game.pitch);
        let done = STATE.with_borrow_mut(|s| {
            let Some(r) = s.returning.as_mut() else {
                return true;
            };
            r.t += dt as f32;
            let p = (r.t / RETURN_TIME).clamp(0.0, 1.0);
            let e = f64::from(p * p * (3.0 - 2.0 * p));
            if let Some(eye) = target {
                s.pos = lerp3(r.from_pos, eye, e);
            }
            s.yaw = lerp_angle(r.from_yaw, to_yaw, e as f32);
            s.pitch = r.from_pitch + (to_pitch - r.from_pitch) * e as f32;
            p >= 1.0
        });
        if done {
            STATE.with_borrow_mut(|s| {
                s.active = false;
                s.returning = None;
                s.held = [false; 7];
            });
        }
        return;
    }
    STATE.with_borrow_mut(|s| {
        let yaw = f64::from(s.yaw).to_radians();
        let pitch = f64::from(s.pitch).to_radians();
        let forward = [-yaw.sin() * pitch.cos(), -pitch.sin(), yaw.cos() * pitch.cos()];
        let flat = [-yaw.sin(), 0.0, yaw.cos()];
        let right = [-flat[2], 0.0, flat[0]];
        let mut dir = [0.0; 3];
        if s.held[0] {
            add(&mut dir, forward, 1.0);
        }
        if s.held[2] {
            add(&mut dir, forward, -1.0);
        }
        if s.held[3] {
            add(&mut dir, right, 1.0);
        }
        if s.held[1] {
            add(&mut dir, right, -1.0);
        }
        if s.held[4] {
            dir[1] += 1.0;
        }
        if s.held[5] {
            dir[1] -= 1.0;
        }
        let speed = SPEED * if s.held[6] { SPRINT } else { 1.0 };
        for i in 0..3 {
            s.pos[i] += dir[i] * speed * dt;
        }
    });
}
