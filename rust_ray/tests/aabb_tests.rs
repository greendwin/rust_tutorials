use rust_ray::math::{Sphere, Vec3, AABB};

#[test]
fn create_box() {
    let bx = AABB::new((0, 0, 0), (1, 2, 3));
    assert_eq!(bx.min, Vec3::zero());
    assert_eq!(bx.max, Vec3::new(1, 2, 3));
}

#[test]
fn is_inside() {
    let bx = AABB::new((0, 0, 0), (1, 1, 1));

    assert!(bx.is_inside((0, 0, 1)));
    assert!(bx.is_inside((1, 0, 1)));
    assert!(bx.is_inside((1, 1, 1)));
    assert!(bx.is_inside((0.5, 0.5, 0.5)));

    assert!(!bx.is_inside((2, 0, 0)));
    assert!(!bx.is_inside((0, 2, 0)));
    assert!(!bx.is_inside((0, 0, 2)));

    assert!(!bx.is_inside((-2, 0, 0)));
    assert!(!bx.is_inside((0, -2, 0)));
    assert!(!bx.is_inside((0, 0, -2)));
}

#[test]
fn is_intersects() {
    let x = AABB::new((0, 0, 0), (1, 1, 1));

    assert!(x.is_intersects(&AABB::from((0.5, 0.5, 0.5))));
    assert!(x.is_intersects(&AABB::new((1, 1, 1), (2, 2, 2))));

    assert!(!x.is_intersects(&AABB::from((1.5, 0.5, 0.5))));
    assert!(!x.is_intersects(&AABB::from((0.5, 1.5, 0.5))));
    assert!(!x.is_intersects(&AABB::from((0.5, 0.5, 1.5))));

    assert!(!x.is_intersects(&AABB::from((-0.5, 0.5, 0.5))));
    assert!(!x.is_intersects(&AABB::from((0.5, -0.5, 0.5))));
    assert!(!x.is_intersects(&AABB::from((0.5, 0.5, -0.5))));
}

#[test]
fn intersect() {
    let x = AABB::new((0, 0, 0), (1, 1, 1));

    assert_eq!(
        x.intersect(&AABB::from((0.5, 0.5, 0.5))),
        Some(AABB::from((0.5, 0.5, 0.5)))
    );
    assert_eq!(
        x.intersect(&AABB::new((1, 1, 1), (2, 2, 2))),
        Some(AABB::from((1, 1, 1)))
    );
    assert_eq!(
        x.intersect(&AABB::new((0.5, 0.6, 0.7), (2, 2, 2))),
        Some(AABB::new((0.5, 0.6, 0.7), (1, 1, 1)))
    );
    assert_eq!(
        x.intersect(&AABB::new((0.5, 0.6, 0.7), (0.6, 0.7, 0.8))),
        Some(AABB::new((0.5, 0.6, 0.7), (0.6, 0.7, 0.8)))
    );

    assert_eq!(
        x.intersect(&AABB::new((0.5, 0.5, 0.5), (2, 0.7, 0.7))),
        Some(AABB::new((0.5, 0.5, 0.5), (1, 0.7, 0.7)))
    );
    assert_eq!(
        x.intersect(&AABB::new((0.5, 0.5, 0.5), (0.7, 2, 0.7))),
        Some(AABB::new((0.5, 0.5, 0.5), (0.7, 1, 0.7)))
    );
    assert_eq!(
        x.intersect(&AABB::new((0.5, 0.5, 0.5), (0.7, 0.7, 2))),
        Some(AABB::new((0.5, 0.5, 0.5), (0.7, 0.7, 1)))
    );

    assert_eq!(x.intersect(&AABB::from((1.5, 0.5, 0.5))), None);
    assert_eq!(x.intersect(&AABB::from((0.5, 1.5, 0.5))), None);
    assert_eq!(x.intersect(&AABB::from((0.5, 0.5, 1.5))), None);

    assert_eq!(x.intersect(&AABB::from((-0.5, 0.5, 0.5))), None);
    assert_eq!(x.intersect(&AABB::from((0.5, -0.5, 0.5))), None);
    assert_eq!(x.intersect(&AABB::from((0.5, 0.5, -0.5))), None);
}

#[test]
fn expand() {
    let mut x = AABB::zero();
    x.expand(&AABB::new((0, 0, 0), (1, 1, 1)));
    assert_eq!(x, AABB::new((0, 0, 0), (1, 1, 1)));

    let mut x = AABB::zero();
    x.expand(&AABB::new((-1, -1, -1), (1, 1, 1)));
    assert_eq!(x, AABB::new((-1, -1, -1), (1, 1, 1)));

    let mut x = AABB::new((-1, -1, -1), (1, 1, 1));
    x.expand(&AABB::new((-0.5, -0.5, -0.5), (0.5, 0.5, 0.5)));
    assert_eq!(x, AABB::new((-1, -1, -1), (1, 1, 1)));
}

#[test]
fn size() {
    let x = AABB::new((-1, -2, -3), (2, 3, 4));
    assert_eq!(x.size(), Vec3::new(3, 5, 7));
}

#[test]
fn from_many() {
    let x = AABB::from_many(&[
        AABB::new((0, 0, 0), (1, 1, 1)),
        AABB::new((3, 3, 3), (4, 4, 4)),
        AABB::new((-10, -3, -3), (-1, -1, -1)),
    ]);

    assert_eq!(x, AABB::new((-10, -3, -3), (4, 4, 4)));
}

#[test]
fn from_sphere() {
    let s = Sphere::new((1, 0, 0), 1);

    assert_eq!(AABB::from(s), AABB::new((0, -1, -1), (2, 1, 1)));
}
