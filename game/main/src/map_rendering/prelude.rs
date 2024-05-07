use bevy::prelude::*;
use world_generation::biome_painter::BiomePainterAsset;

#[derive(Resource)]
pub struct CurrentBiomePainter {
	pub handle: Handle<BiomePainterAsset>,
}
