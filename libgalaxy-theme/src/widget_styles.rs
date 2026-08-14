//! Per-widget style specifications, mirroring the Yaru widget CSS.
//!
//! Each struct describes the colors and decoration for a widget in every
//! interactive state. The renderer (libgalaxy) resolves these against the
//! active [`crate::Theme`] when building GPUI elements.
use crate::semantic::ColorScheme;
use crate::semantic::SemanticColors;
use serde::{Deserialize, Serialize};

/// A single widget state's appearance.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WidgetStateStyle {
    /// Background color.
    pub background: Option<crate::Color>,
    /// Foreground / text color.
    pub foreground: Option<crate::Color>,
    /// Border color.
    pub border: Option<crate::Color>,
    /// Alpha multiplier (e.g. `0.55` for insensitive widgets).
    pub opacity: Option<f32>,
}

/// The full set of states for a styled control.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WidgetStyle {
    /// The rest state.
    pub normal: WidgetStateStyle,
    /// The hovered state.
    pub hovered: WidgetStateStyle,
    /// The pressed / active state.
    pub pressed: WidgetStateStyle,
    /// The focused state.
    pub focused: WidgetStateStyle,
    /// The disabled / insensitive state.
    pub disabled: WidgetStateStyle,
}

impl WidgetStyle {
    /// Compute a widget style from the semantic colors.
    pub fn new(colors: &SemanticColors, scheme: ColorScheme, kind: WidgetKind) -> Self {
        match kind {
            WidgetKind::Button => Self::button(colors, scheme),
            WidgetKind::SuggestedButton => Self::suggested(colors, scheme),
            WidgetKind::DestructiveButton => Self::destructive(colors, scheme),
            WidgetKind::FlatButton => Self::flat(colors, scheme),
            WidgetKind::Entry => Self::entry(colors, scheme),
            WidgetKind::Switch => Self::switch(colors, scheme),
            WidgetKind::Row => Self::row(colors, scheme),
            WidgetKind::Selection => Self::selection(colors, scheme),
            WidgetKind::Card => Self::card(colors, scheme),
        }
    }

    fn button(colors: &SemanticColors, _scheme: ColorScheme) -> Self {
        let border = colors.borders_color;
        Self {
            normal: WidgetStateStyle {
                background: Some(colors.base_color),
                foreground: Some(colors.fg_color),
                border: Some(border),
                opacity: None,
            },
            hovered: WidgetStateStyle {
                background: Some(colors.base_color.lighten(0.02)),
                foreground: Some(colors.fg_color),
                border: Some(border.darken(0.10)),
                opacity: None,
            },
            pressed: WidgetStateStyle {
                background: Some(border.darken(0.10)),
                foreground: Some(colors.fg_color),
                border: Some(border.darken(0.10)),
                opacity: None,
            },
            focused: WidgetStateStyle {
                background: Some(colors.base_color),
                foreground: Some(colors.fg_color),
                border: Some(colors.accent_bg_color),
                opacity: None,
            },
            disabled: WidgetStateStyle {
                background: Some(colors.insensitive_bg_color),
                foreground: Some(colors.insensitive_fg_color),
                border: Some(colors.borders_color),
                opacity: Some(0.6),
            },
        }
    }

    fn suggested(colors: &SemanticColors, _scheme: ColorScheme) -> Self {
        let bg = colors.suggested_bg_color;
        let fg = colors.selected_fg_color;
        Self {
            normal: WidgetStateStyle {
                background: Some(bg),
                foreground: Some(fg),
                border: Some(bg),
                opacity: None,
            },
            hovered: WidgetStateStyle {
                background: Some(bg.lighten(0.04)),
                foreground: Some(fg),
                border: Some(bg.lighten(0.04)),
                opacity: None,
            },
            pressed: WidgetStateStyle {
                background: Some(bg.darken(0.08)),
                foreground: Some(fg),
                border: Some(bg.darken(0.08)),
                opacity: None,
            },
            focused: WidgetStateStyle {
                background: Some(bg),
                foreground: Some(fg),
                border: Some(colors.accent_bg_color),
                opacity: None,
            },
            disabled: WidgetStateStyle {
                background: Some(colors.insensitive_bg_color),
                foreground: Some(colors.insensitive_fg_color),
                border: Some(colors.borders_color),
                opacity: Some(0.6),
            },
        }
    }

