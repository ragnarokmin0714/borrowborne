//! Combat presentation: floating combat text (MISS / BLOCKED / echo
//! gains) and the kill slash across the monster's health bar.
//!
//! Pure theater — verdicts decide everything; these only perform them.

use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, Stroke};

/// One piece of floating text: rises and fades.
struct FloatText {
    pos: Pos2,
    text: String,
    color: Color32,
    age: f32,
}

const FLOAT_TTL: f32 = 1.2;
const FLOAT_RISE: f32 = 46.0;

#[derive(Default)]
pub struct Floats {
    items: Vec<FloatText>,
    last_frame: Option<f64>,
}

impl Floats {
    pub fn spawn(&mut self, pos: Pos2, text: impl Into<String>, color: Color32) {
        self.items.push(FloatText {
            pos,
            text: text.into(),
            color,
            age: 0.0,
        });
        self.last_frame = None;
    }

    /// Returns `true` while text is still floating.
    pub fn tick(&mut self, ctx: &egui::Context) -> bool {
        if self.items.is_empty() {
            return false;
        }
        let now = ctx.input(|i| i.time);
        let dt = self
            .last_frame
            .map_or(1.0 / 60.0, |last| ((now - last) as f32).clamp(0.0, 0.05));
        self.last_frame = Some(now);

        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("fx-floats"),
        ));
        self.items.retain_mut(|f| {
            f.age += dt;
            if f.age >= FLOAT_TTL {
                return false;
            }
            let t = f.age / FLOAT_TTL;
            let pos = f.pos - egui::vec2(0.0, FLOAT_RISE * t);
            let fade = 1.0 - t * t;
            painter.text(
                pos,
                Align2::CENTER_BOTTOM,
                &f.text,
                FontId::proportional(18.0 + 6.0 * (1.0 - t)),
                f.color.linear_multiply(fade),
            );
            true
        });
        !self.items.is_empty()
    }
}

/// The kill: two crossing streaks slashed over the health bar.
#[derive(Default)]
pub struct Slash {
    rect: Option<Rect>,
    started: Option<f64>,
}

const SLASH_TTL: f32 = 0.45;

impl Slash {
    pub fn start(&mut self, rect: Rect) {
        self.rect = Some(rect);
        self.started = Some(f64::NAN); // stamped on next tick
    }

    /// Returns `true` while the slash is visible.
    pub fn tick(&mut self, ctx: &egui::Context) -> bool {
        let (Some(rect), Some(mut t0)) = (self.rect, self.started) else {
            return false;
        };
        let now = ctx.input(|i| i.time);
        if t0.is_nan() {
            t0 = now;
            self.started = Some(now);
        }
        let t = ((now - t0) as f32) / SLASH_TTL;
        if t >= 1.0 {
            self.started = None;
            return false;
        }

        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("fx-slash"),
        ));
        let fade = 1.0 - t;
        let reach = t.min(1.0);
        // Two diagonal streaks racing across the bar.
        let a0 = rect.left_top() + egui::vec2(0.0, rect.height() * 0.2);
        let a1 = a0 + egui::vec2(rect.width() * reach, rect.height() * 0.6);
        let b0 = rect.right_top() + egui::vec2(0.0, rect.height() * 0.1);
        let b1 = b0 + egui::vec2(-rect.width() * reach, rect.height() * 0.8);
        let white = Color32::from_rgb(240, 226, 200).linear_multiply(fade);
        let red = Color32::from_rgb(158, 34, 40).linear_multiply(fade * 0.8);
        painter.line_segment([a0, a1], Stroke::new(3.0_f32, white));
        painter.line_segment([b0, b1], Stroke::new(2.0_f32, red));
        true
    }
}
