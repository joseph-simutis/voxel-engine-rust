pub mod base;
use crate::common::*;
use std::collections::HashMap;

#[derive(bevy::prelude::Resource)]
pub struct RegisteredPacks {
    pub contents: HashMap<String, Box<dyn Pack>>
}

impl RegisteredPacks {
    pub fn new() -> RegisteredPacks {
        let mut new_packs = RegisteredPacks { contents: HashMap::new() };
        // The base pack shouldn't be required. Given a proper replacement, the game should work perfectly well, even without the base pack.
        new_packs.contents.insert(String::from("base"), Box::new(base::VoxelEngineBase {}));
        new_packs
    }
}

// Please ensure that the identifiers you return have the pack value set to the name of your pack.
pub trait Pack: Send + Sync {
    fn get_identifier(&self, obj: &str) -> Identifier;

    fn get_voxels(&self) -> Vec<Identifier> {
        Vec::new()
    }

    fn get_levels(&self) -> Vec<Identifier> {
        Vec::new()
    }

    // Returning None for this method should only be done if the level does not exist.
    fn generate(&self, level_id: Identifier, chunk_coords: Coordinates) -> Option<ChunkData> {
        None
    }
}