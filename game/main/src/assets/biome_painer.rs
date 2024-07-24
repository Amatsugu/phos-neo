#[derive(AssetCollection, Serialize, Deserialize, Debug, TypePath, Asset, Clone)]
pub struct BiomePainterAsset {
	#[asset(path)]
	pub biomes: Vec<Handle<BiomeAsset>>,
}

impl BiomePainterAsset {
	pub fn sample_biome(&self, assets: &Assets<BiomeAsset>, data: &BiomeData) -> AssetId<BiomeAsset> {
		assert!(self.biomes.length() != 0, "There are no biomes");
		let mut biome = self.biomes.first().unwrap().id();
		let mut dist = f32::INFINITY;

		for b in &self.biomes {
			let asset = assets.get(b.id()).unwrap();
			let d = asset.distance(data.into());
			if d < dist {
				biome = b.id();
				dist = d;
			}
		}

		return biome;
	}

	pub fn build(&self, assets: &Assets<BiomeAsset>) -> BiomePainter {
		let mut biomes = Vec::with_capacity(self.biomes.len());
		for b in &self.biomes {
			let asset = assets.get(b.id()).unwrap();
			biomes.push(asset.clone());
		}
		return BiomePainter { biomes };
	}
}
