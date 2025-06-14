use crate::packs::Pack;
use crate::common::*;
use std::collections::HashMap;
// In order to add an addon, make sure to add it as a new module:
// mod addon_module;

pub struct VoxelEngineBase {
    registered_addons: HashMap<String, Box<dyn VoxelEngineBaseAddon>>,
    voxels: Vec<GlobalIdentifier>,
    levels: Vec<GlobalIdentifier>,
}

impl VoxelEngineBase {
    pub fn new() -> VoxelEngineBase {
        let mut base = VoxelEngineBase {
            registered_addons: HashMap::new(),
            voxels: Vec::new(),
            levels: Vec::new(),
        };
        // An addon to the base pack can be registered as such, replacing with your addon's name as needed:
        // base.registered_addons.insert(String::from("addon_name"), Box::new(addon_module::AddonStruct::new()))
        base.voxels.push(base.get_identifier("dev_tile"));
        base.voxels.push(base.get_identifier("err_tile"));
        base.levels.push(base.get_identifier("dev_level"));
        base
    }

    pub fn get_identifier(&self, obj: &str) -> GlobalIdentifier { GlobalIdentifier::new("base", obj) }
}

impl Pack for VoxelEngineBase {
    fn get_voxels(&self) -> Vec<GlobalIdentifier> { self.voxels.clone() }

    fn get_levels(&self) -> Vec<GlobalIdentifier> { self.levels.clone() }

    fn generate(&self, level_id: GlobalIdentifier, coords: ChunkCoordinates) -> Option<[GlobalIdentifier; 4096]> {
        if level_id == self.get_identifier("dev_level") {
            if coords.y >= 0 {
                Some(std::array::from_fn(|_| GlobalIdentifier::new("required", "air")))
            } else {
                Some(std::array::from_fn(|_| self.get_identifier("dev_tile")))
            }
        } else {
            None
        }
    }
}

pub trait VoxelEngineBaseAddon: Send + Sync {}