    fn destructive(colors: &SemanticColors, _scheme: ColorScheme) -> Self {
        let bg = colors.destructive_color;
        let fg = colors.selected_fg_color;
        Self {
            normal: WidgetStateStyle {
                background: Some(bg),
                foreground: Some(fg),
                border: Some(bg),
                opacity: None,
            },
            hovered: WidgetStateStyle {
                background: Some(bg.lighten(0.05)),
                foreground: Some(fg),
                border: Some(bg.lighten(0.05)),
                opacity: None,
            },
            pressed: WidgetStateStyle {
                background: Some(bg.darken(0.08)),
                foreground: Some(fg),
                border: Some(bg.darken(0.08)),
                opacity: None,
            },
            focused: WidgetStateStyle {
                background: Some(bg),
                foreground: Some(fg),
                border: Some(colors.accent_bg_color),
                opacity: None,
            },
            disabled: WidgetStateStyle {
                background: Some(colors.insensitive_bg_color),
                foreground: Some(colors.insensitive_fg_color),
                border: Some(colors.borders_color),
                opacity: Some(0.6),
            },
        }
    }

    fn flat(colors: &SemanticColors, _scheme: ColorScheme) -> Self {
        Self {
            normal: WidgetStateStyle {
                background: None,
                foreground: Some(colors.fg_color),
                border: None,
                opacity: None,
            },
            hovered: WidgetStateStyle {
                background: Some(colors.borders_color.with_alpha(0.35)),
                foreground: Some(colors.fg_color),
                border: None,
                opacity: None,
            },
            pressed: WidgetStateStyle {
                background: Some(colors.borders_color),
                foreground: Some(colors.fg_color),
                border: None,
                opacity: None,
            },
            focused: WidgetStateStyle {
                background: None,
                foreground: Some(colors.fg_color),
                border: Some(colors.accent_bg_color),
                opacity: None,
            },
            disabled: WidgetStateStyle {
                background: None,
                foreground: Some(colors.insensitive_fg_color),
                border: None,
                opacity: Some(0.6),
            },
        }
    }

    fn entry(colors: &SemanticColors, _scheme: ColorScheme) -> Self {
        let border = colors.borders_color;
        Self {
            normal: WidgetStateStyle {
                background: Some(colors.base_color),
                foreground: Some(colors.text_color),
                border: Some(border),
                opacity: None,
            },
            hovered: WidgetStateStyle {
                background: Some(colors.base_color),
                foreground: Some(colors.text_color),
                border: Some(border.darken(0.15)),
                opacity: None,
            },
            pressed: WidgetStateStyle {
                background: Some(colors.base_color),
                foreground: Some(colors.text_color),
                border: Some(colors.accent_bg_color),
                opacity: None,
            },
            focused: WidgetStateStyle {
                background: Some(colors.base_color),
                foreground: Some(colors.text_color),
                border: Some(colors.accent_bg_color),
                opacity: None,
            },
            disabled: WidgetStateStyle {
                background: Some(colors.insensitive_bg_color),
                foreground: Some(colors.insensitive_fg_color),
                border: Some(colors.borders_color),
                opacity: Some(0.6),
            },
        }
    }

    fn switch(colors: &SemanticColors, _scheme: ColorScheme) -> Self {
        let border = colors.borders_color;
        Self {
            normal: WidgetStateStyle {
                background: Some(colors.base_color),
                foreground: Some(colors.fg_color),
                border: Some(border),
                opacity: None,
            },
            hovered: WidgetStateStyle {
                background: Some(colors.base_color.lighten(0.02)),
                foreground: Some(colors.fg_color),
                border: Some(border.darken(0.10)),
                opacity: None,
            },
            pressed: WidgetStateStyle {
                background: Some(border.darken(0.10)),
                foreground: Some(colors.fg_color),
                border: Some(border.darken(0.10)),
                opacity: None,
            },
            focused: WidgetStateStyle {
                background: Some(colors.base_color),
                foreground: Some(colors.fg_color),
                border: Some(colors.accent_bg_color),
                opacity: None,
            },
            disabled: WidgetStateStyle {
                background: Some(colors.insensitive_bg_color),
                foreground: Some(colors.insensitive_fg_color),
                border: Some(colors.borders_color),
                opacity: Some(0.6),
            },
        }
    }

