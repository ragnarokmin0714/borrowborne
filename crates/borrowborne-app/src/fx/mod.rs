//! Visual effects. All animation runs through [`Fx::tick`], the one
//! place allowed to call `request_repaint` for animation — egui is
//! reactive, so a running effect must ask for the next frame or it
//! freezes mid-burst.

mod particles;
mod shake;

use eframe::egui;

use particles::Particles;
use shake::DeathFlash;

/// Aggregated effect state, owned by the app.
#[derive(Default)]
pub struct Fx {
    particles: Particles,
    death: DeathFlash,
}

impl Fx {
    /// A gate opened: golden burst from `origin`.
    pub fn on_pass(&mut self, origin: egui::Pos2) {
        self.particles.burst(origin);
    }

    /// Permadeath: red vignette + trembling banner.
    pub fn on_death(&mut self) {
        self.death.start();
    }

    /// Advance and paint every live effect. Call once per frame, after
    /// the panels, so effects draw on top.
    pub fn tick(&mut self, ctx: &egui::Context) {
        let mut alive = false;
        alive |= self.particles.tick(ctx);
        alive |= self.death.tick(ctx);
        if alive {
            ctx.request_repaint();
        }
    }
}
