pub const RATED_MW: f64 = 4.2;
pub const ROTOR_DIAMETER_M: f64 = 136.0;
pub const AIR_DENSITY: f64 = 1.225;
pub const CP: f64 = 0.45;
pub const CUT_IN: f64 = 3.0;
pub const CUT_OUT: f64 = 25.0;
use std::f64::consts::PI;

/// Swept rotor area in m²
pub fn rotor_area() -> f64 {
    PI * ROTOR_DIAMETER_M.powi(2) / 4.0
}

/// Electrical power in MW for wind speed `v` in m/s.
/// 0 below cut-in or at/above cut-out, P = ½·ρ·A·Cp·v³ in between, capped at RATED_MW.
pub fn power_curve(v: f64) -> f64 {
    //if CUT_OUT <= v  || v < CUT_IN {
    if !(CUT_IN..CUT_OUT).contains(&v) {
        0.0
    } else {
        RATED_MW.min(0.5 * AIR_DENSITY *  rotor_area() * CP * v.powi(3) / 1e6 )  // mega watt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_below_cut_in() {
        assert_eq!(power_curve(2.0), 0.0);
    }

    #[test]
    fn zero_at_cut_out() {
        assert_eq!(power_curve(25.0), 0.0);
    }

    #[test]
    fn rated_at_high_wind() {
        assert_eq!(power_curve(15.0), RATED_MW);
    }

    #[test]
    fn about_2_mw_at_8_ms() {
        let p = power_curve(8.0);
        assert!((p - 2.05).abs() < 0.01, "got {p}");
    }
}
