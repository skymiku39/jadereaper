//! Float functions that give the same bits on every platform.
//!
//! The standard `sin`, `cos`, `exp`, `powf` and `atan2` come from a different
//! maths library on each target, and the native and wasm ones disagree in the
//! last bits. That is enough to make the same seed and inputs play out as two
//! different runs. Everything here is built only from `+ - * /`, `floor` and
//! comparisons, which are exact everywhere. The simulation must use these
//! instead of the standard ones; `npm run determinism` catches a slip.

use core::f32::consts::{FRAC_PI_2, LN_2, PI, TAU};

/// Sine, accurate to about 1e-6 for small angles. Large angles lose precision
/// the same way on every platform.
pub fn sin(x: f32) -> f32 {
    // Reduce to -0.5..0.5 turns, then fold into the quarter turn around zero.
    let t = x * (1.0 / TAU);
    let t = t - (t + 0.5).floor();
    let t = if t > 0.25 {
        0.5 - t
    } else if t < -0.25 {
        -0.5 - t
    } else {
        t
    };
    let a = t * TAU;
    let s = a * a;
    a * (1.0
        + s * (-1.0 / 6.0
            + s * (1.0 / 120.0 + s * (-1.0 / 5040.0 + s * (1.0 / 362_880.0 + s * (-1.0 / 39_916_800.0))))))
}

pub fn cos(x: f32) -> f32 {
    sin(x + FRAC_PI_2)
}

/// e to the power `x`, for `x` in -80..80.
pub fn exp(x: f32) -> f32 {
    let x = x.clamp(-80.0, 80.0);
    // x = k ln 2 + r with r in about -0.35..0.35, so e^x = 2^k e^r.
    let k = (x * (1.0 / LN_2) + 0.5).floor();
    let r = x - k * LN_2;
    let p = 1.0
        + r * (1.0
            + r * (1.0 / 2.0)
                * (1.0
                    + r * (1.0 / 3.0) * (1.0 + r * (1.0 / 4.0) * (1.0 + r * (1.0 / 5.0) * (1.0 + r * (1.0 / 6.0))))));
    p * f32::from_bits(((k as i32 + 127) as u32) << 23)
}

/// `base` to a whole power.
pub fn powu(base: f32, power: u32) -> f32 {
    let mut out = 1.0;
    for _ in 0..power {
        out *= base;
    }
    out
}

/// Angle of the vector (x, y) in -pi..pi, accurate to about 1e-5. Zero for
/// the zero vector.
pub fn atan2(y: f32, x: f32) -> f32 {
    let (ax, ay) = (x.abs(), y.abs());
    let hi = ax.max(ay);
    if hi == 0.0 {
        return 0.0;
    }
    let t = ax.min(ay) / hi;
    let s = t * t;
    let mut a = t
        * (0.999_977_26
            + s * (-0.332_623_47 + s * (0.193_543_46 + s * (-0.116_432_87 + s * (0.052_653_32 + s * -0.011_721_2)))));
    if ay > ax {
        a = FRAC_PI_2 - a;
    }
    if x < 0.0 {
        a = PI - a;
    }
    if y < 0.0 {
        -a
    } else {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sin_and_cos_match_the_standard_ones() {
        for i in -2000..2000 {
            let x = i as f32 * 0.01;
            assert!((sin(x) - x.sin()).abs() < 2e-5, "sin({x}) = {} vs {}", sin(x), x.sin());
            assert!((cos(x) - x.cos()).abs() < 2e-5, "cos({x}) = {} vs {}", cos(x), x.cos());
        }
    }

    #[test]
    fn exp_matches_the_standard_one() {
        for i in -1000..=400 {
            let x = i as f32 * 0.01;
            let (got, want) = (exp(x), x.exp());
            assert!((got - want).abs() <= want * 1e-5, "exp({x}) = {got} vs {want}");
        }
    }

    #[test]
    fn powu_matches_the_standard_one() {
        for n in 0..8 {
            assert!((powu(0.92, n) - 0.92f32.powi(n as i32)).abs() < 1e-6);
        }
    }

    #[test]
    fn atan2_matches_the_standard_one() {
        for i in -40..=40 {
            for j in -40..=40 {
                let (x, y) = (i as f32 * 0.37, j as f32 * 0.41);
                if x == 0.0 && y == 0.0 {
                    continue;
                }
                let (got, want) = (atan2(y, x), y.atan2(x));
                // -pi and pi are the same angle.
                let diff = (got - want).abs().min(TAU - (got - want).abs());
                assert!(diff < 1e-4, "atan2({y}, {x}) = {got} vs {want}");
            }
        }
        assert_eq!(atan2(0.0, 0.0), 0.0);
    }
}
