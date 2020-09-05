use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_rapier3d::na::Vector3;
use bevy_rapier3d::rapier::math::AngVector;
use bevy_rapier3d::physics::RigidBodyHandleComponent;
use bevy_rapier3d::physics::ColliderHandleComponent;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
use bevy_rapier3d::render::RapierRenderPlugin;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_rapier3d::rapier::dynamics::*;

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
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_physics.system())
        .add_startup_system(setup.system())
        .add_system(paddle_movement_system.system())
        .run();
}

struct Player(pub Entity);

struct Paddle {
    speed: f32,
}

fn setup_physics(mut commands: Commands) {
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>, 
) {

    // - Paddle -
    let player_entity = Entity::new();
    commands.spawn_as_entity(
        player_entity,
        PbrComponents {
            mesh: asset_server
                .load("assets/blender/paddle/export/paddle.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.51, 0.51, 0.51).into()),
            translation: Translation::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
    )
    .with(RigidBodyBuilder::new_dynamic()
        .translation(0.0, 0.0, -35.0))
    .with(ColliderBuilder::cuboid(4.0, 1.0, 1.0)
        .friction(0.5))
    .with(Paddle {
        speed: 500.0
    });
    commands.insert_resource(Player(player_entity));

    /*
    let debug_collide = ColliderBuilder::cuboid(4.0, 1.0, 1.0);
    let debug_body = RigidBodyBuilder::new_static().translation(5.0, 1.0, -35.0);
    commands.spawn((debug_body, debug_collide));
    */

    commands
        // - Ball -
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/blender/ball/export/ball.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(2.3, 2.3, 0.0).into()),
            translation: Translation::new(0.0, 0.5, -20.0),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_dynamic()
            .translation(0.0, 0.5, -20.0))
        .with(ColliderBuilder::ball(0.5))

        // - Board - 
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/blender/board/export/board.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.0, 0.0, 0.51).into()),
            translation: Translation::new(0.0, -1.0, 0.0),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_static()
            .translation(0.0, -1.0, 0.0))
        .with(ColliderBuilder::cuboid(50.0, 1.0, 50.0))

        // - Left Wall -
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/blender/wall/export/wall.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.0, 0.0, 0.51).into()),
            translation: Translation::new(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_static().translation(29.8, -2.05, 0.0))
        .with(ColliderBuilder::cuboid(3.0, 10.0, 100.0))

        // - Right Wall -
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/blender/wall/export/wall.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.0, 0.0, 0.51).into()),
            translation: Translation::new(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_static().translation(-32.8, -2.05, 0.0))
        .with(ColliderBuilder::cuboid(10., 1.0, 1.0))

        // - Top Wall -
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/blender/top_wall/export/top_wall.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.0, 0.0, 0.51).into()),
            translation: Translation::new(0.0, 0.0, 0.0),
            rotation: Rotation::from_rotation_y(1.57),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_static()
            .translation(0.0, -2.05, 40.0)
            .rotation(Vector3::new(0.0, 1.57, 0.0)))
        .with(ColliderBuilder::cuboid(1.0, 1.0, 1.0))

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
                /*
                Vec3::new(0.0, 0.0, -75.0),
                Vec3::new(0.0, 0.0, -20.0),
                Vec3::new(0.0, 1.0, 0.0),
                */
            )),
            ..Default::default()
        });
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    player: Res<Player>,
    mut bodies: ResMut<RigidBodySet>,
    mut query: Query<(&RigidBodyHandleComponent, &Paddle)>,
) {
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction -= 1.0;
    }

    if let Ok(body_handle) = query.get::<RigidBodyHandleComponent>(player.0) {
        let mut body = bodies.get_mut(body_handle.handle()).unwrap();
        let paddle = query.get::<Paddle>(player.0).unwrap();

        let x_impulse = time.delta_seconds * direction * paddle.speed;
        println!("{}", x_impulse);
        let impulse = Vector3::new(x_impulse, 0.0, 0.0);
        body.apply_impulse(impulse);
    }
}


fn paddle_movement_system_old(
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
    }
}
