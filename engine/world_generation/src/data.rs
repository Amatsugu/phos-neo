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
	pub size: usize,
}
pub struct Map {
	pub chunks: Vec<Chunk>,
	pub height: usize,
	pub width: usize,
}