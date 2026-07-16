//! Permadeath: a red vignette swells while a trembling banner rises —
//! the world tells you what happened before any text does.

use eframe::egui::{self, Align2, Color32, FontId, Pos2};

use crate::theme::BLOOD;

const DURATION: f32 = 2.2;

#[derive(Default)]
pub struct DeathFlash {
    started: Option<f64>,
}

impl DeathFlash {
    pub fn start(&mut self) {
        // NAN sentinel: stamped with the real frame time on next tick.
        self.started = Some(f64::NAN);
    }

    /// Returns `true` while the flash is running.
    pub fn tick(&mut self, ctx: &egui::Context) -> bool {
        let Some(mut t0) = self.started else {
            return false;
        };
        let now = ctx.input(|i| i.time);
        if t0.is_nan() {
            t0 = now;
            self.started = Some(now);
        }
        let t = (now - t0) as f32;
        if t >= DURATION {
            self.started = None;
            return false;
        }

        let progress = t / DURATION;
        // Swell fast, linger, then fade.
        let alpha = if progress < 0.15 {
            progress / 0.15
        } else {
            1.0 - (progress - 0.15) / 0.85
        };

        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("fx-death"),
        ));
        let rect = ctx.screen_rect();

        // Vignette: translucent blood over the whole world.
        painter.rect_filled(
            rect,
            0.0,
            Color32::from_rgb(70, 8, 10).linear_multiply(alpha * 0.55),
        );

        // The banner trembles — a cheap shake that never disturbs layout.
        let jitter = Pos2::new(
            ((now * 61.0).sin() as f32) * 3.0 * alpha,
            ((now * 47.0).cos() as f32) * 2.0 * alpha,
        );
        painter.text(
            rect.center() + jitter.to_vec2(),
            Align2::CENTER_CENTER,
            "YOU DIED",
            FontId::proportional(64.0),
            BLOOD.linear_multiply(0.3 + alpha * 0.7),
        );
        true
    }
}
