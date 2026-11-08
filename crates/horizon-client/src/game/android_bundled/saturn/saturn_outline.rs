// Copyright (c) 2026 Sally. Saturn Client, bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// The block selection box: Saturn's colour and width, while the mod is on.
pub(super) fn run(game: &crate::game::Game<'_>) -> Option<(u32, f32)> {
    use crate::game::saturn_client as saturn;
    if !game.native_on(saturn::ID) {
        return None;
    }
    saturn::settings(|key| game.native_setting(saturn::ID, key)).box_style()
}
