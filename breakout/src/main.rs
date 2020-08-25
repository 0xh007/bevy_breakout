use bevy::{
    prelude::*,
    window::WindowMode
};

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            width: 2560,
            height: 1080,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(paddle_movement_system.system())
        .run();
}

struct Paddle {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>, 
) {
    // add entities to the world
    commands

        // - Paddle -
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/blender/paddle/export/paddle.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.51, 0.51, 0.51).into()),
            translation: Translation::new(0.0, 0.0, -30.0),
            ..Default::default()
        })
        .with(Paddle {
            speed: 50.0
        })

        // - Ball -
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/blender/ball/export/ball.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(2.3, 2.3, 0.0).into()),
            translation: Translation::new(0.0, 0.0, -20.0),
            ..Default::default()
        })

        // - Board - 
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/blender/board/export/board.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.0, 0.0, 0.51).into()),
            translation: Translation::new(0.0, -1.0, 0.0),
            ..Default::default()
        })

        // - Light -
        .spawn(LightComponents {
            translation: Translation::new(0.0, 10.0, -10.0),
            ..Default::default()
        })

        // - Cameras -
        // - Game View - 
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                Vec3::new(0.0, 55.0, -75.0),
                Vec3::new(0.0, 0.0, -20.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });

        // - Side View -
        /*
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                Vec3::new(-80.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
        */

        /*
        // - Underneath -
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                Vec3::new(0.0, -65.0, -75.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
        */
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Translation)>,
) {
    for (paddle, mut translation) in &mut query.iter() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction -= 1.0;
        }

        *translation.0.x_mut() += time.delta_seconds * direction * paddle.speed;

        // TODO: bound the paddle within the walls
    }
}

