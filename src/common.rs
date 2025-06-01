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
    pub fn new(new_pack: &str, new_obj: &str) -> Identifier {
        Identifier {
            pack: new_pack.to_string(),
            obj: new_obj.to_string(),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Coordinates {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub ctype: CoordType
}

impl Coordinates {
    pub fn new(coords: (i64, i64, i64), ctype: CoordType) -> Coordinates {
        Coordinates {
            x: coords.0,
            y: coords.1,
            z: coords.2,
            ctype: ctype,
        }
    }

    // Only applicable for Relative coordinates. In other cases, always returns true.
    pub fn inside_chunk(&self) -> bool {
        if self.ctype == CoordType::Relative {
            let range = 0..16;
            range.contains(&self.x) && range.contains(&self.y) && range.contains(&self.z)
        } else {
            true
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
// Chunk refers to the coordinates of the chunk itself.
// World refers to the coordinates of a voxel in the world.
// Relative refers to the coordinates of a voxel within its containing chunk.
pub enum CoordType {
    Chunk,
    World,
    Relative,
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
        for pack in registered_packs.contents.values() {
            for level in pack.get_levels().iter() {
                self.levels.insert(level.clone(), Level::new());
                i += 1;
            }
        }
        i
    }
}

pub struct Level {
    pub chunks: HashMap<Coordinates, ChunkData>,
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
    pub contents: HashMap<Coordinates, Option<Voxel>>
}

// In this case, "complete" means that the chunk data contains values for every block inside the chunk.
// If an incomplete chunk is rendered without first completing it, it may cause a crash.
// "Excessive", then, means that the chunk data contains values for blocks outside the chunk.
// If chunk data is excessive, then that will most likely not cause issues, but it will result in storing more data than needed.
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
                    new_data.contents.entry(Coordinates::new((x, y, z), CoordType::Relative)).or_insert(None);
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
pub struct Voxel {
    id: Identifier
}

impl Voxel {
    pub fn new(new_id: Identifier) -> Voxel {
        Voxel  {
            id: new_id,
        }
    }
}