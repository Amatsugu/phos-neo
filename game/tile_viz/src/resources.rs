use bevy::prelude::*;

#[derive(Resource, Default, Reflect)]
pub struct VisualzationMeshes
{
	pub tile_surface: Handle<Mesh>,
}

#[derive(Resource, Default, Reflect)]
pub struct TileSurfaceMaterials
{
	pub selection_material: Handle<StandardMaterial>,
	pub invalid_material: Handle<StandardMaterial>,
	pub valid_material: Handle<StandardMaterial>,
	pub powered_material: Handle<StandardMaterial>,
	pub buff_material: Handle<StandardMaterial>,
}
