use bevy::prelude::*;
use interpolate::scale;
use leafwing_input_manager::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;
use avian3d::prelude::*;
use bevy_tween::prelude::*;
use bevy_tween::{
    interpolate::translation_by
};

mod scripting;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum InputAction {
    Run,
    Jump,
}

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultTweenPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(InputManagerPlugin::<InputAction>::default())
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, spawn_player)
        .add_systems(Update, jump)
        .add_systems(Startup, setup_avian)
        .add_systems(Startup, setup_tweens);

    scripting::setup(&mut app);

    app.run();
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    // Describes how to convert from player inputs into those actions
    let input_map = InputMap::new([(InputAction::Jump, KeyCode::Space)]);
    commands
        .spawn(InputManagerBundle::with_map(input_map))
        .insert(Player);
}

// Query for the `ActionState` component in your game logic systems!
fn jump(query: Query<&ActionState<InputAction>, With<Player>>) {
    let action_state = query.single();
    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(&InputAction::Jump) {
        println!("I'm jumping!");
    }
}

fn setup_tweens(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut sphere_handle = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.0)),
            material: materials.add(Color::WHITE),
            ..default()
        },
    ));

    let sphere = sphere_handle.id().into_target();
    sphere_handle.animation().insert_tween_here(
        Duration::from_secs(10),
        EaseFunction::QuadraticOut,
    sphere.with(scale(Vec3::ZERO, Vec3::ONE)));
}

fn setup_avian(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(4.0, 0.1),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(4.0, 0.1)),
            material: materials.add(Color::WHITE),
            ..default()
        },
    ));

    // Dynamic physics object with a collision shape and initial angular velocity
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
        ..default()
    });
}