use bevy::prelude::*;
use std::collections::HashMap;
use crate::packs::*;

#[derive(Hash, Eq, PartialEq, Clone)]
// pack represents the source pack for the object; obj represents the name of the object. obj need only be unique within a given pack, but the pack needs to be globally unique.
pub struct Identifier {
    pub pack: String,
    pub obj: String,
}

impl Identifier {
    pub fn new(pack: &str, obj: &str) -> Identifier {
        Identifier {
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
    pub x: i64,
    pub y: i64,
    pub z: i64,
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
    pub fn new(x: i64, y: i64, z: i64) -> RelativeCoordinates {
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
    pub levels: HashMap<Identifier, Level>
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
    pub fn generate(&mut self, registered_packs: Res<RegisteredPacks>, level_id: Identifier, coords: ChunkCoordinates) -> bool {
        let generated = registered_packs.generate(level_id.clone(), coords);
        return match generated {
            None => { false }
            Some(data) => {
                self.levels.get_mut(&level_id).expect(&*format!("Unknown level: {}", &level_id.to_string())).chunks.insert(coords, data);
                true
            }
        }
    }
}

pub struct Level {
    pub chunks: HashMap<ChunkCoordinates, ChunkData>,
}

impl Level {
    pub fn new() -> Level {
        Level { chunks: HashMap::new() }
    }
}

pub struct Chunk {
    pub data: ChunkData,
    pub modified: bool,
}

impl Chunk {
    pub fn new(data: ChunkData) -> Chunk {
        Chunk {
            data: data,
            modified: false,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ChunkData {
    pub contents: HashMap<RelativeCoordinates, Option<Voxel>>
}

// In this case, "complete" means that the chunk data contains values for every block inside the chunk.
// If a chunk is rendered with incomplete data, it may cause issues.
// "Excessive" means that the chunk data contains values for blocks outside the chunk.
// If chunk data is excessive, then that will most likely not cause issues, but it will result in storing unused data.
// A given chunk's data can be both incomplete and excessive.
impl ChunkData {
    pub fn is_complete(&self) -> bool {
        self.clone() == self.completed()
    }

    pub fn is_excessive(&self) -> bool {
        self.clone() != self.trimmed()
    }

    pub fn completed(&self) -> ChunkData {
        let mut new_data =  ChunkData { contents: self.contents.clone() };
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    new_data.contents.entry(RelativeCoordinates::new(x, y, z)).or_insert(None);
                }
            }
        }
        new_data
    }

    pub fn trimmed(&self) -> ChunkData {
        let mut new_data =  ChunkData { contents: self.contents.clone() };
        for (coords, voxel) in &self.contents {
            if !coords.inside_chunk() {
                new_data.contents.remove(&coords);
            }
        }
        new_data
    }
}

#[derive(Clone, Eq, PartialEq)]
// Represents an individual voxel in the world.
pub struct Voxel {
    id: Identifier
}

impl Voxel {
    pub fn new(id: Identifier) -> Voxel {
        Voxel {
            id: id,
        }
    }
}