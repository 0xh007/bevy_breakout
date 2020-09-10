use std::f32::consts::PI;
use std::any::type_name;
use std::collections::HashMap;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_rapier3d::na::Vector3;
use bevy_rapier3d::na::Isometry3;
use bevy_rapier3d::na::Translation3;
use bevy_rapier3d::na::UnitQuaternion;
use bevy_rapier3d::rapier::math::AngVector;
use bevy_rapier3d::physics::RigidBodyHandleComponent;
use bevy_rapier3d::physics::ColliderHandleComponent;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
use bevy_rapier3d::physics::Gravity;
use bevy_rapier3d::physics::EventQueue;
use bevy_rapier3d::render::RapierRenderPlugin;
use bevy_rapier3d::rapier::geometry::{
    ColliderBuilder,
    ContactEvent,
    BroadPhase,
    NarrowPhase,
    Proximity,
    ColliderSet,
};
use bevy_rapier3d::rapier::dynamics::*;
use bevy_rapier3d::rapier::pipeline::PhysicsPipeline;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(BodyHandleToEntity(HashMap::new()))
        .add_resource(Scoreboard { score: 0 })
        .add_resource(CurrentState { state: GameState::ArenaStart } )
        .add_resource(WindowDescriptor {
            width: 1920,
            height: 1080,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugin(RapierPhysicsPlugin)
        //.add_plugin(RapierRenderPlugin)
        //.add_startup_system(setup_debug_colliders.system())
        .add_startup_system(setup.system())
        .add_startup_system(setup_blocks.system())
        .add_system(paddle_movement_system.system())
        .add_system(body_to_entity_system.system())
        .add_system(ball_movement_system.system())
        .add_system(contact_system.system())
        .add_system(scoreboard_system.system())
        .add_system(infoboard_system.system())
        .add_resource(Gravity(Vector3::new(0.0, -3.7279, 0.0)))
        .add_default_plugins()
        .run();
}

enum Contacts {
    BallBlock(Entity, Entity),
    BallSideWall(Entity, Entity),
    BallTopWall(Entity, Entity),
    BallPaddle(Entity, Entity),
}

enum GameState {
    ArenaStart,
    Playing,
    GameOver,
}

struct CurrentState {
    state: GameState,
}

struct BodyHandleToEntity(HashMap<RigidBodyHandle, Entity>);

struct PlayerEntity(pub Entity);

struct BallEntity(pub Entity);

struct BlockEntity(pub Entity);

struct WallEntity(pub Entity);

struct TopWallEntity(pub Entity);

struct PaddleEntity(pub Entity);

struct Block {
}

struct Ball {
    velocity: Vec3,
}

struct Wall {
}

struct TopWall {
}

struct Paddle {
    speed: f32,
}

struct Scoreboard {
    score: usize,
}

struct Infoboard {
}

fn setup_blocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ) {

    for z_pos in 5..35 {
        if z_pos % 5 == 0 {
            for x_pos in -25..30 {
                if x_pos % 10 == 0 {
                    let block_entity = Entity::new();
                    commands.spawn_as_entity(
                        block_entity,
                        PbrComponents {
                            mesh: asset_server
                                .load("assets/blender/block/export/block.gltf")
                                .unwrap(),
                            material: materials.add(Color::rgb(2.3, 2.3, 0.0).into()),
                            ..Default::default()
                        },
                    )
                    .with(RigidBodyBuilder::new_dynamic().translation(x_pos as f32, 3.0, z_pos as f32))
                    .with(ColliderBuilder::cuboid(4.0, 1.0, 1.0))
                    .with(Block {});

                    commands.insert_resource(BlockEntity(block_entity));
                }
            }
        }
    }
}

