//! **LibGalaxy Theme** — the Ubuntu Yaru design tokens for LibGalaxy.
//!
//! This crate is dependency-free (aside from `serde`), so the tokens can be
//! consumed by tooling, config files, or any renderer. GPUI color conversion
//! happens in the `libgalaxy` crate via `libgalaxy::theme::to_gpui_color`.
//!
//! # Layout
//!
//! - [`palette`] — the raw named colors from `_palette.scss`.
//! - [`semantic`] — semantic colors resolved per light/dark scheme.
//! - [`accent`] — the 14 selectable accent variants.
//! - [`spacing`] — padding, margin, radius and size tokens.
//! - [`typography`] — the GTK text scale.
//! - [`widget_styles`] — per-widget state styles.
//! - [`theme::Theme`] — the bundle tying it all together.

pub mod accent;
pub mod color;
pub mod palette;
pub mod semantic;
pub mod spacing;
pub mod theme;
pub mod typography;
pub mod widget_styles;

pub use accent::AccentColor;
pub use color::Color;
pub use semantic::{ColorScheme, SemanticColors};
pub use theme::Theme;
pub use typography::{FontWeight, TextScale, Typography};
pub use widget_styles::{WidgetKind, WidgetStateStyle, WidgetStyle};
