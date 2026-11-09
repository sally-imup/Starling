//! Fixed MIT-licensed Android mod bundle. No downloaded code or phone compiler.

use super::*;

use super::native_hooks::*;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/runtime.rs"]
pub(super) mod official_zoom;

#[cfg(target_os = "android")]
#[path = "android_bundled/saturn/runtime.rs"]
pub(super) mod saturn_client;

#[cfg(target_os = "android")]
#[path = "android_bundled/saturn/saturn_list.rs"]
mod saturn_list;

#[cfg(target_os = "android")]
#[path = "android_bundled/saturn/saturn_settings.rs"]
mod saturn_settings;

#[cfg(target_os = "android")]
#[path = "android_bundled/saturn/saturn_click.rs"]
mod saturn_click;

#[cfg(target_os = "android")]
#[path = "android_bundled/saturn/saturn_frame.rs"]
mod saturn_frame;

#[cfg(target_os = "android")]
#[path = "android_bundled/saturn/saturn_outline.rs"]
mod saturn_outline;

#[cfg(target_os = "android")]
#[path = "android_bundled/saturn/saturn_beams.rs"]
mod saturn_beams;

#[cfg(target_os = "android")]
#[path = "android_bundled/saturn/saturn_xray.rs"]
mod saturn_xray;

#[cfg(target_os = "android")]
#[path = "android_bundled/saturn/saturn_gui.rs"]
mod saturn_gui;

#[cfg(target_os = "android")]
#[path = "android_bundled/saturn/saturn_menu.rs"]
mod saturn_menu;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/zoom_key.rs"]
mod zoom_key;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/zoom_wheel.rs"]
mod zoom_wheel;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/zoom_zoom.rs"]
mod zoom_zoom;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/zoom_list.rs"]
mod zoom_list;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/zoom_settings.rs"]
mod zoom_settings;

#[cfg(target_os = "android")]
#[path = "android_bundled/zoom/zoom_click.rs"]
mod zoom_click;

#[cfg(target_os = "android")]
#[path = "android_bundled/minimap/runtime.rs"]
pub(super) mod official_minimap;

#[cfg(target_os = "android")]
#[path = "android_bundled/minimap/minimap_list.rs"]
mod minimap_list;

#[cfg(target_os = "android")]
#[path = "android_bundled/minimap/minimap_settings.rs"]
mod minimap_settings;

#[cfg(target_os = "android")]
#[path = "android_bundled/minimap/minimap_click.rs"]
mod minimap_click;

#[cfg(target_os = "android")]
#[path = "android_bundled/minimap/minimap_minimap.rs"]
mod minimap_minimap;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/runtime.rs"]
pub(super) mod official_recipes;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/glue.rs"]
pub(super) mod official_recipes_glue;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/recipes_list.rs"]
mod recipes_list;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/recipes_settings.rs"]
mod recipes_settings;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/recipes_click.rs"]
mod recipes_click;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/recipes_frame.rs"]
mod recipes_frame;

#[cfg(target_os = "android")]
#[path = "android_bundled/recipes/recipes_event.rs"]
mod recipes_event;

#[cfg(target_os = "android")]
#[path = "android_bundled/smartf3/runtime.rs"]
pub(super) mod smartf3;

#[cfg(target_os = "android")]
#[path = "android_bundled/smartf3/native.rs"]
pub(super) mod smartf3_native;

#[cfg(target_os = "android")]
#[path = "android_bundled/smartf3/smartf3_command.rs"]
mod smartf3_command;

#[cfg(target_os = "android")]
#[path = "android_bundled/smartf3/smartf3_list.rs"]
mod smartf3_list;

#[cfg(target_os = "android")]
#[path = "android_bundled/smartf3/smartf3_settings.rs"]
mod smartf3_settings;

#[cfg(target_os = "android")]
#[path = "android_bundled/smartf3/smartf3_click.rs"]
mod smartf3_click;

pub(super) fn native_key(game: &mut Game<'_>, key: &str, pressed: bool) {
    let _ = (&game, &key, &pressed);
    #[cfg(target_os = "android")]
    {
        saturn_menu::key(&mut *game, key, pressed);
        zoom_key::run(&mut *game, key, pressed);
    }
}

pub(super) fn native_wheel(game: &mut Game<'_>, steps: f32, claimed: &mut bool) {
    let _ = (&game, &steps, &claimed);
    #[cfg(target_os = "android")]
    {
        zoom_wheel::run(&*game, steps, &mut *claimed);
    }
}

pub(super) fn native_zoom(game: &Game<'_>, zoom: &mut Option<NativeZoom>) {
    let _ = (&game, &zoom);
    #[cfg(target_os = "android")]
    {
        zoom_zoom::run(game, &mut *zoom);
    }
}

