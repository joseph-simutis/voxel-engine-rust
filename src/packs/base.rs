use crate::packs::Pack;
use crate::server::*;
use std::collections::HashMap;

pub struct VoxelEngineBase;

impl Pack for VoxelEngineBase {
    fn get_voxels(&self) -> Vec<Identifier> {
        let mut voxels = Vec::new();
        voxels.push(Identifier::new("base", "dev_tile"));
        voxels
    }

    fn get_levels(&self) -> Vec<Identifier> {
        let mut levels = Vec::new();
        levels.push(Identifier::new("base", "dev"));
        levels
    }

    fn generate(&self, level_id: Identifier, chunk_coords: Coordinates) -> Option<ChunkData> {
        let mut new_contents =  HashMap::<Coordinates, Option<Voxel>>::new();
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if chunk_coords.y >= 0 {
                        new_contents.insert(Coordinates::new((x, y, z)), None);
                    } else {
                        new_contents.insert(Coordinates::new((x, y, z)), Some(Voxel::new(Identifier::new("base", "dev_tile"))));
                    }
                }
            }
        }
        Some(ChunkData { contents: new_contents })
    }
}