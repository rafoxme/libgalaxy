//! The Yaru theme: a bundle of design tokens resolved for a color scheme and
//! accent variant.
use crate::accent::AccentColor;
use crate::semantic::{ColorScheme, SemanticColors};
use crate::spacing;
use crate::typography::Typography;
use serde::{Deserialize, Serialize};

/// A complete Yaru theme.
///
/// This is the object stored as a GPUI global by LibGalaxy and read by every
/// widget while rendering. Construct via [`Theme::yaru_light`] /
/// [`Theme::yaru_dark`] and customize with the builder-style setters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    /// The color scheme (light or dark).
    pub color_scheme: ColorScheme,
    /// The active accent color variant.
    pub accent: AccentColor,
    /// The typography scale.
    pub typography: Typography,
    /// The resolved semantic colors for the current scheme/accent.
    pub semantic: SemanticColors,
}

impl Theme {
    /// The light Yaru theme.
    pub fn yaru_light() -> Self {
        Self::new(ColorScheme::Light, AccentColor::default())
    }

    /// The dark Yaru theme.
    pub fn yaru_dark() -> Self {
        Self::new(ColorScheme::Dark, AccentColor::default())
    }

    /// Build a theme from a scheme and accent.
    pub fn new(color_scheme: ColorScheme, accent: AccentColor) -> Self {
        let semantic = SemanticColors::new(color_scheme, accent);
        Self {
            color_scheme,
            accent,
            typography: Typography::yaru(),
            semantic,
        }
    }

    /// The light/dark scheme of this theme.
    pub const fn color_scheme(&self) -> ColorScheme {
        self.color_scheme
    }

    /// The active accent variant.
    pub const fn accent(&self) -> AccentColor {
        self.accent
    }

    /// Swap between light and dark.
    pub fn toggle_scheme(mut self) -> Self {
        self.color_scheme = self.color_scheme.toggle();
        self.recompute();
        self
    }

    /// Change the accent variant, re-resolving semantic colors.
    pub fn set_accent(mut self, accent: AccentColor) -> Self {
        self.accent = accent;
        self.recompute();
        self
    }

    /// Re-resolve semantic colors after a change to scheme or accent.
    fn recompute(&mut self) {
        self.semantic = SemanticColors::new(self.color_scheme, self.accent);
    }

    // -----------------------------------------------------------------------
    // Convenience accessors
    // -----------------------------------------------------------------------

    /// The application background color.
    pub const fn bg(&self) -> crate::Color {
        self.semantic.bg_color
    }

    /// The primary foreground color.
    pub const fn fg(&self) -> crate::Color {
        self.semantic.fg_color
    }

    /// The accent color.
    pub const fn accent_color(&self) -> crate::Color {
        self.semantic.accent_bg_color
    }

    /// The base control border radius.
    pub const fn radius(&self) -> f32 {
        spacing::BASE_BORDER_RADIUS
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::yaru_light()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggling_scheme_updates_colors() {
        let light = Theme::yaru_light();
        let dark = light.clone().toggle_scheme();
        assert_eq!(dark.color_scheme, ColorScheme::Dark);
        assert_ne!(dark.semantic.bg_color, light.semantic.bg_color);
    }

    #[test]
    fn setting_accent_changes_selected_bg() {
        let light = Theme::yaru_light();
        let blue = light.clone().set_accent(AccentColor::Blue);
        assert_eq!(blue.semantic.selected_bg_color, AccentColor::Blue.color());
    }
}
