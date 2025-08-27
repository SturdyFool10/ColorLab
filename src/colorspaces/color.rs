use serde::{Deserialize, Serialize};

/// The main color struct, supporting HDR and multiple color spaces.
/// Internally stores color as linear RGBA with f32 components.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Color {
    /// Red channel (linear, 0.0+ for HDR)
    pub r: f64,
    /// Green channel (linear, 0.0+ for HDR)
    pub g: f64,
    /// Blue channel (linear, 0.0+ for HDR)
    pub b: f64,
    /// Alpha channel (0.0 = transparent, 1.0 = opaque)
    pub a: f64,
}

impl Color {
    /// Construct a new color from linear RGBA components.
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// Construct an opaque color.
    pub fn opaque(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b, a: 1.0 }
    }
}
