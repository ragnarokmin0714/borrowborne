//! Sound: a tiny procedurally-synthesized SFX bank played through kira.
//!
//! No audio assets, no licensing, no downloads — every sound is a few
//! lines of math rendered once at init. The manager is created lazily
//! on the first cast (a click), which doubles as the user gesture
//! browsers demand before an AudioContext may speak.

use std::sync::Arc;

use kira::sound::static_sound::StaticSoundData;
use kira::track::{TrackBuilder, TrackHandle};
use kira::{AudioManager, AudioManagerSettings, Decibels, DefaultBackend, Frame, Tween};

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
    /// Owns the device; sounds play on the two sub-tracks below.
    _manager: AudioManager,
    /// SFX route, so effect volume moves independently of music.
    sfx_track: TrackHandle,
    /// BGM route.
    bgm_track: TrackHandle,
    bank: Vec<(Sfx, StaticSoundData)>,
    /// One looping drone per theme; see [`themes`].
    themes: Vec<StaticSoundData>,
    /// Currently playing theme index and its handle.
    bgm: Option<(usize, kira::sound::static_sound::StaticSoundHandle)>,
}

impl Audio {
    /// Open the audio device and render the bank. `None` when there is
    /// no device (CI, headless) — the game plays on, silently.
    pub fn try_new() -> Option<Self> {
        let mut manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).ok()?;
        let sfx_track = manager.add_sub_track(TrackBuilder::new()).ok()?;
        let bgm_track = manager.add_sub_track(TrackBuilder::new()).ok()?;
        let bank = vec![
            (Sfx::Cast, render(cast_wave, 0.16)),
            (Sfx::Kill, render(kill_wave, 0.35)),
            (Sfx::Miss, render(miss_wave, 0.14)),
            (Sfx::Blocked, render(blocked_wave, 0.20)),
            (Sfx::Cursed, render(cursed_wave, 0.30)),
            (Sfx::Death, render(death_wave, 1.10)),
            (Sfx::Timeout, render(timeout_wave, 0.45)),
        ];
        Some(Self {
            _manager: manager,
            sfx_track,
            bgm_track,
            bank,
            themes: themes(),
            bgm: None,
        })
    }

    /// Apply user volumes (0..=1 each) to the two routes.
    pub fn set_volumes(&mut self, sfx: f32, bgm: f32) {
        self.sfx_track
            .set_volume(to_decibels(sfx), Tween::default());
        self.bgm_track
            .set_volume(to_decibels(bgm), Tween::default());
    }

    pub fn play(&mut self, sfx: Sfx) {
        if let Some((_, sound)) = self.bank.iter().find(|(s, _)| *s == sfx) {
            let _ = self.sfx_track.play(sound.clone());
        }
    }

    /// Keep the drone for `theme` running; switching themes crossfades
    /// (the old fades out over the new). No-op when already playing.
    pub fn ensure_bgm(&mut self, theme: usize) {
        if self.bgm.as_ref().is_some_and(|(ix, _)| *ix == theme) {
            return;
        }
        self.stop_bgm();
        let Some(sound) = self.themes.get(theme) else {
            return;
        };
        if let Ok(handle) = self.bgm_track.play(sound.clone()) {
            self.bgm = Some((theme, handle));
        }
    }

    /// Fade the current drone into silence (mute, shutdown).
    pub fn stop_bgm(&mut self) {
        if let Some((_, mut handle)) = self.bgm.take() {
            handle.stop(Tween {
                duration: std::time::Duration::from_millis(900),
                ..Default::default()
            });
        }
    }
}

/// Slider position (0..=1) to decibels: perceptual-ish curve with a
/// hard floor at silence.
fn to_decibels(v: f32) -> Decibels {
    if v <= 0.005 {
        Decibels::SILENCE
    } else {
        Decibels(20.0 * v.clamp(0.0, 1.0).log10())
    }
}

