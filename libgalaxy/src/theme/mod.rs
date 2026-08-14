//! Yaru theme integration with GPUI.
//!
//! The actual design tokens live in the [`libgalaxy_theme`] crate. This module
//! stores the active [`Theme`] in GPUI's global state and converts token
//! colors into GPUI colors for rendering.
use gpui::{App, Global, Hsla, Rgba};
use libgalaxy_theme as galaxy_theme;

pub use galaxy_theme::*;

/// The theme stored as a GPUI global. Widgets read this during `render`.
pub struct ThemeGlobal(pub Theme);

impl Global for ThemeGlobal {}

/// Install the given theme into the application context.
///
/// Call this from your `Application` setup before opening any window:
///
/// ```rust,ignore
/// libgalaxy::set_theme(cx, libgalaxy::Theme::yaru_light());
/// ```
pub fn set_theme(cx: &mut App, theme: Theme) {
    cx.set_global(ThemeGlobal(theme));
}

/// Read the active theme.
///
/// Falls back to the default Yaru light theme when none has been installed.
pub fn theme(cx: &App) -> Theme {
    theme_ref(cx).cloned().unwrap_or_else(Theme::yaru_light)
}

/// Read the active theme, returning `None` if no theme has been installed.
pub fn theme_ref(cx: &App) -> Option<&Theme> {
    cx.try_global::<ThemeGlobal>().map(|g| &g.0)
}

/// Update the active theme in place.
pub fn update_theme(cx: &mut App, f: impl FnOnce(Theme) -> Theme) {
    let next = f(theme(cx));
    set_theme(cx, next);
}

/// Convert a theme color into a GPUI color.
pub fn to_gpui_color(color: Color) -> Hsla {
    Hsla::from(Rgba {
        r: color.r() as f32 / 255.0,
        g: color.g() as f32 / 255.0,
        b: color.b() as f32 / 255.0,
        a: color.a() as f32 / 255.0,
    })
}

/// Convert a theme color with an explicit alpha (`0..=1`) into a GPUI color.
pub fn to_gpui_color_alpha(color: Color, alpha: f32) -> Hsla {
    let mut c = to_gpui_color(color);
    c.a = alpha.clamp(0.0, 1.0);
    c
}

/// The font weight of a [`TextScale`] as a GPUI font weight.
pub fn to_gpui_weight(weight: FontWeight) -> gpui::FontWeight {
    gpui::FontWeight(weight.0)
}

/// A pixel size in GPUI units.
pub fn to_gpui_px(size: f32) -> gpui::Pixels {
    gpui::px(size)
}

/// The GPUI `Pixels` type, re-exported for convenience.
pub use gpui::px;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_conversion_is_consistent() {
        let c = Color::from_hex(0xE95420);
        let g = to_gpui_color(c);
        let back = Rgba::from(g);
        assert!((back.r - 0xE9 as f32 / 255.0).abs() < 1e-3);
    }
}
