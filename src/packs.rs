use bevy::prelude::*;
use crate::common::*;
use std::collections::HashMap;
mod base;
// In order to add your own pack, make sure to add it as a new module:
// mod pack_module;

#[derive(Resource)]
pub struct RegisteredPacks {
    pub contents: HashMap<String, Box<dyn Pack>>,
    pub voxels: Vec<GlobalIdentifier>,
    pub levels: Vec<GlobalIdentifier>,
}

impl RegisteredPacks {
    pub fn new() -> RegisteredPacks {
        let mut packs = RegisteredPacks {
            contents: HashMap::new(),
            voxels: Vec::new(),
            levels: Vec::new(),
        };

        // The base pack shouldn't be required. Given a proper replacement, the game should work perfectly well, even without the base pack.
        packs.contents.insert(String::from("base"), Box::new(base::VoxelEngineBase::new()));
        // A new pack can be introduced like follows, replacing with your pack's name as needed:
        // packs.contents.insert(String::from("pack_name"), Box::new(pack_module::PackStruct::new()));

        packs.voxels.push(GlobalIdentifier::new("required", "air"));
        for (pack_name, pack) in &packs.contents {
            for voxel in pack.get_voxels() {
                packs.voxels.push(voxel);
            }
            for level in pack.get_levels() {
                packs.levels.push(level);
            }
        }

        packs
    }

    pub fn generate(&self, level_id: GlobalIdentifier, coords: ChunkCoordinates) -> Option<[GlobalIdentifier; 4096]> {
        for (pack_name, pack) in &self.contents {
            if *pack_name == level_id.pack {
                return pack.generate(level_id, coords)
            }
        }
        None
    }
}

// Please ensure that the identifiers you return have the pack value set to the name of your pack.
pub trait Pack: Send + Sync {
    fn get_voxels(&self) -> Vec<GlobalIdentifier>;

    fn get_levels(&self) -> Vec<GlobalIdentifier>;

    // The global identifiers this method outputs will be incorperated into the voxel palette of the containing level.
    // Returning None for this method should only be done if something is wrong with the parameters.
    fn generate(&self, level_id: GlobalIdentifier, coords: ChunkCoordinates) -> Option<[GlobalIdentifier; 4096]>;
}