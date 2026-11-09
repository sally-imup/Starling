// Copyright (c) 2026 Sally. Saturn Client, bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// Saturn Client's menu: RShift opens a grid of its features, one card each. A click on a
// card's top turns that feature on or off; a click on its strip at the bottom opens its
// settings page. While the menu is open, a click inside it is taken from the screen under it.
use crate::game::native_hooks::NativeScreenView;
use horizon_client_render::hud::HudBatch;
use std::cell::RefCell;

const KEY_RSHIFT: &str = "key.keyboard.right.shift";
const KEY_ESCAPE: &str = "key.keyboard.escape";

const GAP: f32 = 8.0;
const HEAD_H: f32 = 40.0;
const CARD_H: f32 = 64.0;
const STRIP_H: f32 = 18.0;
const ROW_H: f32 = 22.0;
const BACK_W: f32 = 80.0;

struct Feature {
    name: &'static str,
    /// Button indices in `Settings::click`; the first is the feature's own switch.
    buttons: &'static [usize],
}

const FEATURES: [Feature; 5] = [
    Feature { name: "No Particle", buttons: &[0] },
    Feature { name: "Custom selection box", buttons: &[1, 2, 3] },
    Feature { name: "Loot beams", buttons: &[4, 5] },
    Feature { name: "X-Ray", buttons: &[6] },
    Feature { name: "GUI animation", buttons: &[7] },
];

#[derive(Default)]
struct State {
    open: bool,
    /// The settings page of feature `page`, or None for the grid.
    page: Option<usize>,
    /// Button indices clicked this frame, applied in `frame`.
    pending: Vec<usize>,
    /// The GUI size the menu was last drawn at, for clicks.
    size: [f32; 2],
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Clone, Copy)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Rect {
    fn has(&self, p: [f32; 2]) -> bool {
        p[0] >= self.x && p[0] < self.x + self.w && p[1] >= self.y && p[1] < self.y + self.h
    }
}

struct Layout {
    panel: Rect,
    /// Each card's switch area and its settings strip.
    cards: Vec<(Rect, Rect)>,
    back: Rect,
    /// One per button of the open feature's settings page.
    rows: Vec<Rect>,
}

fn layout(size: [f32; 2], page: Option<usize>) -> Layout {
    let pw = (size[0] * 0.9).min(520.0);
    let ph = size[1] * 0.8;
    let panel = Rect {
        x: (size[0] - pw) * 0.5,
        y: (size[1] - ph) * 0.5,
        w: pw,
        h: ph,
    };
    let top = panel.y + HEAD_H;
    let mut cards = Vec::new();
    let mut rows = Vec::new();
    let mut back = Rect { x: 0.0, y: 0.0, w: 0.0, h: 0.0 };
    match page {
        None => {
            let cw = (pw - GAP * 4.0) / 3.0;
            for i in 0..FEATURES.len() {
                let x = panel.x + GAP + (i % 3) as f32 * (cw + GAP);
                let y = top + (i / 3) as f32 * (CARD_H + STRIP_H + GAP);
                cards.push((
                    Rect { x, y, w: cw, h: CARD_H },
                    Rect { x, y: y + CARD_H, w: cw, h: STRIP_H },
                ));
            }
        }
        Some(f) => {
            back = Rect { x: panel.x + GAP, y: top, w: BACK_W, h: ROW_H };
            for k in 0..FEATURES[f].buttons.len() {
                rows.push(Rect {
                    x: panel.x + GAP,
                    y: top + ROW_H * (k as f32 + 1.5),
                    w: pw - GAP * 2.0,
                    h: ROW_H,
                });
            }
        }
    }
    Layout { panel, cards, back, rows }
}

/// Whether feature `f` is on, from the settings.
fn enabled(s: &crate::game::saturn_client::Settings, f: usize) -> bool {
    match f {
        0 => s.no_particles,
        1 => s.box_on,
        2 => s.beams_on,
        3 => s.xray_on,
        _ => s.gui_anim,
    }
}

/// RShift opens and closes the menu; Esc goes back a page, then closes it.
pub(super) fn key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::saturn_client as saturn;
    if !pressed || !game.native_on(saturn::ID) {
        return;
    }
    match key {
        KEY_RSHIFT => {
            let opening = STATE.with_borrow_mut(|s| {
                s.open = !s.open;
                s.page = None;
                s.open
            });
            // Free the cursor so the menu can be clicked.
            if opening && game.captured {
                game.capture(false);
            }
        }
        KEY_ESCAPE => STATE.with_borrow_mut(|s| {
            if s.open {
                if s.page.is_some() {
                    s.page = None;
                } else {
                    s.open = false;
                }
            }
        }),
        _ => {}
    }
}

