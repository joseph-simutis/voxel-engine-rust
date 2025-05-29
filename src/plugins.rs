use bevy::prelude::*;
use crate::client;
use crate::server;

pub struct VoxelEngineClientPlugin;
pub struct VoxelEngineServerPlugin;

impl Plugin for VoxelEngineClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, client::startup);
        app.add_systems(Update, client::update);
    }
}

impl Plugin for VoxelEngineServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, server::startup);
        app.add_systems(Update, server::update);
    }
}