#![feature(default_free_fn)]

use std::default::default;

use bevy::prelude::*;
use loader::CustomLoader;

mod loader;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .init_asset_loader::<CustomLoader>()
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_scene(asset_server.load("foo.test"));

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(5.0, 5.0, 20.0)
            .looking_at(Vec3::new(5.0, 5.0, 0.0), Vec3::Y),
        ..default()
    });
}
