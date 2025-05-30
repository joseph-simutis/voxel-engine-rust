use bevy::prelude::*;
mod client;
mod server;
mod packs;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(packs::RegisteredPacks::new())
        .insert_resource(server::Universe::new())
        .add_systems(Startup, (server::startup, client::startup).chain())
        .add_systems(Update, (server::update, client::update))
        .run();
}