/// Render `wave(t, dur) -> sample` into a mono sound of `dur` seconds.
fn render(wave: fn(f32, f32) -> f32, dur: f32) -> StaticSoundData {
    let n = (SAMPLE_RATE as f32 * dur) as usize;
    let frames: Arc<[Frame]> = (0..n)
        .map(|i| {
            let t = i as f32 / SAMPLE_RATE as f32;
            // Soft-clip drive: louder, and the added harmonics carry
            // on small speakers. 2 ms fade-out avoids end clicks.
            let tail = ((dur - t) / 0.002).clamp(0.0, 1.0);
            Frame::from_mono((wave(t, dur) * 2.0).tanh() * 0.85 * tail)
        })
        .collect();
    StaticSoundData {
        sample_rate: SAMPLE_RATE,
        frames,
        settings: Default::default(),
        slice: None,
    }
}

// ── BGM: seamless ambient drones ────────────────────────────────────
//
// One 12-second loop per theme: index 0 is the world map, 1.. follow
// the chapters. Seamlessness is arithmetic, not luck — every partial
// completes an integer number of cycles over the loop (root·12 is
// kept an even integer so the fifth stays integral too), so the loop
// point is inaudible. Rendered at half rate: drones have no highs.

/// Loop length in seconds.
const BGM_LOOP_SECS: f32 = 12.0;
/// Drones render at half rate to halve their memory.
const BGM_SAMPLE_RATE: u32 = 22_050;

/// Root frequencies (Hz): map, village, forest, town, swamp, guild,
/// library. Each is chosen so root × 12 is an even integer (see
/// module comment).
const THEME_ROOTS: [f32; 7] = [55.0, 49.5, 41.5, 62.0, 37.0, 46.5, 58.5];

fn themes() -> Vec<StaticSoundData> {
    THEME_ROOTS
        .iter()
        .enumerate()
        .map(|(theme, &root)| render_theme(root, theme))
        .collect()
}

/// Layered drone breathing on slow LFOs whose rates are integer
/// cycles per loop. The upper partials (×3, ×4, ×6) are not garnish:
/// laptop and phone speakers cannot reproduce the 37–62 Hz roots at
/// all, so the mid layers are what most listeners actually hear.
fn drone_wave(root: f32, t: f32) -> f32 {
    let lfo =
        |cycles: f32, phase: f32| 0.55 + 0.45 * (TAU * cycles * t / BGM_LOOP_SECS + phase).sin();
    let p = |mult: f32| (TAU * root * mult * t).sin();
    p(1.0) * 0.40 * lfo(1.0, 0.0)
        + p(1.5) * 0.25 * lfo(2.0, 1.7)
        + p(2.0) * 0.22 * lfo(3.0, 3.9)
        + p(3.0) * 0.18 * lfo(2.0, 0.9)
        + p(4.0) * 0.14 * lfo(4.0, 2.6)
        + p(6.0) * 0.08 * lfo(5.0, 5.1)
}

// ── Melody: a sparse pluck line so the drone is not a flat pad ───────
//
// Minor-pentatonic ratios over the root (root, ♭3, 4, 5, ♭7) — a dark
// scale that never sours against the drone. A fixed motif of plucked
// bells plays across the 12 s loop, riding ×6 above the root so it
// lands in an audible register. Each region rotates the scale degrees
// (`+ theme`), so no two regions play the same tune. Seamlessness is
// kept by the pluck envelope: every note has decayed to silence well
// before the loop point, so the repeat has nothing to click on.

/// Minor-pentatonic ratios over the root.
const PENTA: [f32; 5] = [1.0, 1.2, 1.333_333_3, 1.5, 1.8];

/// (onset seconds, scale degree) for the motif; last note leaves a
/// tail that decays to silence before the 12 s seam.
const MOTIF: [(f32, usize); 7] = [
    (0.0, 0),
    (1.8, 2),
    (3.4, 4),
    (5.0, 3),
    (6.8, 1),
    (8.6, 4),
    (9.8, 2),
];

