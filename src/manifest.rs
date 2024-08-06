use bevy::{app::AppExit, log::LogPlugin, prelude::*, utils::HashMap};
use leafwing_manifest::{
    asset_state::SimpleAssetState,
    identifier::Id,
    manifest::{Manifest, ManifestFormat},
    plugin::{ManifestPlugin, RegisterManifest},
};
use serde::{Deserialize, Serialize};

/// The data for as single item that might be held in the player's inventory.
///
/// All items with the same name have the same [`Item`] data:
/// a sword of slaying is always a sword of slaying, no matter how many swords the player has.
///
/// Tracking the number of items the player has is done elsewhere, in the player's inventory.
/// Per-item data, such as durability or enchantments, would also be tracked elsewhere.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)] // Properties are for demonstration purposes only.
struct Item {
    name: String,
    description: String,
    value: i32,
    weight: f32,
    max_stack: u8,
}

/// A data-driven manifest, which contains the canonical data for all the items in the game.
#[derive(Debug, Resource, Asset, TypePath, Serialize, Deserialize, PartialEq)]
struct ItemManifest {
    items: HashMap<Id<Item>, Item>,
}

impl Manifest for ItemManifest {
    // Because we're not doing any conversion between the raw and final data,
    // we can use the same type for both.
    type Item = Item;
    type RawItem = Item;
    // Similarly, we don't need to do any conversion between the raw and final data.
    type RawManifest = ItemManifest;
    // Converting between the raw and final data is trivial, so we can use `Infallible`.
    type ConversionError = std::convert::Infallible;

    // Our manifest uses a RON file under the hood.
    // Various common formats are supported out-of-the-box; check the [`ManifestFormat`] docs for more details
    // and remember to enable the corresponding feature in your `Cargo.toml`!
    const FORMAT: ManifestFormat = ManifestFormat::Ron;

    fn get(&self, id: Id<Item>) -> Option<&Self::Item> {
        self.items.get(&id)
    }

    // We're able to read the data directly from the serialized format,
    // so there's no need for any intermediate conversion.
    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        _world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        Ok(raw_manifest)
    }
}

pub fn setup(app: &mut App) {
    app
        // leafwing_manifest requires `AssetPlugin` to function
        // This is our simple state, used to navigate the asset loading process.
        .init_state::<SimpleAssetState>()
        // Coordinates asset loading and state transitions.
        .add_plugins(ManifestPlugin::<SimpleAssetState>::default())
        // Registers our item manifest, triggering it to be loaded.
        .register_manifest::<ItemManifest>("items.ron")
        .add_systems(OnEnter(SimpleAssetState::Ready), list_available_items);
}

/// This system reads the generated item manifest resource and prints out all the items.
fn list_available_items(
    item_manifest: Res<ItemManifest>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (id, item) in item_manifest.items.iter() {
        info!("{:?}: {:?}", id, item);
    }
}