#[macro_use]
extern crate lazy_static;

use rust_ray::math::*;
use std::collections::HashSet;

#[test]
fn grid_dimension() {
    let bounds = AABB::new((0, 0, 0), (10, 9, 8));
    let voxel_size = 1.0;
    let grid = VoxelGrid::new(bounds, voxel_size, &[]);

    assert_eq!((10, 9, 8), grid.dims())
}

#[test]
fn voxel_index() {
    let bounds = AABB::new((0, 0, 0), (10, 9, 8));
    let voxel_size = 1.0;
    let grid = VoxelGrid::new(bounds, voxel_size, &[]);

    assert_eq!(grid.voxel_index((0, 0, 0)), Some(VoxelIndex(0, 0, 0)));
    assert_eq!(grid.voxel_index((0.1, 0.1, 0.1)), Some(VoxelIndex(0, 0, 0)));
    assert_eq!(grid.voxel_index((3, 2, 1)), Some(VoxelIndex(3, 2, 1)));

    // outer corner is out of box
    assert_eq!(grid.voxel_index((10, 9, 8)), None);
}

lazy_static! {
    static ref GRID: VoxelGrid = {
        let bounds = AABB::new((0, 0, 0), (10, 9, 8));
        let voxel_size = 1.0;

        let boxes = [
            // cell (0, 0, 0)
            AABB::from((0.2, 0.1, 0.1)),
            AABB::from((0.1, 0.2, 0.1)),
            AABB::from((0.1, 0.1, 0.2)),
            // cell (1, 1, 1)-(4,2,2)
            AABB::new((1.2, 1.1, 1.1), (4.8, 2.9, 2.9)),
            // cell (1, 1, 1)-(5,3,3)
            AABB::new((1.2, 1.1, 1.1), (5, 3, 3)),
        ];

        VoxelGrid::new(bounds, voxel_size, &boxes)
    };
}

#[test]
fn voxel_objs() {
    let grid = &*GRID;

    assert_eq!(grid.voxel_objs(VoxelIndex(0, 0, 0)), [0, 1, 2]);
    assert_eq!(grid.voxel_objs(VoxelIndex(1, 0, 0)), []);

    assert_eq!(grid.voxel_objs(VoxelIndex(1, 1, 1)), [3, 4]);
    assert_eq!(grid.voxel_objs(VoxelIndex(2, 1, 1)), [3, 4]);
    assert_eq!(grid.voxel_objs(VoxelIndex(4, 2, 2)), [3, 4]);
    assert_eq!(grid.voxel_objs(VoxelIndex(5, 3, 3)), [4]);
    assert_eq!(grid.voxel_objs(VoxelIndex(5, 4, 3)), []);
}

fn trace(pos: impl Into<Vec3>, dir: impl Into<Vec3>) -> Vec<usize> {
    let ray = Ray::new(pos.into(), dir.into());
    let mut objs = HashSet::new();
    GRID.trace_objs(&ray, &mut objs);

    let mut objs: Vec<usize> = objs.into_iter().collect();
    objs.sort_unstable();

    objs
}

#[test]
fn trace_out_of_bounds() {
    let objs = trace((-0.5, -0.5, -0.5), (-1, -1, -1));

    assert_eq!(objs.len(), 0);
}

#[test]
fn trace_one_cell() {
    // cell (0, 0, 0)
    let objs = trace((0.5, 0.5, 0.5), (-1, -1, -1));

    assert_eq!(objs, [0, 1, 2]);
}

#[test]
fn trace_another_cell() {
    // cell (1, 1, 1)
    let objs = trace((1.5, 1.5, 1.5), (-1, 0, 0));

    assert_eq!(objs, [3, 4]);
}

#[test]
fn trace_all() {
    // cell (1, 1, 1)
    let objs = trace((0, 0, 0), (1, 1, 1));

    assert_eq!(objs, [0, 1, 2, 3, 4]);
}