/// A mouse press on the open screen (`native_screen_mouse_button_pressed_pre`).
pub(super) fn click(mouse: [f32; 2], button: i32, canceled: &mut bool) {
    STATE.with_borrow_mut(|s| {
        if !s.open {
            return;
        }
        let lay = layout(s.size, s.page);
        if !lay.panel.has(mouse) {
            return;
        }
        *canceled = true;
        if button != 0 {
            return;
        }
        match s.page {
            None => {
                for (i, (card, strip)) in lay.cards.iter().enumerate() {
                    if card.has(mouse) {
                        s.pending.push(FEATURES[i].buttons[0]);
                        return;
                    }
                    if strip.has(mouse) {
                        s.page = Some(i);
                        return;
                    }
                }
            }
            Some(f) => {
                if lay.back.has(mouse) {
                    s.page = None;
                    return;
                }
                for (k, row) in lay.rows.iter().enumerate() {
                    if row.has(mouse) {
                        s.pending.push(FEATURES[f].buttons[k]);
                        return;
                    }
                }
            }
        }
    });
}

/// Once a frame: apply this frame's clicks to the settings.
pub(super) fn frame(game: &mut crate::game::Game<'_>) {
    use crate::game::saturn_client as saturn;
    let pending = STATE.with_borrow_mut(|s| std::mem::take(&mut s.pending));
    if pending.is_empty() {
        return;
    }
    let mut s = saturn::settings(|key| game.native_setting(saturn::ID, key));
    for i in pending {
        s.click(i);
    }
    for (key, value) in s.stored() {
        game.native_set_setting(saturn::ID, key, &value);
    }
}

/// Draws the open menu over the frame's GUI.
pub(super) fn draw(game: &crate::game::Game<'_>, batch: &mut HudBatch) {
    use crate::game::saturn_client as saturn;
    let (open, page) = STATE.with_borrow_mut(|s| {
        s.size = [batch.width, batch.height];
        (s.open, s.page)
    });
    if !open || !game.native_on(saturn::ID) {
        return;
    }
    let atlas = &game.gui.atlas;
    let lay = layout([batch.width, batch.height], page);
    let settings = saturn::settings(|key| game.native_setting(saturn::ID, key));
    let labels = settings.labels();
    let p = lay.panel;
    batch.fill(atlas, p.x, p.y, p.w, p.h, 0xe610_1018);

    let title = "Saturn Client";
    batch.text(atlas, title, p.x + GAP, p.y + 12.0, 0xff57_ffe1, true);
    let hint = if page.is_some() { "Esc: back" } else { "RShift: close" };
    let hint_x = p.x + p.w - HudBatch::text_width(atlas, hint) - GAP;
    batch.text(atlas, hint, hint_x, p.y + 12.0, 0xffaa_aaaa, true);

    match page {
        None => {
            for (i, feature) in FEATURES.iter().enumerate() {
                let (card, strip) = lay.cards[i];
                let on = enabled(&settings, i);
                let fill = if on { 0xff2f_7d55 } else { 0xff3a_3a4a };
                batch.fill(atlas, card.x, card.y, card.w, card.h, fill);
                batch.text(atlas, feature.name, card.x + 6.0, card.y + 8.0, 0xffff_ffff, true);
                let status = if on { "ON" } else { "OFF" };
                batch.text(atlas, status, card.x + 6.0, card.y + 30.0, 0xffff_ffff, true);
                batch.fill(atlas, strip.x, strip.y, strip.w, strip.h, 0xff22_2230);
                batch.text(atlas, "Settings >", strip.x + 6.0, strip.y + 4.0, 0xffaa_aaaa, false);
            }
        }
        Some(f) => {
            let b = lay.back;
            batch.fill(atlas, b.x, b.y, b.w, b.h, 0xff3a_3a4a);
            batch.text(atlas, "< Back", b.x + 6.0, b.y + 6.0, 0xffff_ffff, true);
            let name = FEATURES[f].name;
            batch.text(atlas, name, b.x + b.w + GAP, b.y + 6.0, 0xff57_ffe1, true);
            for (k, row) in lay.rows.iter().enumerate() {
                let label = labels.get(FEATURES[f].buttons[k]).map_or("", String::as_str);
                batch.fill(atlas, row.x, row.y, row.w, row.h, 0xff2a_2a38);
                batch.text(atlas, label, row.x + 6.0, row.y + 6.0, 0xffff_ffff, true);
            }
        }
    }
}

/// While the menu is open, the mouse goes to it: a press inside the panel is its click, a press
/// outside closes it, and every release is held back so the world does not get it.
pub(super) fn takes_mouse(mouse: [f32; 2], button: i32, pressed: bool) -> bool {
    let open = STATE.with_borrow(|s| s.open);
    if !open {
        return false;
    }
    if !pressed {
        return true;
    }
    let mut canceled = false;
    click(mouse, button, &mut canceled);
    if !canceled {
        // A press outside the panel closes the menu, and does not reach the world.
        STATE.with_borrow_mut(|s| {
            s.open = false;
            s.page = None;
        });
    }
    true
}
