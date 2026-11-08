// Copyright (c) 2026 Sally. Saturn Client, bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// Saturn Client's row in Options > Mods.
pub(super) fn run(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::saturn_client as saturn;
    mods.push(crate::game::native_hooks::NativeMod {
        id: saturn::ID.into(),
        title: format!("Saturn Client {}", saturn::VERSION),
        description: saturn::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
