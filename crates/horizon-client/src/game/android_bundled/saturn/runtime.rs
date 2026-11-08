// Copyright (c) 2026 Sally. Saturn Client, bundled in the Android client.
// SPDX-License-Identifier: MIT; see LICENSE in this directory.
// Saturn Client's shared state: its id, its settings, and what each one does.
pub const ID: &str = "saturn";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "Saturn Client: client-side tweaks for the Android client. Now: No Particle.";
pub const NO_PARTICLES: &str = "no_particles";

/// The settings, stored as text in the mod's folder (one file per key).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// Hide every particle the game would make.
    pub no_particles: bool,
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            no_particles: get(NO_PARTICLES).is_some_and(|v| v.trim() == "true"),
        }
    }
    /// The settings page's buttons, in order.
    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![format!("No particles: {}", on(self.no_particles))]
    }
    /// A click on button `i`: flip that setting.
    pub fn click(&mut self, i: usize) {
        if i == 0 {
            self.no_particles = !self.no_particles;
        }
    }
    pub fn stored(&self) -> [(&'static str, &'static str); 1] {
        [(NO_PARTICLES, if self.no_particles { "true" } else { "false" })]
    }
}

/// The settings as stored (see `Settings::read`).
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    Settings::read(get)
}
