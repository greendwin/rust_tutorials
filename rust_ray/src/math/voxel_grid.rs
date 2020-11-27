use crate::math::*;
use std::collections::HashSet;

pub trait BoundBox {
    fn get_bounds(&self) -> AABB;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct VoxelIndex(pub i32, pub i32, pub i32);

impl VoxelIndex {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self(x as i32, y as i32, z as i32)
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn as_tuple(&self) -> (usize, usize, usize) {
        (self.0 as usize, self.1 as usize, self.2 as usize)
    }
}

#[derive(Copy, Clone, Debug)]
struct Voxel {
    offset: u32,
    count: u32,
}

impl Voxel {
    fn new() -> Self {
        Voxel {
            offset: 0,
            count: 0,
        }
    }
}

pub struct VoxelGrid {
    nx: usize,
    ny: usize,
    nz: usize,
    bounds: AABB,
    voxel_size: f64,
    objs: Vec<u32>,
    cells: Vec<Voxel>,
}

impl VoxelGrid {
    pub fn new(bounds: AABB, voxel_size: f64, boxes: &[AABB]) -> Self {
        let num_cells = bounds.size() / voxel_size;

        let nx = num_cells.x.ceil() as usize;
        let ny = num_cells.y.ceil() as usize;
        let nz = num_cells.z.ceil() as usize;

        println!("DBG: calc_voxel_cells...");
        let (cells, objs) = calc_voxel_cells(&boxes, &bounds, (nx, ny, nz), voxel_size);

        println!("DBG: grid ready...");

        Self {
            nx,
            ny,
            nz,
            bounds,
            voxel_size,
            cells,
            objs,
        }
    }

    #[inline]
    pub fn voxel_index(&self, pos: impl Into<Vec3>) -> Option<VoxelIndex> {
        voxel_index(pos.into(), &self.bounds, self.voxel_size)
    }

    pub fn voxel_objs(&self, index: VoxelIndex) -> &[u32] {
        let (x, y, z) = index.as_tuple();
        let index = x * self.ny * self.nz + y * self.nz + z;

        let p = self.cells[index];

        &self.objs[p.offset as usize..(p.offset + p.count) as usize]
    }

    #[inline]
    pub fn dims(&self) -> (usize, usize, usize) {
        (self.nx, self.ny, self.nz)
    }

    pub fn trace_objs(&self, ray: &Ray, found: &mut HashSet<usize>) {
        found.clear();

        let mut cur = match self.voxel_index(ray.orig) {
            None => return,
            Some(p) => p,
        };

        for &p in self.voxel_objs(cur) {
            found.insert(p as usize);
        }

        let norm_dir = ray.dir.norm();
        let mut cur_pos = ray.orig;

        loop {
            let next_pos = cur_pos + norm_dir * self.voxel_size;
            let next = match self.voxel_index(next_pos) {
                None => return,
                Some(p) => p,
            };

            if next == cur {
                cur_pos = next_pos;
                cur = next;
                continue;
            }

            let mut neighbours = [VoxelIndex::zero(); 9];
            let count = get_neighbours(cur, next, &mut neighbours);

            for &cell_idx in &neighbours[0..count] {
                if cell_idx.0 < 0 || cell_idx.1 < 0 || cell_idx.2 < 0 {
                    continue;
                }
                if cell_idx.0 >= self.nx as i32
                    || cell_idx.1 >= self.ny as i32
                    || cell_idx.2 >= self.nz as i32
                {
                    continue;
                }

                for &p in self.voxel_objs(cell_idx) {
                    found.insert(p as usize);
                }
            }

            cur_pos = next_pos;
            cur = next; // update
        }
    }
}

#[inline]
fn voxel_offset(idx: VoxelIndex, dims: &(usize, usize, usize)) -> usize {
    let VoxelIndex(x, y, z) = idx;
    let (_nx, ny, nz) = dims;
    x as usize * ny * nz + y as usize * nz + z as usize
}

fn voxel_index(pos: Vec3, bounds: &AABB, voxel_size: f64) -> Option<VoxelIndex> {
    if !bounds.is_inside(pos) {
        return None;
    }

    let vpos = (pos - bounds.min) / voxel_size;
    Some(VoxelIndex(vpos.x as i32, vpos.y as i32, vpos.z as i32))
}

fn get_neighbours(cur: VoxelIndex, next: VoxelIndex, r: &mut [VoxelIndex; 9]) -> usize {
    let VoxelIndex(cx, cy, cz) = cur;

    let mut count = 0;
    for dx in &[-1i32, 0, 1] {
        for dy in &[-1i32, 0, 1] {
            for dz in &[-1i32, 0, 1] {
                let x = cx + dx;
                let y = cy + dy;
                let z = cz + dz;

                // NOTE: include next itself (only cur should be already processed)

                let mut is_neighbour = false;
                if x == next.0 && y == next.1 && (z - next.2).abs() <= 1 {
                    is_neighbour = true;
                } else if x == next.0 && z == next.2 && (y - next.1).abs() <= 1 {
                    is_neighbour = true;
                } else if y == next.1 && z == next.2 && (x - next.0).abs() <= 1 {
                    is_neighbour = true;
                }

                if is_neighbour {
                    r[count] = VoxelIndex(x, y, z);
                    count += 1;
                }
            }
        }
    }

    count
}

fn calc_voxel_cells(
    boxes: &[AABB],
    bounds: &AABB,
    dims: (usize, usize, usize),
    voxel_size: f64,
) -> (Vec<Voxel>, Vec<u32>) {
    let (nx, ny, nz) = dims;
    let num_cells = nx * ny * nz;

    let mut cells = vec![Voxel::new(); num_cells];

    // calc voxels range
    let indexes: Vec<_> = boxes
        .iter()
        .map(|p| {
            let obj_bounds = bounds.intersect(p)?;
            let min_index = voxel_index(obj_bounds.min, bounds, voxel_size)?;
            let max_index = voxel_index(obj_bounds.max - f64::EPSILON, bounds, voxel_size)?;
            Some((min_index, max_index))
        })
        .collect();

    // first pass: increment cell's objs count
    for p in &indexes {
        let (min_index, max_index) = match p {
            None => continue,
            Some(p) => p,
        };

        for x in min_index.0..max_index.0 + 1 {
            for y in min_index.1..max_index.1 + 1 {
                for z in min_index.2..max_index.2 + 1 {
                    let offset = voxel_offset(VoxelIndex(x, y, z), &dims);
                    cells[offset].count += 1;
                }
            }
        }
    }

    // second pass: count matches and store voxels' offsets
    let mut num_objs = 0;
    for offset in 0..num_cells {
        let p = &mut cells[offset];
        if p.count > 0 {
            p.offset = num_objs;
            num_objs += p.count;
        }
    }

    // third pass: write objects
    let num_objs = num_objs as usize;
    let mut cells_offset: Vec<_> = cells.iter().map(|p| p.offset as usize).collect();
    let mut objs = vec![u32::MAX; num_objs];

    for (obj_idx, p) in indexes.iter().enumerate() {
        let (min_index, max_index) = match p {
            None => continue,
            Some(p) => p,
        };

        for x in min_index.0..max_index.0 + 1 {
            for y in min_index.1..max_index.1 + 1 {
                for z in min_index.2..max_index.2 + 1 {
                    let offset = voxel_offset(VoxelIndex(x, y, z), &dims);
                    objs[cells_offset[offset]] = obj_idx as u32;
                    cells_offset[offset] += 1;
                }
            }
        }
    }

    objs.iter().for_each(|&idx| assert!(idx != u32::MAX));

    (cells, objs)
}
