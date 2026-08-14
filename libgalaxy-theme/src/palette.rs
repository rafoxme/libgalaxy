//! The Yaru base palette, extracted from
//! `yaru/gtk/src/default/gtk-3.0/_palette.scss`.
//!
//! These constants are the raw named colors of the palette; semantic colors
//! (see [`crate::semantic`]) are derived from them per color scheme.
use crate::color::Color;

pub mod blacks {
    use super::*;
    pub const JET: Color = Color::from_hex(0x181818);
    pub const INKSTONE: Color = Color::from_hex(0x3D3D3D);
    pub const SLATE: Color = Color::from_hex(0x5D5D5D);
    pub const GRAPHITE: Color = Color::from_hex(0x666666);
}

pub mod whites {
    use super::*;
    pub const WHITE: Color = Color::from_hex(0xFFFFFF);
    pub const PORCELAIN: Color = Color::from_hex(0xF7F7F7);
    pub const SILK: Color = Color::from_hex(0xCCCCCC);
    pub const WARM_GRAY: Color = Color::from_hex(0xAEA79F);
    pub const ASH: Color = Color::from_hex(0x878787);
}

pub mod purples {
    use super::*;
    pub const AUBERGINE: Color = Color::from_hex(0x924D8B);
    pub const PURPLE: Color = Color::from_hex(0x762572);
    pub const LIGHT_AUBERGINE: Color = Color::from_hex(0x77216F);
    pub const MID_AUBERGINE: Color = Color::from_hex(0x5E2750);
    pub const DARK_AUBERGINE: Color = Color::from_hex(0x2C001E);
}

pub mod reds {
    use super::*;
    pub const RED: Color = Color::from_hex(0xC7162B);
}

pub mod oranges {
    use super::*;
    /// The default accent color.
    pub const ORANGE: Color = Color::from_hex(0xE95420);
}

pub mod yellows {
    use super::*;
    pub const YELLOW: Color = Color::from_hex(0xF99B11);
}

pub mod greens {
    use super::*;
    pub const GREEN: Color = Color::from_hex(0x0E8420);
}

pub mod blues {
    use super::*;
    pub const BLUE: Color = Color::from_hex(0x19B6EE);
    pub const LINK_BLUE: Color = Color::from_hex(0x007AA6);
    pub const DARK_BLUE: Color = Color::from_hex(0x335280);
}

// ---------------------------------------------------------------------------
// Convenience re-exports
// ---------------------------------------------------------------------------

pub use blacks::{GRAPHITE, INKSTONE, JET, SLATE};
pub use blues::{BLUE, DARK_BLUE, LINK_BLUE};
pub use greens::GREEN;
pub use oranges::ORANGE;
pub use purples::{
    AUBERGINE, DARK_AUBERGINE, LIGHT_AUBERGINE, MID_AUBERGINE, PURPLE,
};
pub use reds::RED;
pub use whites::{ASH, PORCELAIN, SILK, WARM_GRAY, WHITE};
pub use yellows::YELLOW;
