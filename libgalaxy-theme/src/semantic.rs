//! Semantic colors: the "meaningful" tokens derived from the base palette for
//! light and dark color schemes.
//!
//! Mirrors the Yaru SCSS semantic definitions:
//!
//! ```scss
//! $base_color:            if(light, #ffffff, lighten($jet, 6%));
//! $text_color:            if(light, black, white);
//! $bg_color:              if(light, #FAFAFA, lighten($jet, 8%));
//! $fg_color:              if(light, $inkstone, $porcelain);
//! $selected_bg_color:     $accent_bg_color;
//! $borders_color:         if(light, darken($bg_color, 20%), darken($bg_color, 8%));
//! $warning_color:         $yellow;
//! $error_color:           $red;
//! $success_color:         lighten($green, 5%);
//! $destructive_color:     if(light, $red, darken($red, 10%));
//! $suggested_bg_color:    if($accent_bg_color == $orange, green-based, $accent_bg_color);
//! $progress_bg_color:     $accent_bg_color;
//! $checkradio_bg_color:   $accent_bg_color;
//! ```
use crate::accent::AccentColor;
use crate::color::Color;
use crate::palette;
use serde::{Deserialize, Serialize};

/// Whether the theme targets a light or a dark color scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ColorScheme {
    /// The light color scheme.
    #[default]
    Light,
    /// The dark color scheme.
    Dark,
}

