//! Color primitives shared across LibGalaxy.
//!
//! The theme crate deliberately does **not** depend on GPUI so that the design
//! tokens can be reused by tooling, config files, or future renderers. GPUI
//! color types are produced via `libgalaxy::theme::to_gpui_color`.
use serde::{Deserialize, Serialize};

/// An sRGB color with an alpha channel.
///
/// Channels are stored as `u8` in `0..=255`, matching the hex notation used
/// throughout the Yaru palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Color {
    /// The packed `0xRRGGBBAA` representation of this color.
    packed: u32,
}

impl Color {
    /// Create a color from its `0xRRGGBBAA` packed representation.
    pub const fn from_packed(packed: u32) -> Self {
        Self { packed }
    }

    /// Create an opaque color from a `0xRRGGBB` hex value.
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            packed: (hex << 8) | 0xFF,
        }
    }

    /// Create a color from explicit channel values.
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            packed: ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | a as u32,
        }
    }

    /// The packed `0xRRGGBBAA` representation.
    pub const fn packed(&self) -> u32 {
        self.packed
    }

    /// The red channel (`0..=255`).
    pub const fn r(&self) -> u8 {
        (self.packed >> 24) as u8
    }

    /// The green channel (`0..=255`).
    pub const fn g(&self) -> u8 {
        (self.packed >> 16) as u8
    }

    /// The blue channel (`0..=255`).
    pub const fn b(&self) -> u8 {
        (self.packed >> 8) as u8
    }

    /// The alpha channel (`0..=255`).
    pub const fn a(&self) -> u8 {
        self.packed as u8
    }

    /// An opaque color with the given `0xRRGGBB` value.
    pub const fn rgb(&self) -> u32 {
        self.packed >> 8
    }

    /// The color formatted as `#RRGGBB` or `#RRGGBBAA`.
    pub fn to_hex_string(&self) -> String {
        if self.a() == 0xFF {
            format!("#{:06X}", self.rgb())
        } else {
            format!("#{:08X}", self.packed)
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::from_rgba(0, 0, 0, 255)
    }
}

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

/// Parse a `#RRGGBB` / `#RRGGBBAA` string into a [`Color`].
impl core::str::FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim_start_matches('#');
        if hex.len() == 6 {
            u32::from_str_radix(hex, 16)
                .map(Color::from_hex)
                .map_err(|e| e.to_string())
        } else if hex.len() == 8 {
            u32::from_str_radix(hex, 16)
                .map(Color::from_packed)
                .map_err(|e| e.to_string())
        } else {
            Err(format!("invalid color `{s}`"))
        }
    }
}

// ---------------------------------------------------------------------------
// SCSS-compatible color manipulation
// ---------------------------------------------------------------------------

/// A color expressed in the HSL color space, matching the model used by SCSS
/// (`darken`/`lighten` operate on this space).
#[derive(Debug, Clone, Copy, PartialEq)]
struct Hsl {
    /// Hue, `0..=360` degrees.
    h: f32,
    /// Saturation, `0..=1`.
    s: f32,
    /// Lightness, `0..=1`.
    l: f32,
    /// Alpha, `0..=1`.
    a: f32,
}

impl Color {
    /// Convert to HSL. Hue is returned in degrees.
    fn to_hsl(&self) -> Hsl {
        let r = self.r() as f32 / 255.0;
        let g = self.g() as f32 / 255.0;
        let b = self.b() as f32 / 255.0;
        let a = self.a() as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let mut h = 0.0;
        if delta > 0.0 {
            if max == r {
                h = 60.0 * (((g - b) / delta).rem_euclid(6.0));
            } else if max == g {
                h = 60.0 * (((b - r) / delta) + 2.0);
            } else {
                h = 60.0 * (((r - g) / delta) + 4.0);
            }
        }

        let l = (max + min) / 2.0;
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        Hsl { h, s, l, a }
    }

    /// Convert from HSL (hue in degrees) back to RGB.
    fn from_hsl(hsl: Hsl) -> Self {
        let Hsl { h, s, l, a } = hsl;
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = match h.rem_euclid(360.0) as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Self::from_rgba(
            ((r + m) * 255.0).round() as u8,
            ((g + m) * 255.0).round() as u8,
            ((b + m) * 255.0).round() as u8,
            (a * 255.0).round() as u8,
        )
    }

    /// Darken the color by `amount` percentage points of lightness (SCSS
    /// semantics: `darken($color, $amount)` where `amount` is `0..=1`).
    pub fn darken(&self, amount: f32) -> Self {
        let mut hsl = self.to_hsl();
        hsl.l = (hsl.l - amount).clamp(0.0, 1.0);
        Self::from_hsl(hsl)
    }

    /// Lighten the color by `amount` percentage points of lightness (SCSS
    /// semantics: `lighten($color, $amount)` where `amount` is `0..=1`).
    pub fn lighten(&self, amount: f32) -> Self {
        let mut hsl = self.to_hsl();
        hsl.l = (hsl.l + amount).clamp(0.0, 1.0);
        Self::from_hsl(hsl)
    }

    /// Blend `self` toward `other` by `t` (`0.0` keeps `self`, `1.0` yields `other`).
    pub fn mix(&self, other: Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::from_rgba(
            (self.r() as f32 + (other.r() as f32 - self.r() as f32) * t).round() as u8,
            (self.g() as f32 + (other.g() as f32 - self.g() as f32) * t).round() as u8,
            (self.b() as f32 + (other.b() as f32 - self.b() as f32) * t).round() as u8,
            (self.a() as f32 + (other.a() as f32 - self.a() as f32) * t).round() as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hex_strings() {
        let c: Color = "#E95420".parse().unwrap();
        assert_eq!(c.r(), 0xE9);
        assert_eq!(c.g(), 0x54);
        assert_eq!(c.b(), 0x20);
        assert_eq!(c.a(), 0xFF);
    }

    #[test]
    fn darken_reduces_lightness() {
        let base = Color::from_hex(0xFAFAFA);
        let darker = base.darken(0.20);
        assert!(darker.r() < base.r());
        assert!(darker.g() < base.g());
    }

    #[test]
    fn lighten_increases_lightness() {
        let base = Color::from_hex(0x181818);
        let lighter = base.lighten(0.06);
        assert!(lighter.r() > base.r());
    }

    #[test]
    fn roundtrips_through_hsl() {
        let original = Color::from_hex(0xE95420);
        let roundtripped = Color::from_hsl(original.to_hsl());
        assert_eq!(original, roundtripped);
    }
}
