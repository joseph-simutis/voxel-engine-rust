use crate::packs::Pack;
use crate::common::*;
use std::collections::HashMap;

pub struct VoxelEngineBase {
    voxels: Vec<Identifier>,
    levels: Vec<Identifier>,
}

impl VoxelEngineBase {
    pub fn new() -> VoxelEngineBase {
        let mut base = VoxelEngineBase {
            voxels: Vec::new(),
            levels: Vec::new(),
        };
        base.voxels.push(base.get_identifier("dev_tile"));
        base.voxels.push(base.get_identifier("err_tile"));
        base.levels.push(base.get_identifier("dev_level"));
        base
    }

    pub fn get_identifier(&self, obj: &str) -> Identifier { Identifier::new("base", obj) }
}

impl Pack for VoxelEngineBase {
    fn get_voxels(&self) -> Vec<Identifier> { self.voxels.clone() }

    fn get_levels(&self) -> Vec<Identifier> { self.levels.clone() }

    fn generate(&self, level_id: Identifier, coords: ChunkCoordinates) -> Option<ChunkData> {
        let mut new_data =  ChunkData { contents: HashMap::<RelativeCoordinates, Option<Voxel>>::new() };
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if coords.y >= 0 {
                        new_data.contents.insert(RelativeCoordinates::new(x, y, z), None);
                    } else {
                        new_data.contents.insert(RelativeCoordinates::new(x, y, z), Some(Voxel::new(self.get_identifier("dev_tile"))));
                    }
                }
            }
        }
        Some(new_data)
    }
}