// Copyright (c) 2026 Sally. Saturn Client, bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// Each frame: apply Saturn Client's settings to the particle engine.
pub(super) fn run(game: &mut crate::game::Game<'_>) {
    use crate::game::saturn_client as saturn;
    let on = game.native_on(saturn::ID);
    let settings = saturn::settings(|key| game.native_setting(saturn::ID, key));
    game.particles.no_particles = on && settings.no_particles;
}
