use rust_ray::math::*;

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

#[test]
fn voxel_objs() {
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

    let grid = VoxelGrid::new(bounds, voxel_size, &boxes);

    assert_eq!(grid.voxel_objs(VoxelIndex(0, 0, 0)), [0, 1, 2]);
    assert_eq!(grid.voxel_objs(VoxelIndex(1, 0, 0)), []);

    assert_eq!(grid.voxel_objs(VoxelIndex(1, 1, 1)), [3, 4]);
    assert_eq!(grid.voxel_objs(VoxelIndex(2, 1, 1)), [3, 4]);
    assert_eq!(grid.voxel_objs(VoxelIndex(4, 2, 2)), [3, 4]);
    assert_eq!(grid.voxel_objs(VoxelIndex(5, 3, 3)), [4]);
    assert_eq!(grid.voxel_objs(VoxelIndex(5, 4, 3)), []);
}
