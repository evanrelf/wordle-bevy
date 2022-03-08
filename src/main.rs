use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Wordle".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let rows = 6;
    let columns = 5;
    let size = Vec3::new(50.0, 50.0, 0.0);
    let spacing = 20.0;
    let width = columns as f32 * (size.x + spacing) - spacing;
    let offset = Vec3::new(-(width - size.x) / 2.0, 100.0, 0.0);

    for row in 0..rows {
        let y_position = row as f32 * (size.y + spacing);
        for column in 0..columns {
            let position = Vec3::new(column as f32 * (size.x + spacing), y_position, 0.0) + offset;
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    ..Default::default()
                },
                transform: Transform {
                    translation: position,
                    scale: size,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
