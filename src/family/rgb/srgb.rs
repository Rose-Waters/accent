#![allow(unused)]
// srgb.rs
use crate::channels::rgb::RGBchannels;
use crate::model::RGBmodel;
use crate::color::Color;
/// # sRGB
pub struct SRGB;

impl Color<RGBmodel, SRGB, u8> {
    ///[`8-bit`]
    ///  ## sRGB
    /// returns a opaque color from RGB channels.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(RGBchannels { r, g, b }, 255)
    }
    ///[`8-bit`]
    ///  ## sRGB
    /// returns a color with explicit alpha.
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(RGBchannels { r, g, b }, a)
    }
}