pub(super) fn native_mods(game: &mut Game<'_>, mods: &mut Vec<NativeMod>) {
    let _ = (&game, &mods);
    #[cfg(target_os = "android")]
    {
        zoom_list::run(&mut *mods);
        saturn_list::run(&mut *mods);
        minimap_list::run(&mut *mods);
        recipes_list::run(&mut *mods);
        smartf3_list::run(&mut *mods);
    }
}

pub(super) fn native_settings(game: &mut Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    let _ = (&game, &mod_id, &buttons);
    #[cfg(target_os = "android")]
    {
        zoom_settings::run(&mut *game, mod_id, &mut *buttons);
        saturn_settings::run(&mut *game, mod_id, &mut *buttons);
        minimap_settings::run(&mut *game, mod_id, &mut *buttons);
        recipes_settings::run(&mut *game, mod_id, &mut *buttons);
        smartf3_settings::run(&mut *game, mod_id, &mut *buttons);
    }
}

pub(super) fn native_settings_click(game: &mut Game<'_>, mod_id: &str, button: usize) {
    let _ = (&game, &mod_id, &button);
    #[cfg(target_os = "android")]
    {
        zoom_click::run(&mut *game, mod_id, button);
        saturn_click::run(&mut *game, mod_id, button);
        minimap_click::run(&mut *game, mod_id, button);
        recipes_click::run(&mut *game, mod_id, button);
        smartf3_click::run(&mut *game, mod_id, button);
    }
}

pub(super) fn native_minimap(game: &Game<'_>, minimap: &mut Option<NativeMinimap>) {
    let _ = (&game, &minimap);
    #[cfg(target_os = "android")]
    {
        minimap_minimap::run(game, &mut *minimap);
    }
}

pub(super) fn native_frame(game: &mut Game<'_>, elapsed: Duration) {
    let _ = (&game, &elapsed);
    #[cfg(target_os = "android")]
    {
        recipes_frame::run(&mut *game);
        saturn_frame::run(&mut *game);
        saturn_gui::frame(&*game, elapsed);
        saturn_menu::frame(&mut *game);
    }
}

pub(super) fn native_panel_event(game: &mut Game<'_>, mod_id: &str, event: &NativePanelEvent) {
    let _ = (&game, &mod_id, &event);
    #[cfg(target_os = "android")]
    {
        recipes_event::run(&mut *game, mod_id, event);
    }
}

pub(super) fn native_command(game: &mut Game<'_>, command: &str, ran: &mut bool) {
    let _ = (&game, &command, &ran);
    #[cfg(target_os = "android")]
    {
        smartf3_command::run(&mut *game, command, &mut *ran);
    }
}

#[cfg(target_os = "android")]
pub(super) fn reset() {
    official_zoom::reset();
    official_minimap::reset();
    official_recipes::set_viewer(None);
    smartf3_native::reset();
}

pub(super) fn native_outline_style(game: &Game<'_>) -> Option<(u32, f32)> {
    #[cfg(target_os = "android")]
    {
        saturn_outline::run(game)
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = game;
        None
    }
}

pub(super) fn native_tick(game: &mut Game<'_>) {
    let _ = &game;
    #[cfg(target_os = "android")]
    {
        saturn_beams::run(&mut *game);
        saturn_xray::run(&mut *game);
    }
}

pub(super) fn native_xray_lines(game: &Game<'_>) -> Vec<crate::gizmos::Line> {
    #[cfg(target_os = "android")]
    {
        saturn_xray::lines(game)
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = game;
        Vec::new()
    }
}

pub(super) fn native_gui_batch(game: &Game<'_>, batch: &mut horizon_client_render::hud::HudBatch) {
    #[cfg(target_os = "android")]
    {
        saturn_gui::apply(game, batch);
        saturn_menu::draw(game, batch);
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = (game, batch);
    }
}

pub(super) fn native_screen_mouse_button_pressed_pre(
    screen: &NativeScreenView<'_>,
    button: i32,
    canceled: &mut bool,
) {
    #[cfg(target_os = "android")]
    {
        saturn_menu::click(screen.mouse, button, canceled);
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = (screen, button, canceled);
    }
}

pub(super) fn native_menu_takes_mouse(
    game: &mut Game<'_>,
    mouse: [f32; 2],
    button: i32,
    pressed: bool,
) -> bool {
    let _ = (&game, &mouse, &button, &pressed);
    #[cfg(target_os = "android")]
    {
        saturn_menu::takes_mouse(mouse, button, pressed)
    }
    #[cfg(not(target_os = "android"))]
    {
        false
    }
}
