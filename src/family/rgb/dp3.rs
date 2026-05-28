#![allow(unused)]
// dp3.rs
/// # Display P3
pub struct DP3;

use crate::family::rgb::LINEAR;

/// ## Display P3
/// Linear RGB [`LINEAR`] -> Display P3 [`DP3`] Matrix
pub const LRGB_DP3: [[f32; 3]; 3] = [
    [0.8224621, 0.1775380, 0.0000000],
    [0.0331941, 0.9668058, 0.0000000],
    [0.0170827, 0.0723974, 0.9105199],
];

/// ## Display P3
/// Display P3 [`DP3`] -> Linear RGB [`LINEAR`] Matrix
pub const DP3_LRGB: [[f32; 3]; 3] = [
    [ 1.2249401, -0.2249404, 0.0000000],
    [-0.0420569,  1.0420571, 0.0000000],
    [-0.0196376, -0.0786361, 1.0978731],
];