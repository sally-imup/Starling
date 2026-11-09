// Copyright (c) 2026 Sally. No Particle, a native mod bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// Hides every particle the game would make. Its one setting is on the mod's page.
use crate::game::native_hooks::NativeMod;

pub const ID: &str = "no_particle";
pub const VERSION: &str = "1.0.0";
const DESCRIPTION: &str = "No Particle: hides the particles the game would make.";
const KEY: &str = "enabled";

fn enabled(game: &crate::game::Game<'_>) -> bool {
    game.native_setting(ID, KEY).is_some_and(|v| v.trim() == "true")
}

/// Its row in Options > Mods.
pub(super) fn list(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: ID.into(),
        title: format!("No Particle {VERSION}"),
        description: DESCRIPTION.into(),
        keys: Vec::new(),
    });
}

/// Its page: one button, and the state of it.
pub(super) fn settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != ID {
        return;
    }
    let on = if enabled(game) { "ON" } else { "OFF" };
    buttons.push(format!("No particles: {on}"));
}

/// A click on its page: flip the setting.
pub(super) fn click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    if mod_id != ID || button != 0 {
        return;
    }
    let value = if enabled(game) { "false" } else { "true" };
    game.native_set_setting(ID, KEY, value);
}

/// Each frame: keep the particle engine in step with the setting.
pub(super) fn frame(game: &mut crate::game::Game<'_>) {
    let on = game.native_on(ID) && enabled(game);
    game.particles.no_particles = on;
}
