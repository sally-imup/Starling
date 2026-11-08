// Copyright (c) 2026 Sally. Saturn Client, bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// Saturn Client's settings page.
pub(super) fn run(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::saturn_client as saturn;
    if mod_id != saturn::ID {
        return;
    }
    let game = &*game;
    let settings = saturn::settings(|key| game.native_setting(saturn::ID, key));
    buttons.extend(settings.labels());
}
