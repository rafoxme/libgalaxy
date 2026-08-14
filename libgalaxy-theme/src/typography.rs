//! The Yaru / libadwaita typography scale.
//!
//! GTK4 expresses type sizes as percentages of a base size; the SCSS tokens
//! from the spec are given in `pt`. GPUI works in pixels, so sizes are
//! converted with `1pt = 4/3 px` (96 DPI, `96 / 72`).
use serde::{Deserialize, Serialize};

/// Dots-per-inch / points-per-pixel conversion factor (`96 / 72`).
pub const POINTS_PER_PIXEL: f32 = 4.0 / 3.0;

/// The base font size of the theme (11pt, GTK's default `$font-size`).
pub const BASE_FONT_SIZE_PT: f32 = 11.0;

/// The canonical body font family used by Yaru.
pub const FONT_FAMILY: &str = "Ubuntu";

/// A font weight, matching the CSS numeric scale (`400` = normal).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FontWeight(pub f32);

impl FontWeight {
    /// Thin weight (100).
    pub const THIN: Self = Self(100.0);
    /// Extra-light weight (200).
    pub const EXTRA_LIGHT: Self = Self(200.0);
    /// Light weight (300).
    pub const LIGHT: Self = Self(300.0);
    /// Normal weight (400).
    pub const NORMAL: Self = Self(400.0);
    /// Medium weight (500).
    pub const MEDIUM: Self = Self(500.0);
    /// Semi-bold weight (600).
    pub const SEMIBOLD: Self = Self(600.0);
    /// Bold weight (700).
    pub const BOLD: Self = Self(700.0);
    /// Extra-bold weight (800).
    pub const EXTRA_BOLD: Self = Self(800.0);
    /// Black weight (900).
    pub const BLACK: Self = Self(900.0);
}

/// The named text styles from the Yaru / libadwaita type scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextScale {
    /// `font-weight: 300; font-size: 24pt;`
    LargeTitle,
    /// `font-weight: 800; font-size: 20pt;`
    Title1,
    /// `font-weight: 800; font-size: 15pt;`
    Title2,
    /// `font-weight: 700; font-size: 15pt;`
    Title3,
    /// `font-weight: 700; font-size: 13pt;`
    Title4,
    /// `font-weight: 700; font-size: 11pt;`
    Heading,
    /// `font-weight: 400; font-size: 11pt;`
    Body,
    /// `font-weight: 700; font-size: 9pt;`
    CaptionHeading,
    /// `font-weight: 400; font-size: 9pt;`
    Caption,
}

impl TextScale {
    /// The font size of this scale in points.
    pub const fn size_pt(self) -> f32 {
        match self {
            Self::LargeTitle => 24.0,
            Self::Title1 => 20.0,
            Self::Title2 => 15.0,
            Self::Title3 => 15.0,
            Self::Title4 => 13.0,
            Self::Heading => 11.0,
            Self::Body => 11.0,
            Self::CaptionHeading => 9.0,
            Self::Caption => 9.0,
        }
    }

    /// The font size of this scale in pixels (`pt * 96 / 72`).
    pub const fn size_px(self) -> f32 {
        self.size_pt() * POINTS_PER_PIXEL as f32
    }

    /// The font weight of this scale.
    pub const fn weight(self) -> FontWeight {
        match self {
            Self::LargeTitle => FontWeight::LIGHT,
            Self::Title1 | Self::Title2 => FontWeight::EXTRA_BOLD,
            Self::Title3 | Self::Title4 | Self::Heading | Self::CaptionHeading => {
                FontWeight::BOLD
            }
            Self::Body | Self::Caption => FontWeight::NORMAL,
        }
    }
}

/// The complete typography specification for a theme.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Typography {
    /// The primary font family.
    pub font_family: String,
    /// The size of each named text scale, in pixels.
    pub scales: std::collections::HashMap<TextScale, f32>,
}

impl Typography {
    /// The default Yaru typography: `Ubuntu` at the GTK type scale.
    pub fn yaru() -> Self {
        let scales = TextScale::ALL
            .iter()
            .map(|&scale| (scale, scale.size_px()))
            .collect();
        Self {
            font_family: FONT_FAMILY.to_string(),
            scales,
        }
    }

    /// The size in pixels for a given scale.
    pub fn size(&self, scale: TextScale) -> f32 {
        self.scales.get(&scale).copied().unwrap_or_else(|| scale.size_px())
    }
}

impl Default for Typography {
    fn default() -> Self {
        Self::yaru()
    }
}

impl TextScale {
    /// All text scales.
    pub const ALL: [TextScale; 9] = [
        Self::LargeTitle,
        Self::Title1,
        Self::Title2,
        Self::Title3,
        Self::Title4,
        Self::Heading,
        Self::Body,
        Self::CaptionHeading,
        Self::Caption,
    ];
}
