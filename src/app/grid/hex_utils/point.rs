use float_eq::derive_float_eq;
use emath::Pos2;
use std::convert::From;

#[derive_float_eq(
    ulps_tol = "PointUlps",
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    debug_ulps_diff = "PointUlpsDebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, Debug, PartialEq",
    all_tol = "f64"
)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl<T> From<[T;2]> for Point where T: Into<f64> + Copy {

    fn from(value: [T;2]) -> Self {
        Self { 
            x: value[0].into(), 
            y: value[1].into(),
        }
    }
}

impl<T, U> From<(T, U)> for Point where 
    T: Into<f64> + Copy,
    U: Into<f64> + Copy,
{

    fn from(value: (T, U)) -> Self {
        Self { 
            x:value.0.into(), 
            y:value.1.into(),
        }
    }
}

impl From<Pos2> for Point {

    fn from(value: Pos2) -> Self {
        Self { 
            x: value.x as f64, 
            y: value.y as f64,
        }
    }
}