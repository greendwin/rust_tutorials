use rust_ray::math::Vec3;

#[test]
fn vec3_create() {
    let v = Vec3::new(1, 2, 3);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn vec3_print() {
    let v = Vec3::new(1, 2, 3);
    assert_eq!("(1, 2, 3)", format!("{}", v));
}

#[test]
fn vec3_neg() {
    let v = -Vec3::new(1, 2, 3);
    assert_eq!(v.x, -1.0);
    assert_eq!(v.y, -2.0);
    assert_eq!(v.z, -3.0);
}

#[test]
fn vec3_add() {
    let v = Vec3::new(1, 2, 3) + Vec3::new(4, 5, 6);
    assert_eq!(v.x, 5.0);
    assert_eq!(v.y, 7.0);
    assert_eq!(v.z, 9.0);
}

#[test]
fn vec3_add_scalar() {
    let v = Vec3::new(1, 2, 3) + 100.0;
    assert_eq!(v.x, 101.0);
    assert_eq!(v.y, 102.0);
    assert_eq!(v.z, 103.0);
}

#[test]
fn vec3_add_assign() {
    let mut v = Vec3::new(1, 2, 3);
    v += Vec3::new(4, 5, 6);
    assert_eq!(v.x, 5.0);
    assert_eq!(v.y, 7.0);
    assert_eq!(v.z, 9.0);
}

#[test]
fn vec3_add_assign_scalar() {
    let mut v = Vec3::new(1, 2, 3);
    v += 10.0;
    assert_eq!(v.x, 11.0);
    assert_eq!(v.y, 12.0);
    assert_eq!(v.z, 13.0);
}

#[test]
fn vec3_sub() {
    let v = Vec3::new(4, 5, 6) - Vec3::new(1, 3, 5);
    assert_eq!(v.x, 3.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 1.0);
}

#[test]
fn vec3_sub_scalar() {
    assert_eq!(Vec3::new(4, 5, 6) - 1.0, Vec3::new(3, 4, 5));
    assert_eq!(
        Vec3::from_scalar(10.0) - Vec3::new(4, 5, 6),
        Vec3::new(6, 5, 4)
    );
}

#[test]
fn vec3_mul() {
    assert_eq!(
        Vec3::new(4, 5, 6) * Vec3::new(2, 3, 4),
        Vec3::new(8, 15, 24)
    );
}

#[test]
fn vec3_mul_scalar() {
    assert_eq!(Vec3::new(4, 5, 6) * 10.0, Vec3::new(40, 50, 60));
}

#[test]
fn vec3_mul_assign_scalar() {
    let mut v = Vec3::new(1, 2, 3);
    v *= 10.0;
    assert_eq!(v.x, 10.0);
    assert_eq!(v.y, 20.0);
    assert_eq!(v.z, 30.0);
}

#[test]
fn vec3_div_scalar() {
    assert_eq!(Vec3::new(4, 5, 6) / 2.0, Vec3::new(2, 2.5, 3));
}

#[test]
fn vec3_div_assign_scalar() {
    let mut v = Vec3::new(1, 2, 3);
    v /= 10.0;
    assert_eq!(v.x, 0.1);
    assert_eq!(v.y, 0.2);
    assert!((v.z - 0.3).abs() <= f64::EPSILON);
}

#[test]
fn vec3_dot() {
    assert_eq!(Vec3::new(1, 2, 3).dot(Vec3::new(2, 3, 4)), 20.0);
}

#[test]
fn vec3_cross() {
    assert_eq!(
        Vec3::new(1, 0, 0).cross(Vec3::new(0, 1, 0)),
        Vec3::new(0, 0, 1)
    );
}

#[test]
fn vec3_norm() {
    let l = Vec3::new(1, 1, 1).norm();
    let r = Vec3::new(3f64.sqrt() / 3.0, 3f64.sqrt() / 3.0, 3f64.sqrt() / 3.0);
    assert!((l.x - r.x).abs() <= f64::EPSILON);
    assert!((l.y - r.y).abs() <= f64::EPSILON);
    assert!((l.z - r.z).abs() <= f64::EPSILON);
}

#[test]
fn vec3_less() {
    assert!(Vec3::new(0, 0, 0) < Vec3::new(1, 1, 1));

    let x = Vec3::new(0, 2, 0);
    let y = Vec3::new(1, 1, 1);

    assert_eq!(x.partial_cmp(&y), None);

    assert!(!(x < y));
    assert!(!(x > y));
    assert!(!(x == y));
}
