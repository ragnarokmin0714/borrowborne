//! Sound: a tiny procedurally-synthesized SFX bank played through kira.
//!
//! No audio assets, no licensing, no downloads — every sound is a few
//! lines of math rendered once at init. The manager is created lazily
//! on the first cast (a click), which doubles as the user gesture
//! browsers demand before an AudioContext may speak.

use std::sync::Arc;

use kira::sound::static_sound::StaticSoundData;
use kira::{AudioManager, AudioManagerSettings, DefaultBackend, Frame};

const SAMPLE_RATE: u32 = 44_100;

/// Every sound the game can make.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Sfx {
    /// The spell leaves your lips.
    Cast,
    /// The gate opens: slash + chime.
    Kill,
    /// Compile error: a dull whiff.
    Miss,
    /// Trial failed: metallic parry.
    Blocked,
    /// The curse rejects the spell outright.
    Cursed,
    /// Permadeath: low boom and rumble.
    Death,
    /// Lost in the loop.
    Timeout,
}

pub struct Audio {
    manager: AudioManager,
    bank: Vec<(Sfx, StaticSoundData)>,
}

impl Audio {
    /// Open the audio device and render the bank. `None` when there is
    /// no device (CI, headless) — the game plays on, silently.
    pub fn try_new() -> Option<Self> {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).ok()?;
        let bank = vec![
            (Sfx::Cast, render(cast_wave, 0.16)),
            (Sfx::Kill, render(kill_wave, 0.35)),
            (Sfx::Miss, render(miss_wave, 0.14)),
            (Sfx::Blocked, render(blocked_wave, 0.20)),
            (Sfx::Cursed, render(cursed_wave, 0.30)),
            (Sfx::Death, render(death_wave, 1.10)),
            (Sfx::Timeout, render(timeout_wave, 0.45)),
        ];
        Some(Self { manager, bank })
    }

    pub fn play(&mut self, sfx: Sfx) {
        if let Some((_, sound)) = self.bank.iter().find(|(s, _)| *s == sfx) {
            let _ = self.manager.play(sound.clone());
        }
    }
}

/// Render `wave(t, dur) -> sample` into a mono sound of `dur` seconds.
fn render(wave: fn(f32, f32) -> f32, dur: f32) -> StaticSoundData {
    let n = (SAMPLE_RATE as f32 * dur) as usize;
    let frames: Arc<[Frame]> = (0..n)
        .map(|i| {
            let t = i as f32 / SAMPLE_RATE as f32;
            // Master gain + a 2 ms fade-out to avoid end clicks.
            let tail = ((dur - t) / 0.002).clamp(0.0, 1.0);
            Frame::from_mono(wave(t, dur).clamp(-1.0, 1.0) * 0.5 * tail)
        })
        .collect();
    StaticSoundData {
        sample_rate: SAMPLE_RATE,
        frames,
        settings: Default::default(),
        slice: None,
    }
}

const TAU: f32 = std::f32::consts::TAU;

/// Deterministic white noise; no rng crate for a hiss.
fn noise(t: f32) -> f32 {
    let x = (t * SAMPLE_RATE as f32) as u64;
    let h = x
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((h >> 33) as f32 / u32::MAX as f32) * 2.0 - 1.0
}

fn decay(t: f32, rate: f32) -> f32 {
    (-t * rate).exp()
}

/// A whoosh: filtered-ish noise swelling then dying, over a low hum.
fn cast_wave(t: f32, dur: f32) -> f32 {
    let swell = (t / dur * TAU / 2.0).sin();
    noise(t) * 0.4 * swell + (TAU * 110.0 * t).sin() * 0.2 * swell
}

/// Slash: a saw sweeping down, then a bright chime on top.
fn kill_wave(t: f32, _dur: f32) -> f32 {
    let f = 900.0 - 700.0 * (t / 0.25).min(1.0);
    let sweep = ((t * f) % 1.0 * 2.0 - 1.0) * decay(t, 12.0) * 0.6;
    let chime = (TAU * 1970.0 * t).sin() * decay(t, 9.0) * 0.35;
    sweep + chime
}

/// A dull thud that never connected.
fn miss_wave(t: f32, _dur: f32) -> f32 {
    (TAU * 90.0 * t).sin() * decay(t, 30.0)
}

/// Metal on metal: two inharmonic partials, struck.
fn blocked_wave(t: f32, _dur: f32) -> f32 {
    let d = decay(t, 22.0);
    ((TAU * 620.0 * t).sin() * 0.5 + (TAU * 1711.0 * t).sin() * 0.35) * d
}

/// Two tones a tritone apart — the classic "forbidden" interval.
fn cursed_wave(t: f32, _dur: f32) -> f32 {
    let d = decay(t, 8.0);
    ((TAU * 311.0 * t).sin() + (TAU * 440.0 * t).sin()) * 0.4 * d
}

/// The night claims another: sub boom + slow rumble.
fn death_wave(t: f32, _dur: f32) -> f32 {
    let boom = (TAU * (55.0 - 20.0 * t) * t).sin() * decay(t, 3.0) * 0.9;
    let rumble = noise(t) * 0.15 * decay(t, 4.0);
    boom + rumble
}

/// A wobbling tone wandering off into the dark.
fn timeout_wave(t: f32, _dur: f32) -> f32 {
    let wobble = 440.0 + 12.0 * (TAU * 5.0 * t).sin();
    (TAU * wobble * t).sin() * decay(t, 6.0) * 0.6
}
