use crate::packs::Pack;
use crate::common::*;
use std::collections::HashMap;

pub struct VoxelEngineBase;

impl Pack for VoxelEngineBase {
    fn get_identifier(&self, obj: &str) -> Identifier {
        Identifier::new("base", obj)
    }

    fn get_voxels(&self) -> Vec<Identifier> {
        let mut voxels = Vec::new();
        voxels.push(self.get_identifier("dev_tile"));
        voxels.push(self.get_identifier("err_tile"));
        voxels
    }

    fn get_levels(&self) -> Vec<Identifier> {
        let mut levels = Vec::new();
        levels.push(self.get_identifier("lvdev"));
        levels
    }

    fn generate(&self, level_id: Identifier, chunk_coords: Coordinates) -> Option<ChunkData> {
        let mut new_contents =  HashMap::<Coordinates, Option<Voxel>>::new();
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if chunk_coords.y >= 0 {
                        new_contents.insert(Coordinates::new((x, y, z), CoordType::Relative), None);
                    } else {
                        new_contents.insert(Coordinates::new((x, y, z), CoordType::Relative), Some(Voxel::new(self.get_identifier("dev_tile"))));
                    }
                }
            }
        }
        Some(ChunkData { contents: new_contents })
    }
}