fn setup_debug_colliders(
    mut commands: Commands,
) {
    // - DEBUG COLLIDER
    let debug_collide = ColliderBuilder::cuboid(1.0, 3.0, 30.0);
    let debug_body = RigidBodyBuilder::new_static().translation(0.0, 00.0, 0.0).rotation(Vector3::new(0.0, 1.57, 0.0));
    commands.spawn((debug_body, debug_collide));
    // - END DEBUG
}
    

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>, 
) {
    // - Ball -
    let ball_entity = Entity::new();
    commands.spawn_as_entity(
        ball_entity,
        PbrComponents {
            mesh: asset_server
                .load("assets/blender/ball/export/ball.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.7, 0.0, 0.0).into()),
            ..Default::default()
        },
    )
    .with(RigidBodyBuilder::new_dynamic()
        .translation(0.0, 2.5, -20.0)
        )
    .with(ColliderBuilder::ball(1.0))
    .with(Ball {
        velocity: Vec3::new(-1.0, 0.0, -1.0).normalize(),
    });
    commands.insert_resource(BallEntity(ball_entity));

    // - Paddle -
    let player_entity = Entity::new();
    commands.spawn_as_entity(
        player_entity,
        PbrComponents {
            mesh: asset_server
                .load("assets/blender/paddle/export/paddle.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.9, 0.92, 1.0).into()),
            ..Default::default()
        },
    )
    .with(RigidBodyBuilder::new_kinematic()
        .translation(0.0, 3.0, -35.0))
    .with(ColliderBuilder::cuboid(4.0, 1.0, 1.0))

    .with(Paddle {
        speed: 50.0
    });
    commands.insert_resource(PlayerEntity(player_entity));

    // - Left Wall -
    let left_wall_entity = Entity::new();
    commands.spawn_as_entity(
        left_wall_entity,
        PbrComponents {
            mesh: asset_server
                .load("assets/blender/wall/export/wall.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.0, 0.0, 2.04).into()),
            ..Default::default()
        },
    )
    .with(RigidBodyBuilder::new_static().translation(31.5, 1.0, 0.0))
    .with(ColliderBuilder::cuboid(2.0, 3.0, 40.0))
    .with(Wall {});
    commands.insert_resource(WallEntity(left_wall_entity));

    // - Right Wall -
    let right_wall_entity = Entity::new();
    commands.spawn_as_entity(
        right_wall_entity,
        PbrComponents {
            mesh: asset_server
                .load("assets/blender/wall/export/wall.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.0, 0.0, 2.04).into()),
            ..Default::default()
        },
    )
    .with(RigidBodyBuilder::new_static().translation(-31.5, 1.0, 0.0))
    .with(ColliderBuilder::cuboid(2.0, 3.0, 40.0))
    .with(Wall {});
    commands.insert_resource(WallEntity(right_wall_entity));
    
    // - Top Wall -
    let top_wall_entity = Entity::new();
    commands.spawn_as_entity(
        top_wall_entity,
        PbrComponents {
        mesh: asset_server
            .load("assets/blender/top_wall/export/top_wall.gltf")
            .unwrap(),
        material: materials.add(Color::rgb(0.0, 0.0, 2.04).into()),
        translation: Translation::new(0.0, 0.0, 0.0),
        rotation: Rotation::from_rotation_y(1.57),
        ..Default::default()
    })
    .with(RigidBodyBuilder::new_static()
        .translation(0.0, 1.0, 39.0)
        .rotation(Vector3::new(0.0, 1.57, 0.0)))
    .with(ColliderBuilder::cuboid(1.0, 3.0, 30.0));
    commands.insert_resource(TopWallEntity(top_wall_entity));

    // - Score Text
    commands.spawn(TextComponents {
        text: Text {
            font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
            value: "".to_string(),
            style: TextStyle {
                color: Color::rgb(0.2, 0.2, 0.8),
                font_size: 40.0,
            },
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(25.0),
                left: Val::Px(25.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .with(Scoreboard {score: 0});

    // - Info Text
    commands.spawn(TextComponents {
        text: Text {
            font: asset_server.load("assets/fonts/FiraMono-Medium.ttf").unwrap(),
            value: "".to_string(),
            style: TextStyle {
                color: Color::rgb(0.2, 0.2, 0.8),
                font_size: 40.0,
            },
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(75.0),
                left: Val::Px(600.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .with(Infoboard {});


    commands
        // - Board - 
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/blender/board/export/board.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.0, 0.0, 2.04).into()),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_static()
            .translation(0.0, 0.0, 0.0))
        .with(ColliderBuilder::cuboid(30.0, 2.0, 40.0))

        // - Space - 
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
            material: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            translation: Translation::new(0.0, -10.0, 0.0),
            ..Default::default()
        })

        // - Light -
        .spawn(LightComponents {
            translation: Translation::new(0.0, 10.0, -10.0),
            ..Default::default()
        })

        // - Cameras -
        // - UI
        .spawn(UiCameraComponents::default())
        // - Game View - 
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                Vec3::new(0.0, 60.0, -85.0),
                Vec3::new(0.0, 0.0, -10.0),
                Vec3::new(0.0, 1.0, 0.0),

                //Front - Side
                /*
                Vec3::new(0.0, 3.0, -75.0),
                Vec3::new(0.0, 0.0, -20.0),
                Vec3::new(0.0, 1.0, 0.0),
                */

                // Right - Side
                /*
                Vec3::new(-50.0, 2.0, -30.0),
                Vec3::new(0.0, 0.0, -30.0),
                Vec3::new(0.0, 1.0, 0.0),
                */
            )),
            ..Default::default()
        });
}

fn body_to_entity_system(
    mut h_to_e: ResMut<BodyHandleToEntity>,
    mut added: Query<(Entity, Added<RigidBodyHandleComponent>)>,
) {
    for (entity, body_handle) in &mut added.iter() {
        h_to_e.0.insert(body_handle.handle(), entity);
    }
}

fn contact_system(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut pipeline: ResMut<PhysicsPipeline>,
    mut broad_phase: ResMut<BroadPhase>,
    mut narrow_phase: ResMut<NarrowPhase>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut joints: ResMut<JointSet>,

    events: Res<EventQueue>,
    h_to_e: Res<BodyHandleToEntity>,

    balls: Query<Mut<Ball>>,
    blocks: Query<Mut<Block>>,
    side_walls: Query<Mut<Wall>>,
    top_walls: Query<Mut<TopWall>>,
    paddles: Query<Mut<Paddle>>,
    handles: Query<&RigidBodyHandleComponent>,
) {
    let mut contacts = vec![];
    while let Ok(contact_event) = events.contact_events.pop() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                let e1 = *(h_to_e.0.get(&h1).unwrap());
                let e2 = *(h_to_e.0.get(&h2).unwrap());
                
                if balls.get::<Ball>(e1).is_ok() {
                    if blocks.get::<Block>(e2).is_ok() {
                        contacts.push(Contacts::BallBlock(e1, e2));
                    }
                    else if side_walls.get::<Wall>(e2).is_ok() {
                        contacts.push(Contacts::BallSideWall(e1, e2));
                    }
                    else if top_walls.get::<TopWall>(e2).is_ok() {
                        contacts.push(Contacts::BallTopWall(e1, e2));
                    }
                    else if paddles.get::<Paddle>(e2).is_ok() {
                        contacts.push(Contacts::BallPaddle(e1, e2));
                    }

                } else if balls.get::<Ball>(e2).is_ok() {
                    if blocks.get::<Block>(e1).is_ok() {
                        contacts.push(Contacts::BallBlock(e2, e1));
                    }
                    else if side_walls.get::<Wall>(e1).is_ok() {
                        contacts.push(Contacts::BallSideWall(e2, e1));
                    }
                    else if top_walls.get::<TopWall>(e1).is_ok() {
                        contacts.push(Contacts::BallTopWall(e2, e1));
                    }
                    else if paddles.get::<Paddle>(e1).is_ok() {
                        contacts.push(Contacts::BallPaddle(e2, e1));
                    }
                }
            }
            _ => (),
        };
    }

    for contact in contacts.into_iter() {
        match contact {
            Contacts::BallBlock(e1, e2) => {
                let ball_handle = handles
                    .get::<RigidBodyHandleComponent>(e1)
                    .unwrap()
                    .handle();

                    scoreboard.score += 1;
                {
                    // Richochet the ball when it hits a block
                    let mut ball_body = bodies.get_mut(ball_handle).unwrap();
                    ball_body.linvel.z = -ball_body.linvel.z;
                }

                let block_handle = handles
                    .get::<RigidBodyHandleComponent>(e2)
                    .unwrap()
                    .handle();
                
                pipeline.remove_rigid_body(
                    block_handle,
                    &mut broad_phase,
                    &mut narrow_phase,
                    &mut bodies,
                    &mut colliders,
                    &mut joints,
                );

                commands.despawn(e2);
            },
            Contacts::BallPaddle(e1, e2) => {
                let ball_handle = handles
                    .get::<RigidBodyHandleComponent>(e1)
                    .unwrap()
                    .handle();

                let mut ball_body = bodies.get_mut(ball_handle).unwrap();
                ball_body.linvel.z = -ball_body.linvel.z;
            },
            Contacts::BallTopWall(e1, e2) => {
                let ball_handle = handles
                    .get::<RigidBodyHandleComponent>(e1)
                    .unwrap()
                    .handle();

                let mut ball_body = bodies.get_mut(ball_handle).unwrap();
                ball_body.linvel.z = -ball_body.linvel.z;
            },
            Contacts::BallSideWall(e1, e2) => {
                let ball_handle = handles
                    .get::<RigidBodyHandleComponent>(e1)
                    .unwrap()
                    .handle();

                let mut ball_body = bodies.get_mut(ball_handle).unwrap();
                ball_body.linvel.x = -ball_body.linvel.x;
            },
        }
    }
}

