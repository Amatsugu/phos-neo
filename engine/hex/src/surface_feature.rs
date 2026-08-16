use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Reflect)]
pub enum SurfaceFeature
{
	#[default]
	None,
	Building
	{
		id: usize, rotation: usize
	},
	Resource
	{
		id: usize
	},
}
