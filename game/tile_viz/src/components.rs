use bevy::prelude::*;
use hex::prelude::HexCoord;

#[derive(Component, Reflect, Debug)]
pub struct TileSurfaceVisualization;

#[derive(Component, Reflect, Debug)]
#[require(Transform, Visibility)]
pub struct HexAreaSelection(pub HexCoord, pub usize);

#[derive(Component, Reflect, Debug)]
#[require(Transform, Visibility)]
pub enum HexSelectionRing
{
	Tile(usize),
	Border(usize),
}

#[derive(Component, Reflect, Debug)]
pub struct RadialBorder(pub f32);
