use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>, 
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // add entities to the world
    commands

        // - Paddle -
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/blender/paddle/export/paddle.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(0.51, 0.51, 0.51).into()),
            translation: Translation::new(0.0, 0.0, -10.0),
            ..Default::default()
        })

        // - Ball -
        .spawn(PbrComponents {
            mesh: asset_server
                .load("assets/blender/ball/export/ball.gltf")
                .unwrap(),
            material: materials.add(Color::rgb(2.3, 2.3, 0.0).into()),
            ..Default::default()
        })

        // - Board - 
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 60.0,
            })),
            material: materials.add(Color::rgb(0.0, 0.0, 1.02).into()),
            translation: Translation::new(0.0, -1.0, 0.0),
            ..Default::default()
        })

        // - Light -
        .spawn(LightComponents {
            translation: Translation::new(0.0, 10.0, 0.0),
            ..Default::default()
        })

        // - Camera
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                Vec3::new(0.0, 35.0, -35.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}

