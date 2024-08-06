use bevy::prelude::*;
use bevy_scriptum::prelude::*;
use bevy_scriptum::runtimes::lua::prelude::*;

pub fn setup(app: &mut App) {
  app.add_scripting::<LuaRuntime>(|runtime| {
            runtime.add_function(String::from("hello_bevy"), || {
              println!("hello bevy, called from script");
            });
       })
      .add_systems(Startup, startup)
      .add_systems(Update, call_lua_on_update_from_rust);
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