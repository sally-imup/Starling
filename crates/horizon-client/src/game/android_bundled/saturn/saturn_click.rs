// Copyright (c) 2026 Sally. Saturn Client, bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// A click on Saturn Client's settings page: change the setting and keep it.
pub(super) fn run(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::saturn_client as saturn;
    if mod_id != saturn::ID {
        return;
    }
    let mut settings = saturn::settings(|key| game.native_setting(saturn::ID, key));
    settings.click(button);
    for (key, value) in settings.stored() {
        game.native_set_setting(saturn::ID, key, value);
    }
}
