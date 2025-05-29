use bevy::prelude::*;
mod client;
mod server;
mod plugins;

fn main() {
    // TODO: Implement a way for a user to select whether to use the client or the server
    let use_client = true;
    if use_client {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(plugins::VoxelEngineClientPlugin)
            .run();
    } else {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(plugins::VoxelEngineServerPlugin)
            .run();
    }
}