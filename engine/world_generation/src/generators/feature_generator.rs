use crate::mapping::{biome_map::BiomeMap, map::Map};

pub trait FeatureGenerator
{
	fn generate_surface_features(&self, map: &mut Map, biomes: &BiomeMap);
}
