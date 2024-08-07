// https://solhsa.com/interpolation/

/// Smooths out t in \[0,1].
///
/// It takes a while before it rises and slows down before reaching the end.
pub fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// An even smoother [smooth_step], but more expensive to calculate.
#[allow(dead_code)]
pub fn smoother_step(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Calculates the weighted average.
#[allow(dead_code)]
pub fn weighted_average(current: f32, target: f32, slowdown_factor: f32) -> f32 {
    ((current * (slowdown_factor - 1.0)) + target) / slowdown_factor
}
