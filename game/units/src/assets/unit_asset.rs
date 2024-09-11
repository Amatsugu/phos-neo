use asset_loader::create_asset_loader;
use bevy::{ecs::world::CommandQueue, prelude::*};
use serde::{Deserialize, Serialize};

use crate::components::{AirUnit, LandUnit, NavalUnit, Unit, UnitDomain};

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct UnitAsset {
	pub name: String,
	pub description: String,
	pub size: u32,
	pub prefab_path: String,
	#[serde(skip)]
	pub prefab: Handle<Scene>,
	pub unit_type: UnitType,
	pub domain: UnitDomain,
}

impl UnitAsset {
	pub fn spawn(&self, transform: Transform) -> CommandQueue {
		let mut commands = CommandQueue::default();

		let bundle = (
			PbrBundle {
				transform: transform,
				..default()
			},
			Unit,
		);
		let domain = self.domain.clone();
		commands.push(move |world: &mut World| {
			let mut e = world.spawn(bundle);
			match domain {
				UnitDomain::Land => e.insert(LandUnit),
				UnitDomain::Air => e.insert(AirUnit),
				UnitDomain::Naval => e.insert(NavalUnit),
			};
		});

		todo!();
	}
}

create_asset_loader!(
	UnitAssetPlugin,
	UnitAssetLoader,
	UnitAsset,
	&["unit", "unit.ron"],
	prefab_path -> prefab
	;?
);

#[derive(Debug, Serialize, Deserialize)]
pub enum UnitType {
	Basic,
	Turret,
}