    fn row(colors: &SemanticColors, _scheme: ColorScheme) -> Self {
        Self {
            normal: WidgetStateStyle {
                background: None,
                foreground: Some(colors.fg_color),
                border: None,
                opacity: None,
            },
            hovered: WidgetStateStyle {
                background: Some(colors.bg_color.darken(0.03)),
                foreground: Some(colors.fg_color),
                border: None,
                opacity: None,
            },
            pressed: WidgetStateStyle {
                background: Some(colors.borders_color.with_alpha(0.6)),
                foreground: Some(colors.fg_color),
                border: None,
                opacity: None,
            },
            focused: WidgetStateStyle {
                background: Some(colors.accent_bg_color.with_alpha(0.15)),
                foreground: Some(colors.fg_color),
                border: None,
                opacity: None,
            },
            disabled: WidgetStateStyle {
                background: None,
                foreground: Some(colors.insensitive_fg_color),
                border: None,
                opacity: Some(0.6),
            },
        }
    }

    fn selection(colors: &SemanticColors, _scheme: ColorScheme) -> Self {
        Self {
            normal: WidgetStateStyle {
                background: Some(colors.selected_bg_color),
                foreground: Some(colors.selected_fg_color),
                border: None,
                opacity: None,
            },
            hovered: WidgetStateStyle {
                background: Some(colors.selected_bg_color.lighten(0.03)),
                foreground: Some(colors.selected_fg_color),
                border: None,
                opacity: None,
            },
            pressed: WidgetStateStyle {
                background: Some(colors.selected_bg_color.darken(0.05)),
                foreground: Some(colors.selected_fg_color),
                border: None,
                opacity: None,
            },
            focused: WidgetStateStyle {
                background: Some(colors.selected_bg_color),
                foreground: Some(colors.selected_fg_color),
                border: None,
                opacity: None,
            },
            disabled: WidgetStateStyle {
                background: Some(colors.insensitive_bg_color),
                foreground: Some(colors.insensitive_fg_color),
                border: None,
                opacity: Some(0.6),
            },
        }
    }

    fn card(colors: &SemanticColors, _scheme: ColorScheme) -> Self {
        Self {
            normal: WidgetStateStyle {
                background: Some(colors.card_bg_color),
                foreground: Some(colors.fg_color),
                border: Some(colors.borders_color),
                opacity: None,
            },
            hovered: WidgetStateStyle {
                background: Some(colors.card_bg_color),
                foreground: Some(colors.fg_color),
                border: Some(colors.borders_strong_color),
                opacity: None,
            },
            pressed: WidgetStateStyle {
                background: Some(colors.borders_color.with_alpha(0.4)),
                foreground: Some(colors.fg_color),
                border: Some(colors.borders_strong_color),
                opacity: None,
            },
            focused: WidgetStateStyle {
                background: Some(colors.card_bg_color),
                foreground: Some(colors.fg_color),
                border: Some(colors.accent_bg_color),
                opacity: None,
            },
            disabled: WidgetStateStyle {
                background: Some(colors.card_bg_color),
                foreground: Some(colors.insensitive_fg_color),
                border: Some(colors.borders_color),
                opacity: Some(0.6),
            },
        }
    }
}

/// The kinds of widgets for which a [`WidgetStyle`] can be resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WidgetKind {
    /// A standard button.
    Button,
    /// A `suggested-action` button.
    SuggestedButton,
    /// A `destructive-action` button.
    DestructiveButton,
    /// A `flat` button.
    FlatButton,
    /// A text entry.
    Entry,
    /// A switch.
    Switch,
    /// A list row.
    Row,
    /// A selected list row / navigation item.
    Selection,
    /// A card (e.g. `AdwPreferencesGroup` background).
    Card,
}

// ---------------------------------------------------------------------------
// Color helpers used by the widget styles
// ---------------------------------------------------------------------------

impl crate::Color {
    /// Return this color with the given alpha (`0..=1`).
    pub fn with_alpha(&self, alpha: f32) -> Self {
        crate::Color::from_rgba(
            self.r(),
            self.g(),
            self.b(),
            (alpha.clamp(0.0, 1.0) * 255.0).round() as u8,
        )
    }
}
