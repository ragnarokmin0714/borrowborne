//! The gothic skin: near-black panels, bone-parchment text, one blood
//! accent. Dark only — hunters do not fight at noon.

use eframe::egui::{self, Color32, Rounding, Stroke, Visuals};

/// Height of one standard control in toolbar rows.
pub const CONTROL_HEIGHT: f32 = 26.0;

/// The blood accent: verdict highlights, the Cast button, death text.
pub const BLOOD: Color32 = Color32::from_rgb(158, 34, 40);
/// Softer ember tone for hover states and progress.
pub const EMBER: Color32 = Color32::from_rgb(196, 84, 52);
/// Bone-parchment foreground text.
pub const BONE: Color32 = Color32::from_rgb(214, 204, 184);
/// Faded rune-gold for passes and solved badges.
pub const RUNE_GOLD: Color32 = Color32::from_rgb(190, 160, 92);

pub fn apply(ctx: &egui::Context) {
    let mut v = Visuals::dark();

    v.override_text_color = Some(BONE);
    v.panel_fill = Color32::from_rgb(18, 16, 17);
    v.window_fill = Color32::from_rgb(24, 21, 22);
    v.extreme_bg_color = Color32::from_rgb(12, 11, 11); // editor bed
    v.faint_bg_color = Color32::from_rgb(30, 26, 27);

    v.selection.bg_fill = BLOOD.linear_multiply(0.55);
    v.hyperlink_color = EMBER;

    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(52, 44, 45));
    v.widgets.inactive.bg_fill = Color32::from_rgb(36, 31, 32);
    v.widgets.inactive.weak_bg_fill = Color32::from_rgb(36, 31, 32);
    v.widgets.hovered.bg_fill = Color32::from_rgb(52, 42, 43);
    v.widgets.hovered.weak_bg_fill = Color32::from_rgb(52, 42, 43);
    v.widgets.active.bg_fill = BLOOD.linear_multiply(0.7);

    let rounding = Rounding::same(4.0);
    v.widgets.noninteractive.rounding = rounding;
    v.widgets.inactive.rounding = rounding;
    v.widgets.hovered.rounding = rounding;
    v.widgets.active.rounding = rounding;
    v.window_rounding = Rounding::same(6.0);

    ctx.set_visuals(v);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.interact_size.y = CONTROL_HEIGHT;

    // egui's defaults run small (Body ~12.5, Monospace ~12), and the
    // spell editor renders in Monospace — the most-read text in the
    // game. Bump every text style so nothing strains the eyes, the
    // code editor most of all.
    // Base sizes at 100%; the top-bar A-/A+ control scales the whole
    // UI on top of these (egui zoom) for big or high-DPI screens.
    use egui::{FontFamily, FontId, TextStyle};
    style.text_styles = [
        (
            TextStyle::Small,
            FontId::new(13.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(16.0, FontFamily::Proportional)),
        (
            TextStyle::Button,
            FontId::new(16.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Heading,
            FontId::new(23.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(18.0, FontFamily::Monospace),
        ),
    ]
    .into();

    ctx.set_style(style);
}
