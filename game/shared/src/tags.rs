use bevy::prelude::*;
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub enum Faction {
	Player,
	Phos,
}
