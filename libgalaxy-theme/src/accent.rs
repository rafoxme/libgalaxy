//! The Yaru accent color variants.
//!
//! Mirror of `AdwAccentColor` from libadwaita. Selecting an accent color
//! changes the `accent_bg_color` semantic token used by suggested/active
//! widgets. The default variant maps to Yaru orange (`#E95420`).
use crate::color::Color;
use serde::{Deserialize, Serialize};

/// The selectable accent color variants (14 total, matching the libadwaita
/// widget gallery's accent picker).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccentColor {
    /// Ubuntu orange, the Yaru default.
    Default,
    /// Bark brown.
    Bark,
    /// Sage green.
    Sage,
    /// Olive yellow-green.
    Olive,
    /// Viridian teal.
    Viridian,
    /// Prussian green.
    PrussianGreen,
    /// Blue.
    Blue,
    /// Purple.
    Purple,
    /// Magenta.
    Magenta,
    /// Red.
    Red,
    /// Yellow.
    Yellow,
    /// Warty brown (first Ubuntu release).
    WartyBrown,
}

impl AccentColor {
    /// The RGB value of this accent variant.
    pub const fn color(self) -> Color {
        match self {
            Self::Default => Color::from_hex(0xE95420),
            Self::Bark => Color::from_hex(0xA57650),
            Self::Sage => Color::from_hex(0x9A9F61),
            Self::Olive => Color::from_hex(0xB4A545),
            Self::Viridian => Color::from_hex(0x3C948B),
            Self::PrussianGreen => Color::from_hex(0x1660A7),
            Self::Blue => Color::from_hex(0x3584E4),
            Self::Purple => Color::from_hex(0x9141AC),
            Self::Magenta => Color::from_hex(0xC061CB),
            Self::Red => Color::from_hex(0xE01B24),
            Self::Yellow => Color::from_hex(0xF5C211),
            Self::WartyBrown => Color::from_hex(0x98512B),
        }
    }

    /// Whether this is the default (orange) accent.
    pub const fn is_default(self) -> bool {
        matches!(self, Self::Default)
    }

    /// A human readable label for the accent picker.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Bark => "Bark",
            Self::Sage => "Sage",
            Self::Olive => "Olive",
            Self::Viridian => "Viridian",
            Self::PrussianGreen => "Prussian Green",
            Self::Blue => "Blue",
            Self::Purple => "Purple",
            Self::Magenta => "Magenta",
            Self::Red => "Red",
            Self::Yellow => "Yellow",
            Self::WartyBrown => "Warty Brown",
        }
    }

    /// All accent variants, in display order.
    pub const ALL: [Self; 12] = [
        Self::Default,
        Self::Bark,
        Self::Sage,
        Self::Olive,
        Self::Viridian,
        Self::PrussianGreen,
        Self::Blue,
        Self::Purple,
        Self::Magenta,
        Self::Red,
        Self::Yellow,
        Self::WartyBrown,
    ];
}

impl Default for AccentColor {
    fn default() -> Self {
        Self::Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_orange() {
        assert_eq!(AccentColor::default().color(), crate::palette::ORANGE);
    }
}