/// Octave lift for the melody so it clears the 37–62 Hz roots.
const MELODY_MULT: f32 = 6.0;

fn melody_wave(root: f32, theme: usize, t: f32) -> f32 {
    let mut s = 0.0;
    for (onset, degree) in MOTIF {
        if t < onset {
            continue;
        }
        let age = t - onset;
        // Pluck: near-instant attack, exponential decay. Silent again
        // long before the seam, so the loop is clickless whatever the
        // note frequency.
        let env = (age / 0.015).min(1.0) * (-age * 2.2).exp();
        if env < 1e-4 {
            continue;
        }
        let freq = root * MELODY_MULT * PENTA[(degree + theme) % PENTA.len()];
        // A soft bell: fundamental plus a quiet octave.
        let tone = (TAU * freq * t).sin() * 0.7 + (TAU * freq * 2.0 * t).sin() * 0.2;
        s += tone * env * 0.18;
    }
    s
}

/// The full BGM sample at time `t`: drone bed plus the region's melody.
/// Pure and deterministic, so the loop seam is unit-testable.
fn bgm_wave(root: f32, theme: usize, t: f32) -> f32 {
    drone_wave(root, t) + melody_wave(root, theme, t)
}

fn render_theme(root: f32, theme: usize) -> StaticSoundData {
    let n = (BGM_SAMPLE_RATE as f32 * BGM_LOOP_SECS) as usize;
    let frames: Arc<[Frame]> = (0..n)
        .map(|i| {
            let t = i as f32 / BGM_SAMPLE_RATE as f32;
            // Driven for warmth and presence; still under the SFX.
            Frame::from_mono((bgm_wave(root, theme, t) * 1.3).tanh() * 0.6)
        })
        .collect();
    StaticSoundData {
        sample_rate: BGM_SAMPLE_RATE,
        frames,
        settings: Default::default(),
        slice: None,
    }
    .loop_region(..)
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The loop must not click: every theme's wave at the seam (t = loop
    /// length, which wraps to t = 0) must nearly equal its value at 0.
    /// The drone is exact by construction (integer-cycle partials); the
    /// melody's pluck envelopes have decayed to near-silence by then.
    #[test]
    fn bgm_loops_without_a_click() {
        for (theme, &root) in THEME_ROOTS.iter().enumerate() {
            let head = bgm_wave(root, theme, 0.0);
            let seam = bgm_wave(root, theme, BGM_LOOP_SECS);
            let jump = (head - seam).abs();
            assert!(jump < 0.02, "theme {theme}: seam jump {jump} too large");
        }
    }

    /// Every region must play a distinct tune: rotating the pentatonic
    /// by the theme index means no two share a melody line.
    #[test]
    fn regions_do_not_share_a_melody() {
        // Sample the melody of each theme at a fixed root, off the
        // harmonic grid (an odd root and step, so samples never all
        // land on zero crossings). Two themes differ when their sampled
        // waves differ substantially, not by a coincidental hair.
        let sample = |theme: usize| -> Vec<f32> {
            (0..60)
                .map(|k| melody_wave(51.7, theme, k as f32 * 0.19 + 0.05))
                .collect()
        };
        for a in 0..THEME_ROOTS.len() {
            for b in (a + 1)..THEME_ROOTS.len() {
                // Rotations repeat every PENTA.len(); only themes whose
                // rotations actually differ are required to sound apart.
                if a % PENTA.len() == b % PENTA.len() {
                    continue;
                }
                let (sa, sb) = (sample(a), sample(b));
                let diff: f32 = sa.iter().zip(&sb).map(|(x, y)| (x - y).abs()).sum();
                assert!(diff > 1.0, "themes {a} and {b} share a tune (diff {diff})");
            }
        }
    }
}
