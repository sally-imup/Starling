// Copyright (c) 2026 Sally. Saturn Client, bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// Saturn Client's shared state: its id, its settings, and what each one does.
pub const ID: &str = "saturn";
pub const VERSION: &str = "0.3.0";
pub const DESCRIPTION: &str = "Saturn Client: client-side tweaks for the Android client. No Particle, a custom block selection box and loot beams.";
pub const NO_PARTICLES: &str = "no_particles";
pub const BOX: &str = "box";
pub const BOX_COLOR: &str = "box_color";
pub const BOX_WIDTH: &str = "box_width";
pub const BEAMS: &str = "beams";
pub const BEAM_COLOR: &str = "beam_color";

/// The colours the box and the beams pick from: name and ARGB.
pub const COLORS: [(&str, u32); 6] = [
    ("White", 0xffff_ffff),
    ("Cyan", 0xff57_ffe1),
    ("Red", 0xffff_5555),
    ("Yellow", 0xffff_ff55),
    ("Green", 0xff55_ff55),
    ("Black", 0xff00_0000),
];
/// The selection box's width scales (times the game's own outline width).
pub const WIDTHS: [f32; 5] = [0.5, 1.0, 1.5, 2.0, 3.0];

/// The settings, stored as text in the mod's folder (one file per key).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// Hide every particle the game would make.
    pub no_particles: bool,
    /// Draw the block selection box in the colour and width below.
    pub box_on: bool,
    /// Index into `COLORS`.
    pub box_color: usize,
    /// Index into `WIDTHS`.
    pub box_width: usize,
    /// A column of particles above each item on the ground.
    pub beams_on: bool,
    /// Index into `COLORS`.
    pub beam_color: usize,
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let index = |key: &str, default: usize, len: usize| {
            get(key)
                .and_then(|v| v.trim().parse::<usize>().ok())
                .filter(|&i| i < len)
                .unwrap_or(default)
        };
        Self {
            no_particles: get(NO_PARTICLES).is_some_and(|v| v.trim() == "true"),
            box_on: get(BOX).is_some_and(|v| v.trim() == "true"),
            box_color: index(BOX_COLOR, 1, COLORS.len()),
            box_width: index(BOX_WIDTH, 1, WIDTHS.len()),
            beams_on: get(BEAMS).is_some_and(|v| v.trim() == "true"),
            beam_color: index(BEAM_COLOR, 1, COLORS.len()),
        }
    }
    /// The settings page's buttons, in order.
    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("No particles: {}", on(self.no_particles)),
            format!("Custom selection box: {}", on(self.box_on)),
            format!("Box colour: {}", COLORS[self.box_color].0),
            format!("Box width: {}x", WIDTHS[self.box_width]),
            format!("Loot beams: {}", on(self.beams_on)),
            format!("Beam colour: {}", COLORS[self.beam_color].0),
        ]
    }
    /// A click on button `i`: change that setting.
    pub fn click(&mut self, i: usize) {
        match i {
            0 => self.no_particles = !self.no_particles,
            1 => self.box_on = !self.box_on,
            2 => self.box_color = (self.box_color + 1) % COLORS.len(),
            3 => self.box_width = (self.box_width + 1) % WIDTHS.len(),
            4 => self.beams_on = !self.beams_on,
            5 => self.beam_color = (self.beam_color + 1) % COLORS.len(),
            _ => {}
        }
    }
    pub fn stored(&self) -> [(&'static str, String); 6] {
        let flag = |b: bool| String::from(if b { "true" } else { "false" });
        [
            (NO_PARTICLES, flag(self.no_particles)),
            (BOX, flag(self.box_on)),
            (BOX_COLOR, self.box_color.to_string()),
            (BOX_WIDTH, self.box_width.to_string()),
            (BEAMS, flag(self.beams_on)),
            (BEAM_COLOR, self.beam_color.to_string()),
        ]
    }
    /// The outline to draw, as (ARGB, width scale), or None when the box is off.
    pub fn box_style(&self) -> Option<(u32, f32)> {
        self.box_on
            .then(|| (COLORS[self.box_color].1, WIDTHS[self.box_width]))
    }
}

/// The settings as stored (see `Settings::read`).
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    Settings::read(get)
}
