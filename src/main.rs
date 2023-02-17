use bevy::{prelude::*, winit::WinitSettings};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::resource::ResourcePlugin;

mod ant;
mod camera;
mod resource;

#[derive(Resource)]
pub struct PlaceHolderGraphics {
    texture_atlas: Handle<TextureAtlas>,
    player_index: usize,
}

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
        .insert_resource(ClearColor(Color::Rgba {
            red: 0.2,
            green: 0.4,
            blue: 0.2,
            alpha: 0.0,
        }))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(ShapePlugin)
        .add_plugin(ResourcePlugin)
        //.add_startup_system_to_stage(StartupStage::PreStartup, load_graphics)
        .add_startup_system(spawn_camera)
        //.add_startup_system(spawn_player)
        //.add_startup_system(create_ui)
        //.add_system(button_clicked)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands, graphics: Res<PlaceHolderGraphics>) {
    let sprite = TextureAtlasSprite::new(graphics.player_index);
    commands.spawn(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: graphics.texture_atlas.clone(),
        ..default()
    });
}

fn load_graphics(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_assets: ResMut<Assets<TextureAtlas>>,
) {
    let image_handle = assets.load("player_placeholder.png");
    let mut atlas = TextureAtlas::new_empty(image_handle, Vec2::splat(256.0));
    let player_index = atlas.add_texture(Rect {
        min: Vec2::splat(0.0),
        max: Vec2::splat(32.0),
    });

    let atlas_handle = texture_assets.add(atlas);

    commands.insert_resource(PlaceHolderGraphics {
        texture_atlas: atlas_handle,
        player_index: player_index,
    })
}

fn create_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                //justify_content: JustifyContent::FlexEnd,
                flex_direction: FlexDirection::Column,
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
                            size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
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
