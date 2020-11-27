use rust_ray::math::*;

#[test]
fn clamp_indentity() {
    assert_eq!(0.0.clamp(0.0, 1.0), 0.0);
    assert_eq!(0.5.clamp(0.0, 1.0), 0.5);
    assert_eq!(1.0.clamp(0.0, 1.0), 1.0);
}

#[test]
fn clamp_minmax() {
    assert_eq!((-10.0).clamp(0.0, 1.0), 0.0);
    assert_eq!(100.0.clamp(0.0, 10.0), 10.0);
    assert_eq!(100.0.clamp(0.0, 10.0), 10.0);
}

#[test]
fn lerm_interpolate() {
    assert_eq!(0.0.lerp(10.0, 20.0), 10.0);
    assert_eq!(0.5.lerp(10.0, 20.0), 15.0);
    assert_eq!(1.0.lerp(10.0, 20.0), 20.0);
}

#[test]
fn lerm_minmax() {
    assert_eq!((-1.0).lerp(10.0, 20.0), 10.0);
    assert_eq!(2.0.lerp(10.0, 20.0), 20.0);
}

#[test]
fn lerp_unclamped_interpolate() {
    assert_eq!(0.0.lerp_unclamped(10.0, 20.0), 10.0);
    assert_eq!(0.5.lerp_unclamped(10.0, 20.0), 15.0);
    assert_eq!(1.0.lerp_unclamped(10.0, 20.0), 20.0);
}

#[test]
fn lerp_unclamped_minmax() {
    assert_eq!((-1.0).lerp_unclamped(10.0, 20.0), 0.0);
    assert_eq!(2.0.lerp_unclamped(10.0, 20.0), 30.0);
}

#[test]
fn inv_lerp_calc() {
    assert_eq!(10.0.inv_lerp(10.0, 20.0), 0.0);
    assert_eq!(20.0.inv_lerp(10.0, 20.0), 1.0);
    assert_eq!(15.0.inv_lerp(10.0, 20.0), 0.5);
    assert_eq!(30.0.inv_lerp(10.0, 20.0), 2.0);
    assert_eq!(0.0.inv_lerp(10.0, 20.0), -1.0);
}

#[test]
fn inv_lerp_eq_ab() {
    assert_eq!(10.0.inv_lerp(10.0, 10.0), 0.0);
    assert_eq!(20.0.inv_lerp(10.0, 10.0), 0.0);
}
