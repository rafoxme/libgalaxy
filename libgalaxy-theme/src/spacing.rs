//! Spacing, sizing and radius tokens, extracted from the Yaru theme.
//!
//! All values are in device-independent pixels, matching GPUI's `Pixels`.
//! These mirror the SCSS tokens:
//! `$base_padding`, `$base_margin`, `$base_border_radius`, etc.

/// The base padding applied to most controls (`$base_padding: 6px`).
pub const BASE_PADDING: f32 = 6.0;

/// The base margin (`$base_margin: 4px`).
pub const BASE_MARGIN: f32 = 4.0;

/// The default border radius for controls (`$base_border_radius: 6px`).
pub const BASE_BORDER_RADIUS: f32 = 6.0;

/// Radius used to force a fully circular shape (`$forced_circular_radius: 999px`).
pub const CIRCULAR_RADIUS: f32 = 999.0;

/// Radius for modal dialogs (`$modal_radius: 6px`).
pub const MODAL_RADIUS: f32 = 6.0;

/// Radius for alert boxes (`$alert_radius: 8px`).
pub const ALERT_RADIUS: f32 = 8.0;

/// Radius for menus (`$yaru_menu_border_radius: $modal_radius * 2.25`).
pub const MENU_BORDER_RADIUS: f32 = MODAL_RADIUS * 2.25;

// ---------------------------------------------------------------------------
// Icon sizes
// ---------------------------------------------------------------------------

/// The base icon size (`$base_icon_size: 16px`).
pub const BASE_ICON_SIZE: f32 = 16.0;

/// The medium icon size (`$medium_icon_size: 24px`).
pub const MEDIUM_ICON_SIZE: f32 = 24.0;

/// The large icon size (`$large_icon_size: 32px`).
pub const LARGE_ICON_SIZE: f32 = 32.0;

// ---------------------------------------------------------------------------
// Derived padding scale (GTK spacing step)
// ---------------------------------------------------------------------------

/// The GTK spacing scale (`$spacing` halving/halving around `$base_padding`).
pub mod spacing {
    /// `$spacing * 0.5`.
    pub const SPACING_2XS: f32 = 2.0;
    /// `$spacing * 0.75`.
    pub const SPACING_XS: f32 = 3.0;
    /// `$spacing` (base).
    pub const SPACING_SM: f32 = super::BASE_PADDING * 0.75;
    /// `$spacing * 1.5`.
    pub const SPACING_MD: f32 = super::BASE_PADDING * 1.5;
    /// `$spacing * 2`.
    pub const SPACING_LG: f32 = super::BASE_PADDING * 2.0;
    /// `$spacing * 3`.
    pub const SPACING_XL: f32 = super::BASE_PADDING * 3.0;
}

// ---------------------------------------------------------------------------
// Sizes
// ---------------------------------------------------------------------------

/// The height of a standard control (`$menuitem_height`: 32px).
pub const CONTROL_HEIGHT: f32 = 32.0;

/// The height of a large control (`$menuitem_height + $base_padding`).
pub const CONTROL_HEIGHT_LARGE: f32 = 40.0;

/// The width of a standard switch (`$switch_width: 42px`).
pub const SWITCH_WIDTH: f32 = 42.0;

/// The height of a standard switch (`$switch_height: 24px`).
pub const SWITCH_HEIGHT: f32 = 24.0;

/// The width of a switch slider knob.
pub const SWITCH_SLIDER_WIDTH: f32 = 18.0;

/// The default avatar size (48px, matching `AdwAvatar`).
pub const AVATAR_SIZE: f32 = 48.0;

/// The default spinner size (16px).
pub const SPINNER_SIZE: f32 = 16.0;
