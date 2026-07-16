//! Pass celebration: a burst of rune-gold sparks with gravity.

use eframe::egui::{self, Color32, Pos2, Vec2};

use crate::theme::{EMBER, RUNE_GOLD};

const COUNT: usize = 90;
const TTL: f32 = 1.4;
const GRAVITY: f32 = 340.0;

struct Spark {
    pos: Pos2,
    vel: Vec2,
    age: f32,
    color: Color32,
    size: f32,
}

#[derive(Default)]
pub struct Particles {
    sparks: Vec<Spark>,
    last_frame: Option<f64>,
}

impl Particles {
    pub fn burst(&mut self, origin: Pos2) {
        // Deterministic pseudo-random fan: cheap, no rng dependency,
        // and nobody can tell with 90 sparks.
        for i in 0..COUNT {
            let t = i as f32 / COUNT as f32;
            let angle = t * std::f32::consts::TAU * 3.7 + (i as f32 * 12.9898).sin() * 0.9;
            let speed = 90.0 + ((i as f32 * 78.233).sin().abs()) * 260.0;
            self.sparks.push(Spark {
                pos: origin,
                vel: Vec2::angled(angle) * speed - Vec2::new(0.0, 120.0),
                age: 0.0,
                color: if i % 3 == 0 { EMBER } else { RUNE_GOLD },
                size: 1.5 + (i % 4) as f32,
            });
        }
        self.last_frame = None;
    }

    /// Returns `true` while sparks are still alive.
    pub fn tick(&mut self, ctx: &egui::Context) -> bool {
        if self.sparks.is_empty() {
            return false;
        }
        let now = ctx.input(|i| i.time);
        let dt = self
            .last_frame
            .map_or(1.0 / 60.0, |last| ((now - last) as f32).clamp(0.0, 0.05));
        self.last_frame = Some(now);

        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("fx-sparks"),
        ));
        self.sparks.retain_mut(|s| {
            s.age += dt;
            if s.age >= TTL {
                return false;
            }
            s.vel.y += GRAVITY * dt;
            s.pos += s.vel * dt;
            let fade = 1.0 - s.age / TTL;
            painter.circle_filled(s.pos, s.size * fade, s.color.linear_multiply(fade));
            true
        });
        !self.sparks.is_empty()
    }
}
