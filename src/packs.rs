pub mod base;
use crate::server::*;
use std::collections::HashMap;

#[derive(bevy::prelude::Resource)]
pub struct RegisteredPacks {
    // Note: This should be updated to allow any type of pack, but I have not been able to get that to work yet.
    pub contents: HashMap<String, base::VoxelEngineBase>
}

impl RegisteredPacks {
    pub fn new() -> RegisteredPacks {
        RegisteredPacks {
            contents: HashMap::from([
                (String::from("base"), base::VoxelEngineBase {}),
            ])
        }
    }
}

// Please ensure that the identifiers you return have the pack value set to the name of your pack.
pub trait Pack {
    fn get_voxels(&self) -> Vec<Identifier> {
        Vec::new()
    }

    fn get_levels(&self) -> Vec<Identifier> {
        Vec::new()
    }

    fn generate(&self, level_id: Identifier, chunk_coords: Coordinates) -> Option<ChunkData> {
        None
    }
}