//channels/rgb.rs
use crate::family::rgb::*;
use crate::consts::*;

#[repr(C)]
pub struct RGBchannels<T> {
    /// The red color component.
    pub r: T,
    /// The green color component.
    pub g: T,
    /// The blue color component.
    pub b: T,
}

impl RGBchannels<u8> {
    /// Converts sRGB color values into [`LINEAR`] color space.
    ///
    /// This applies the standard sRGB gamma correction and returns
    /// linear-light RGB values.
    pub fn to_linear(&self) -> RGBchannels<f32> {
        let linear_channel = |channel_u8: u8| -> f32 {
            let channel = channel_u8 as f32 / 255.0;
            
            if channel <= 0.04045 { channel / 12.92 }
            else { ((channel + 0.055) / 1.055).powf(2.4) }
        };
        
        RGBchannels { 
            r: linear_channel(self.r),
            g: linear_channel(self.g),
            b: linear_channel(self.b),
        }
    }
}

impl RGBchannels<f32> {
    /// Converts Linear RGB color values into [`SRGB`] color space.
    ///
    /// ### Notes
    /// - Input values are assumed to be **linear RGB** in the range `[0.0, 1.0]`.
    /// - Values outside this range are not explicitly clamped before gamma encoding,
    ///   but the final result is clamped to `[0, 255]`.
    /// - The sRGB transfer function (gamma correction) is applied per channel.
    pub fn to_srgb(&self) -> RGBchannels<u8> {
        let srgb_channel = |channel: f32| -> u8 {
            let v = if channel <= 0.0031308 {
                channel * 12.92
            } else {
                1.055 * channel.powf(1.0 / 2.4) - 0.055
            };

            (v.clamp(0.0, 1.0) * 255.0).round() as u8
        };

        RGBchannels { 
            r: srgb_channel(self.r),
            g: srgb_channel(self.g),
            b: srgb_channel(self.b),
        }
    }
    /// Converts Linear RGB [`LINEAR`] to Display P3 [`DP3`] color space.
    pub fn to_linear_dp3(&self) -> RGBchannels<f32> {
        let r = LRGB_DP3[0][0] * self.r + LRGB_DP3[0][1] * self.g + LRGB_DP3[0][2] * self.b;
        let g = LRGB_DP3[1][0] * self.r + LRGB_DP3[1][1] * self.g + LRGB_DP3[1][2] * self.b;
        let b = LRGB_DP3[2][0] * self.r + LRGB_DP3[2][1] * self.g + LRGB_DP3[2][2] * self.b;

        RGBchannels { r, g, b }
    }
    /// Converts Display P3 [`DP3`] to Linear RGB [`LINEAR`] color space.
    pub fn from_dp3(&self) -> RGBchannels<f32> {
        let r = DP3_LRGB[0][0] * self.r - DP3_LRGB[0][1] * self.g + DP3_LRGB[0][2] * self.b;
        let g = DP3_LRGB[1][0] * self.r + DP3_LRGB[1][1] * self.g + DP3_LRGB[1][2] * self.b;
        let b = DP3_LRGB[2][0] * self.r - DP3_LRGB[2][1] * self.g + DP3_LRGB[2][2] * self.b;

        RGBchannels { r, g, b }
    }
}

impl<T> From<(T, T, T)> for RGBchannels<T> {
    fn from(tuple: (T, T, T)) -> Self {
        RGBchannels { 
            r: tuple.0,
            g: tuple.1,
            b: tuple.2,
        }
    }
}