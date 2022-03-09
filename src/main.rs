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
    commands.spawn_bundle(UiCameraBundle::default());

    let rows = 6;
    let columns = 5;

    let canvas_node = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    };

    let grid_node = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::ColumnReverse,
            aspect_ratio: Some(columns as f32 / rows as f32),
            ..Default::default()
        },
        ..Default::default()
    };

    let row_node = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::RowReverse,
            ..Default::default()
        },
        ..Default::default()
    };

    let box_node = |row, column| NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        color: if row % 2 == 0 {
            if column % 2 == 0 {
                Color::RED.into()
            } else {
                Color::YELLOW.into()
            }
        } else {
            if column % 2 == 0 {
                Color::BLUE.into()
            } else {
                Color::GREEN.into()
            }
        },
        ..Default::default()
    };

    commands
        .spawn_bundle(canvas_node)
        .with_children(|canvas_parent| {
            canvas_parent
                .spawn_bundle(grid_node)
                .with_children(|grid_parent| {
                    for row in 0..rows {
                        grid_parent
                            .spawn_bundle(row_node.clone())
                            .with_children(|row_parent| {
                                for column in 0..columns {
                                    row_parent.spawn_bundle(box_node(row, column));
                                }
                            });
                    }
                });
        });
}
