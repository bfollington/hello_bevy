use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum InputAction {
    Run,
    Jump,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_scripting::<LuaRuntime>(|runtime| {
            runtime.add_function(String::from("hello_bevy"), || {
              println!("hello bevy, called from script");
            });
       })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(InputManagerPlugin::<InputAction>::default())
        .add_systems(Startup, startup)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, jump)
        .add_systems(Update, call_lua_on_update_from_rust)
        .run();
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

fn startup(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn(Script::<LuaScript>::new(
        assets_server.load("lua/update.lua"),
    ));
}

fn call_lua_on_update_from_rust(
    mut scripted_entities: Query<(Entity, &mut LuaScriptData)>,
    scripting_runtime: ResMut<LuaRuntime>,
) {
    for (entity, mut script_data) in &mut scripted_entities {
        scripting_runtime
            .call_fn("on_update", &mut script_data, entity, ())
            .unwrap();
    }
}