use bevy::prelude::*;
#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Clone, Copy)]
pub enum Faction {
	Player,
	Phos,
}
