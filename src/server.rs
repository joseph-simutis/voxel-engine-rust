use bevy::prelude::*;
use crate::packs::*;
use crate::common::*;

pub fn startup(
    mut universe: ResMut<Universe>,
    registered_packs: Res<RegisteredPacks>,
) {
    println!("[SERVER] Initializing...");
    println!("[SERVER] Registered {} levels...", universe.add_levels(registered_packs));
    println!("[SERVER] Initialized!");
}

pub fn update(time: Res<Time>) {
    let _delta = time.delta();
}