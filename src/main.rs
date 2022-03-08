use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Wordle".to_string(),
            width: 500.0,
            height: 1000.0,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}
