use bevy::prelude::*;

pub fn startup() {
    println!("Server Initialized!");
}

pub fn update(time: Res<Time>) {
    let delta = time.delta();
}