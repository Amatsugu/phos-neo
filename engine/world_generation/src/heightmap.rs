mod data;
use data::*;

pub fn generate_heightmap(
	height: usize,
	width: usize,
	cfg: &GenerationConfig,
	seed: u32,
) -> Map {
	let mut chunks: Vec<Chunk> = Vec::with_capacity(height * width);
	for z in 0..height {
		for x in 0..width {
			chunks.push(generate_chunk(x as f64, z as f64, 32, cfg, seed));
		}
	}
	return Map {
		chunks: chunks,
		height: height,
		width: width,
	};
}

pub fn generate_chunk(
	chunk_x: f64,
	chunk_z: f64,
	size: usize,
	cfg: &GenerationConfig,
	seed: u32,
) -> Chunk {
	let mut result: Vec<f32> = Vec::with_capacity(size * size);
	let noise = SuperSimplex::new(seed);
	for z in 0..size {
		for x in 0..size {
			result.push(sample_point(
				x as f64 + chunk_x * size as f64,
				z as f64 + chunk_z * size as f64,
				&cfg,
				&noise,
			));
		}
	}
	return Chunk {
		points: result,
		size: size,
	};
}

fn sample_point(x: f64, z: f64, cfg: &GenerationConfig, noise: &SuperSimplex) -> f32 {
	let x_s = x / cfg.noise_scale;
	let z_s = z / cfg.noise_scale;

	let mut elevation: f64 = 0.;
	let mut first_layer: f64 = 0.;
	for i in 0..cfg.layers.len() {
		let value: f64;
		let layer = &cfg.layers[i];
		if layer.is_rigid {
			value = sample_rigid(x_s, z_s, layer, noise);
		} else {
			value = sample_simple(x_s, z_s, layer, noise);
		}
		if i == 0 {
			first_layer = value;
		}
		if layer.first_layer_mask {
			elevation += mask(first_layer, value, cfg.sea_level);
		} else {
			elevation += value;
		}
	}

	return elevation as f32;
}

fn mask(mask: f64, value: f64, sea_level: f64) -> f64 {
	let m = (mask - sea_level).max(0.);
	return value * m;
}

fn sample_simple(x: f64, z: f64, cfg: &GeneratorLayer, noise: &SuperSimplex) -> f64 {
	let mut freq: f64 = cfg.base_roughness;
	let mut amp: f64 = 1.;
	let mut value = 0.;

	for _ in 0..cfg.layers {
		let v = noise.get([x * freq, z * freq]);
		value += (v + 1.) * 0.5 * amp;
		freq *= cfg.roughness;
		amp *= cfg.persistence;
	}
	value -= cfg.min_value;
	return value * cfg.strength;
}
fn sample_rigid(x: f64, z: f64, cfg: &GeneratorLayer, noise: &SuperSimplex) -> f64 {
	let mut freq: f64 = cfg.base_roughness;
	let mut amp: f64 = 1.;
	let mut value = 0.;
	let mut weight = 1.;
	for _ in 0..cfg.layers {
		let mut v = 1. - noise.get([x * freq, z * freq]).abs();
		v *= v;
		v *= weight;
		weight = v * cfg.weight_multi;
		weight = weight.clamp(0., 1.);
		value += v * amp;
		freq *= cfg.roughness;
		amp *= cfg.persistence;
	}
	value -= cfg.min_value;
	return value * cfg.strength;
}