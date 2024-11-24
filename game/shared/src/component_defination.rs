use bevy::{
	ecs::system::EntityCommands, math::{Quat, Vec3}, prelude::*
};
use serde::{Deserialize, Serialize};

use crate::prefab_defination::AnimationComponent;
#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentDefination {
	pub path: String,
	pub animations: Vec<AnimationComponent>,
}


impl ComponentDefination {
	pub fn apply(&self, commands: &mut EntityCommands){
		for c in &self.animations {
			c.apply(commands);
		}
	}
}