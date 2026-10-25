use bevy::prelude::*;

#[derive(Debug)]
pub struct SaveGame
{
	pub name: String,
	pub created_date: u64,
	pub modified_date: u64,
}
