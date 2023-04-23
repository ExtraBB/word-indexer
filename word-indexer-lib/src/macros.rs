#[macro_export]
macro_rules! assert_eq_f64 {
    ($left:expr, $right:expr) => {{
        assert!(f64::abs($left - $right) < 0.0000001);
    }};
}