fn ball_restart_system(
    mut current_state: ResMut<CurrentState>,
) {
}

fn ball_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    events: Res<EventQueue>,
    ball_entity: Res<BallEntity>,
    mut current_state: ResMut<CurrentState>,
    mut bodies: ResMut<RigidBodySet>,
    mut query: Query<(&RigidBodyHandleComponent, &Ball)>,
    mut infoboard_query: Query<(&mut Text, &Infoboard)>,
) {
    let delta_seconds = f32::min(0.2, time.delta_seconds);

    if let Ok(body_handle) = query.get::<RigidBodyHandleComponent>(ball_entity.0) {
        let mut body = bodies.get_mut(body_handle.handle()).unwrap();
        let ball = query.get::<Ball>(ball_entity.0).unwrap();

        match current_state.state {
            GameState::ArenaStart => {
                if keyboard_input.pressed(KeyCode::Space) {
                    let x_impulse = -10.0; 
                    let z_impulse = -10.0; 
                    let impulse = Vector3::new(x_impulse, -10.0, z_impulse);
                    body.apply_impulse(impulse);

                    current_state.state = GameState::Playing;
                    for (mut text, _infoboard) in &mut infoboard_query.iter() {
                        text.value = format!("");
                    }
                }
            },
            GameState::Playing => {
                if body.linvel.x > 0.0 {
                    body.linvel.x = 30.0;
                } else {
                    body.linvel.x = -30.0;
                }
                if body.linvel.z > 0.0 {
                    body.linvel.z = 30.0;
                } else {
                    body.linvel.z = -30.0;
                }
                if body.linvel.y > 0.0 {
                    body.linvel.y = -30.0;
                }
            },
            _ => (),
        };
    }
}

