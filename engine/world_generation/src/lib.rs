pub mod prelude {
	use bevy::math::IVec2;

	pub struct GenerationConfig {
		pub noise_scale: f64,
		pub sea_level: f64,
		pub layers: Vec<GeneratorLayer>,
	}
	pub struct GeneratorLayer {
		pub strength: f64,
		pub min_value: f64,
		pub base_roughness: f64,
		pub roughness: f64,
		pub persistence: f64,
		pub is_rigid: bool,
		pub weight: f64,
		pub weight_multi: f64,
		pub layers: usize,
		pub first_layer_mask: bool,
	}
	pub struct Chunk {
		pub points: Vec<f32>,
		pub chunk_offset: IVec2,
	}

	impl Chunk {
		pub const SIZE: usize = 32;
	}
	pub struct Map {
		pub chunks: Vec<Chunk>,
		pub height: usize,
		pub width: usize,
	}
}

pub mod heightmap;
pub mod hex_utils;
pub mod mesh_generator;

#[cfg(test)]
mod tests {
	use super::*;

	// #[test]
	// fn it_works() {
	//     let result = add(2, 2);
	//     assert_eq!(result, 4);
	// }
}