impl ColorScheme {
    /// The CSS-style name of this scheme (`"light"` / `"dark"`).
    pub const fn css_name(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    /// Toggle between light and dark.
    pub const fn toggle(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

/// The green-based "suggested action" color used when the accent is default
/// orange (light scheme).
const SUGGESTED_GREEN_LIGHT: u32 = 0x2C8B1B;
/// The green-based "suggested action" color used when the accent is default
/// orange (dark scheme).
const SUGGESTED_GREEN_DARK: u32 = 0x249723;

/// The complete set of semantic colors for one scheme/accent combination.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SemanticColors {
    /// `$base_color` — the window/entry base color.
    pub base_color: Color,
    /// `$text_color` — primary text on `base_color`.
    pub text_color: Color,
    /// `$bg_color` — the application background.
    pub bg_color: Color,
    /// `$fg_color` — foreground text.
    pub fg_color: Color,
    /// `$selected_bg_color` — text selection background (usually the accent).
    pub selected_bg_color: Color,
    /// `$selected_fg_color` — text on selected backgrounds.
    pub selected_fg_color: Color,
    /// `$borders_color` — control borders.
    pub borders_color: Color,
    /// `$borders_strong_color` — a stronger variant of `borders_color`.
    pub borders_strong_color: Color,
    /// `$warning_color`.
    pub warning_color: Color,
    /// `$error_color`.
    pub error_color: Color,
    /// `$success_color`.
    pub success_color: Color,
    /// `$destructive_color`.
    pub destructive_color: Color,
    /// `$suggested_bg_color` — "recommended" buttons (green when default accent).
    pub suggested_bg_color: Color,
    /// `$progress_bg_color`.
    pub progress_bg_color: Color,
    /// `$checkradio_bg_color`.
    pub checkradio_bg_color: Color,
    /// `$link_color` — hyperlinks.
    pub link_color: Color,
    /// `$accent_bg_color` — the accent color itself.
    pub accent_bg_color: Color,
    /// `$accent_fg_color` — text placed on top of the accent.
    pub accent_fg_color: Color,
    /// Header bar background.
    pub headerbar_bg_color: Color,
    /// Header bar foreground.
    pub headerbar_fg_color: Color,
    /// Header bar border.
    pub headerbar_border_color: Color,
    /// Sidebar background.
    pub sidebar_bg_color: Color,
    /// View / content area background.
    pub view_bg_color: Color,
    /// Card background (`AdwPreferencesGroup`).
    pub card_bg_color: Color,
    /// Popover / menu background.
    pub menu_bg_color: Color,
    /// Tooltip background.
    pub tooltip_bg_color: Color,
    /// Insensitive (disabled) foreground.
    pub insensitive_fg_color: Color,
    /// Insensitive background.
    pub insensitive_bg_color: Color,
    /// Backdrop (unfocused) foreground.
    pub backdrop_fg_color: Color,
    /// Shadow color used for elevation.
    pub shadow_color: Color,
}

impl SemanticColors {
    /// Compute the semantic colors for a given scheme and accent.
    pub fn new(scheme: ColorScheme, accent: AccentColor) -> Self {
        let accent_color = accent.color();
        let base_color = match scheme {
            ColorScheme::Light => Color::from_hex(0xFFFFFF),
            ColorScheme::Dark => palette::JET.lighten(0.06),
        };
        let text_color = match scheme {
            ColorScheme::Light => Color::from_hex(0x000000),
            ColorScheme::Dark => Color::from_hex(0xFFFFFF),
        };
        let bg_color = match scheme {
            ColorScheme::Light => Color::from_hex(0xFAFAFA),
            ColorScheme::Dark => palette::JET.lighten(0.08),
        };
        let fg_color = match scheme {
            ColorScheme::Light => palette::INKSTONE,
            ColorScheme::Dark => palette::PORCELAIN,
        };
        let borders_color = match scheme {
            ColorScheme::Light => bg_color.darken(0.20),
            ColorScheme::Dark => bg_color.darken(0.08),
        };
        let borders_strong_color = match scheme {
            ColorScheme::Light => borders_color.darken(0.25),
            ColorScheme::Dark => borders_color.lighten(0.10),
        };
        let warning_color = palette::YELLOW;
        let error_color = palette::RED;
        let success_color = palette::GREEN.lighten(0.05);
        let destructive_color = match scheme {
            ColorScheme::Light => palette::RED,
            ColorScheme::Dark => palette::RED.darken(0.10),
        };
        let suggested_bg_color = if accent.is_default() {
            match scheme {
                ColorScheme::Light => Color::from_hex(SUGGESTED_GREEN_LIGHT),
                ColorScheme::Dark => Color::from_hex(SUGGESTED_GREEN_DARK),
            }
        } else {
            accent_color
        };
        let accent_fg_color = if accent_color.lightness_ratio() > 0.55 {
            Color::from_hex(0x000000)
        } else {
            Color::from_hex(0xFFFFFF)
        };

        let headerbar_bg_color = match scheme {
            ColorScheme::Light => Color::from_hex(0xFFFFFF),
            ColorScheme::Dark => base_color,
        };
        let headerbar_fg_color = fg_color;
        let headerbar_border_color = match scheme {
            ColorScheme::Light => borders_color,
            ColorScheme::Dark => borders_color.darken(0.10),
        };
        let sidebar_bg_color = match scheme {
            ColorScheme::Light => Color::from_hex(0xF3F3F3),
            ColorScheme::Dark => Color::from_hex(0x232323),
        };
        let view_bg_color = bg_color;
        let card_bg_color = match scheme {
            ColorScheme::Light => Color::from_hex(0xFFFFFF),
            ColorScheme::Dark => bg_color.lighten(0.03),
        };
        let menu_bg_color = match scheme {
            ColorScheme::Light => Color::from_hex(0xFFFFFF),
            ColorScheme::Dark => base_color,
        };
        let tooltip_bg_color = match scheme {
            ColorScheme::Light => Color::from_rgba(0x2C, 0x2C, 0x2C, 0xED),
            ColorScheme::Dark => Color::from_rgba(0xE0, 0xE0, 0xE0, 0xED),
        };
        let insensitive_fg_color = match scheme {
            ColorScheme::Light => palette::ASH,
            ColorScheme::Dark => palette::INKSTONE,
        };
        let insensitive_bg_color = match scheme {
            ColorScheme::Light => Color::from_hex(0xF7F7F7),
            ColorScheme::Dark => Color::from_hex(0x202020),
        };
        let backdrop_fg_color = match scheme {
            ColorScheme::Light => palette::GRAPHITE,
            ColorScheme::Dark => palette::SLATE,
        };
        let shadow_color = match scheme {
            ColorScheme::Light => Color::from_rgba(0, 0, 0, 0x33),
            ColorScheme::Dark => Color::from_rgba(0, 0, 0, 0x66),
        };

        Self {
            base_color,
            text_color,
            bg_color,
            fg_color,
            selected_bg_color: accent_color,
            selected_fg_color: accent_fg_color,
            borders_color,
            borders_strong_color,
            warning_color,
            error_color,
            success_color,
            destructive_color,
            suggested_bg_color,
            progress_bg_color: accent_color,
            checkradio_bg_color: accent_color,
            link_color: palette::LINK_BLUE,
            accent_bg_color: accent_color,
            accent_fg_color,
            headerbar_bg_color,
            headerbar_fg_color,
            headerbar_border_color,
            sidebar_bg_color,
            view_bg_color,
            card_bg_color,
            menu_bg_color,
            tooltip_bg_color,
            insensitive_fg_color,
            insensitive_bg_color,
            backdrop_fg_color,
            shadow_color,
        }
    }
}

impl Color {
    /// The relative luminance of this color (`0.0..=1.0`).
    pub fn luminance(&self) -> f32 {
        let linearize = |c: f32| {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        let r = linearize(self.r() as f32 / 255.0);
        let g = linearize(self.g() as f32 / 255.0);
        let b = linearize(self.b() as f32 / 255.0);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// A rough `0..=1` lightness ratio, used to pick readable foreground text.
    pub fn lightness_ratio(&self) -> f32 {
        (self.r() as f32 * 0.299 + self.g() as f32 * 0.587 + self.b() as f32 * 0.114) / 255.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_semantic_colors() {
        let colors = SemanticColors::new(ColorScheme::Light, AccentColor::Default);
        assert_eq!(colors.base_color, Color::from_hex(0xFFFFFF));
        assert_eq!(colors.bg_color, Color::from_hex(0xFAFAFA));
        assert_eq!(colors.fg_color, palette::INKSTONE);
        assert_eq!(colors.accent_bg_color, palette::ORANGE);
        assert_ne!(colors.borders_color, colors.bg_color);
    }

    #[test]
    fn dark_base_color_is_lightened_jet() {
        let colors = SemanticColors::new(ColorScheme::Dark, AccentColor::Default);
        assert!(colors.base_color.r() > palette::JET.r());
    }

    #[test]
    fn suggested_is_green_for_default_accent() {
        let colors = SemanticColors::new(ColorScheme::Light, AccentColor::Default);
        assert_eq!(colors.suggested_bg_color, Color::from_hex(SUGGESTED_GREEN_LIGHT));

        let blue = SemanticColors::new(ColorScheme::Light, AccentColor::Blue);
        assert_eq!(blue.suggested_bg_color, blue.accent_bg_color);
    }
}