fn paddle_movement_system(
    current_state: Res<CurrentState>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    player: Res<PlayerEntity>,
    mut bodies: ResMut<RigidBodySet>,
    mut query: Query<(&ColliderHandleComponent, &RigidBodyHandleComponent, &Paddle)>,
) {
    match current_state.state {
        GameState::ArenaStart => {
        },
        GameState::Playing => {
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

                // Kinematic Move
                let mut x_trans = body.position.translation.x + time.delta_seconds * direction * paddle.speed;
                x_trans = f32::max(-25.5, f32::min(25.5, x_trans));

                let translation = Translation3::new(x_trans, body.position.translation.y, body.position.translation.z);
                let rotation = UnitQuaternion::from_scaled_axis(Vector3::y() * PI);
                let isometry = Isometry3::from_parts(translation, rotation);

                body.set_next_kinematic_position(isometry);
            }
        },
        _ => (),
    };
}

fn infoboard_system(
    current_state: Res<CurrentState>,
    mut query: Query<(&mut Text, &Infoboard)>,
    ) {
        match current_state.state {
            GameState::ArenaStart => {
                for (mut text, _infoboard) in &mut query.iter() {
                    text.value = "Press SPACE to Start".to_string();
                }
            },
            GameState::Playing => {
                for (mut text, _infoboard) in &mut query.iter() {
                    text.value = "".to_string();
                }
            },
            _ => (),
        };
}

fn scoreboard_system(
    current_state: Res<CurrentState>,
    scoreboard: Res<Scoreboard>,
    mut query: Query<(&mut Text, &Scoreboard)>
    ) {
        match current_state.state {
            GameState::ArenaStart => {
                for (mut text, _scoreboard_component) in &mut query.iter() {
                    text.value = format!("");
                }
            },
            GameState::Playing => {
                for (mut text, _scoreboard_component) in &mut query.iter() {
                    text.value = format!("Score: {}", scoreboard.score);
                }
            },
            _ => (),
        };
}
