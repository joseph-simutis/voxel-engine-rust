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
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinates {
    pub fn new(coords: (i64, i64, i64)) -> Coordinates {
        Coordinates {
            x: coords.0,
            y: coords.1,
            z: coords.2,
        }
    }

    pub fn tuple(&self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }

    // Only applies for voxel coordinates inside a chunk, not chunk coordinates or world coordinates.
    pub fn outside_chunk(&self) -> bool {
        let range = 0..16;
        !range.contains(&self.x) || !range.contains(&self.y) || !range.contains(&self.z)
    }
}

#[derive(Resource)]
pub struct Universe {
    pub levels: HashMap<Identifier, Level>
}

impl Universe {
    pub fn new() -> Universe {
        Universe {
            levels: HashMap::new(),
        }
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
        Level {
            chunks: HashMap::new(),
        }
    }
}

pub struct Chunk {
    pub data: ChunkData,
    pub modified: bool,
}

impl Chunk {
    pub fn new(new_data: ChunkData) -> Chunk {
        Chunk {
            data: new_data,
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
    pub fn is_complete(self) -> bool {
        self.clone() == self.completed()
    }

    pub fn is_excessive(self) -> bool {
        self.clone() != self.trimmed()
    }

    pub fn completed(self) -> ChunkData {
        let mut new_contents =  self.contents.clone();
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    new_contents.entry(Coordinates::new((x, y, z))).or_insert(None);
                }
            }
        }
        ChunkData { contents: new_contents }
    }

    pub fn trimmed(self) -> ChunkData {
        let mut new_contents = self.contents.clone();
        for (coords, voxel) in self.contents {
            if coords.outside_chunk() {
                new_contents.remove(&coords);
            }
        }
        ChunkData { contents: new_contents }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Voxel {
    id: Identifier
}

pub fn startup(mut universe: ResMut<Universe>, registered_packs: Res<RegisteredPacks>) {
    println!("[SERVER] Initializing...");
    println!("[SERVER] Registered {} levels...", universe.add_levels(registered_packs));
    println!("[SERVER] Initialized!");
}

pub fn update(time: Res<Time>) {
    let _delta = time.delta();
}