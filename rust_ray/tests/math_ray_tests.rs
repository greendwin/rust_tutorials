use rust_ray::math::Ray;

#[test]
fn ray_at() {
    let r = Ray::new((0, 0, 0), (1, 2, 3));

    assert_eq!(r.at(0), r.orig);
}
