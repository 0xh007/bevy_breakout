use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

fn main(){
    App::build()
        .add_default_plugins()
        .run();
}

struct Paddle {
    speed: f32,
}

struct Ball {
    velocity: Vec3,
}

struct Scoreboard {
    score: usize,
}

enum Collider {
    Solid,
    Scorable,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world
    commands
        // cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // paddle
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            translation: Translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(120.0, 30.0),
            },
            ..Default::default()
        })
        .with(Paddle { speed: 500.0 })
        .with(Collider::Solid)
        // ball
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
            translation: Translation(Vec3::new(0.0, -50.0, 1.0)),
            sprite: Sprite {
                size: Vec2::new(30.0, 30.0),
            },
            ..Default::default()
        })
        .with(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize()
        })
        // scoreboard
        .spawn(TextComponents {
        }
}
