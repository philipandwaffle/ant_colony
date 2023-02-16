use bevy::{prelude::*, winit::WinitSettings};

#[derive(Component, Clone, Copy, Debug)]
enum ButtonType {
    foo,
    bar,
    baz,
}
static buttons: [ButtonType; 3] = [ButtonType::foo, ButtonType::bar, ButtonType::baz];

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(create_camera)
        .add_startup_system(create_ui)
        .add_system(button_clicked)
        .run();
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn create_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        })
        .with_children(|commands| {
            for i in 0..3 {
                commands
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(15.0 * 9.0 / 16.0), Val::Percent(15.0)),
                            align_self: AlignSelf::FlexEnd,
                            margin: UiRect::all(Val::Percent(2.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(buttons[i]);
            }
        });
}

fn button_clicked(q: Query<(&Interaction, &ButtonType), Changed<Interaction>>) {
    for (i, b) in &q {
        if matches!(i, Interaction::Clicked) {
            println!("{:?}, {:?}", i, b);
        }
    }
}
