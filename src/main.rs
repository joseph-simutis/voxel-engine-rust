use bevy::prelude::*;
mod client;
mod common;
mod server;
mod packs;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(packs::RegisteredPacks::new())
        .insert_resource(common::Universe::new())
        .insert_state(client::GameState::TitleScreen)
        .add_systems(Startup, (server::startup, client::startup).chain())
        .add_systems(Update, (server::update, client::update))
        .run();
}