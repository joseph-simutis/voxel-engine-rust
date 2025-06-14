use bevy::prelude::*;
use std::collections::HashMap;
use crate::packs::*;
use bimap::BiMap;

#[derive(Hash, Eq, PartialEq, Clone)]
// pack represents the source pack for the object; obj represents the name of the object. obj need only be unique within a given pack, but the pack needs to be globally unique.
pub struct GlobalIdentifier {
    pub pack: String,
    pub obj: String,
}

impl GlobalIdentifier {
    pub fn new(pack: &str, obj: &str) -> GlobalIdentifier {
        GlobalIdentifier {
            pack: pack.to_string(),
            obj: obj.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.pack, self.obj)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
// Level Coordinates represent the location of a block inside of its level.
pub struct LevelCoordinates {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
// Chunk Coordinates represent the location of a chunk inside of its level.
pub struct ChunkCoordinates {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
// Relative Coordinates represent the location of a block inside of its chunk.
pub struct RelativeCoordinates {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl LevelCoordinates {
    pub fn new(x: i64, y: i64, z: i64) -> LevelCoordinates {
        LevelCoordinates {
            x: x,
            y: y,
            z: z,
        }
    }
}
impl ChunkCoordinates {
    pub fn new(x: i64, y: i64, z: i64) -> ChunkCoordinates {
        ChunkCoordinates {
            x: x,
            y: y,
            z: z,
        }
    }
}
impl RelativeCoordinates {
    pub fn new(x: usize, y: usize, z: usize) -> RelativeCoordinates {
        RelativeCoordinates {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn inside_chunk(&self) -> bool {
        let range = 0..16;
        range.contains(&self.x) && range.contains(&self.y) && range.contains(&self.z)
    }
}

#[derive(Resource)]
pub struct Universe {
    pub levels: HashMap<GlobalIdentifier, Level>
}


impl Universe {
    pub fn new() -> Universe {
        Universe { levels: HashMap::new() }
    }

    pub fn add_levels(&mut self, registered_packs: Res<RegisteredPacks>) -> usize {
        let mut i = 0;
        for level in &registered_packs.levels {
            if !self.levels.contains_key(&level) {
                self.levels.insert(level.clone(), Level::new());
                i += 1;
            }
        }
        i
    }

    // If the level id and coordinates correspond to an existing chunk, it will be overwritten.
    // Will return true if the generation was successful, else return false.
    pub fn generate(&mut self, registered_packs: Res<RegisteredPacks>, level_id: GlobalIdentifier, coords: ChunkCoordinates) -> bool {
        let generated = registered_packs.generate(level_id.clone(), coords);
        return match generated {
            None => { false }
            Some(chunk) => {
                self.levels.get_mut(&level_id).expect(&*format!("Unknown level: {}", &level_id.to_string())).add_global_chunk(coords, chunk);
                true
            }
        }
    }
}

pub struct Level {
    voxel_palette: BiMap<u16, GlobalIdentifier>,
    next_local: u16,
    pub chunks: HashMap<ChunkCoordinates, Chunk>,
}

impl Level {
    pub fn new() -> Level {
        Level {
            voxel_palette: BiMap::new(),
            next_local: 0,
            chunks: HashMap::new(),
        }
    }

    pub fn add_global_chunk(&mut self, chunk_coords: ChunkCoordinates, global_chunk: [GlobalIdentifier; 4096]) {
        self.chunks.insert(chunk_coords, Chunk::new(global_chunk.map(|global_id| {
            if !self.voxel_palette.contains_right(&global_id) {
                self.voxel_palette.insert(self.next_local, global_id.clone());
                self.next_local += 1;
            }
            *self.voxel_palette.get_by_right(&global_id).unwrap()
        }).clone()));
    }
}

// Each voxel is stored as its associated local identifier.
// "modified" refers to whether or not the chunk has been modified after generation.
// If unmodified, the chunk will not be stored in the world in order to save space.
pub struct Chunk {
    voxels: [u16; 4096],
    pub modified: bool,
}

impl Chunk {
    pub fn new(voxels: [u16; 4096]) -> Chunk {
        Chunk {
            voxels: voxels,
            modified: false,
        }
    }

    // Returns None if the coordinates are invalid, else returns Some containing the voxel.
    pub fn get_voxel(&self, coords: RelativeCoordinates) -> Option<u16> {
        if coords.inside_chunk() {
            Some(self.voxels[coords.x + 16*coords.y + 256*coords.z])
        } else {
            None
        }
    }
}