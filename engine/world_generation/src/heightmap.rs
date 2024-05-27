use bevy::math::IVec2;
use bevy::prelude::{FloatExt, Vec2};
use bevy::utils::default;
use noise::{NoiseFn, SuperSimplex};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::prelude::*;

pub fn generate_heightmap(cfg: &GenerationConfig, seed: u32) -> Map {
	// let mut chunks: Vec<Chunk> = Vec::with_capacity(cfg.size.length_squared() as usize);
	let chunks = (0..cfg.size.y)
		.into_par_iter()
		.flat_map(|z| {
			(0..cfg.size.x)
				.into_par_iter()
				.map(move |x| generate_chunk(x as f64, z as f64, cfg, seed))
		})
		.collect();
	return Map {
		chunks,
		height: cfg.size.y as usize,
		width: cfg.size.x as usize,
		sea_level: cfg.sea_level as f32,
	};
}

pub fn generate_chunk(chunk_x: f64, chunk_z: f64, cfg: &GenerationConfig, seed: u32) -> Chunk {
	let mut result: [f32; Chunk::SIZE * Chunk::SIZE] = [0.; Chunk::SIZE * Chunk::SIZE];
	let mut moisture = [0.; Chunk::SIZE * Chunk::SIZE];
	let mut temp = [0.; Chunk::SIZE * Chunk::SIZE];
	let noise = SuperSimplex::new(seed);
	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let sample = sample_point(
				x as f64 + chunk_x * Chunk::SIZE as f64,
				z as f64 + chunk_z * Chunk::SIZE as f64,
				&cfg,
				&noise,
			);
			result[x + z * Chunk::SIZE] = sample;
			moisture[x + z * Chunk::SIZE] = noise.get([
				(x as f64 + chunk_x * Chunk::SIZE as f64) / &cfg.noise_scale,
				(z as f64 + chunk_z * Chunk::SIZE as f64) / &cfg.noise_scale,
			]) as f32;
			temp[x + z * Chunk::SIZE] =
				sample_tempurature(z as f32 + chunk_z as f32 * Chunk::SIZE as f32, sample, &cfg, 100.);
		}
	}
	return Chunk {
		heights: result,
		moisture: moisture,
		temperature: temp,
		chunk_offset: IVec2::new(chunk_x as i32, chunk_z as i32),
		..default()
	};
}

fn sample_tempurature(z: f32, height: f32, cfg: &GenerationConfig, equator: f32) -> f32 {
	let d = (equator - z).abs();
	let max_d = equator.max(cfg.get_total_height() as f32 - equator);
	let t_mod = d.remap(0., max_d, 0., 1.).clamp(0., 1.);

	// let max_d = d.max()
	return (height.remap(0., 50., 0., 1.).clamp(0., 1.) + t_mod) / 2.;
}

fn sample_point(x: f64, z: f64, cfg: &GenerationConfig, noise: &impl NoiseFn<f64, 2>) -> f32 {
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

	let outer = cfg.size.as_vec2() * Chunk::SIZE as f32;

	let p = Vec2::new(x as f32, z as f32);
	let d1 = p.x.min(p.y);
	let od = outer - p;
	let d2 = od.x.min(od.y);
	let d = d1.min(d2).min(cfg.border_size).remap(0., cfg.border_size, 0., 1.);

	return (elevation as f32) * d;
}

fn mask(mask: f64, value: f64, sea_level: f64) -> f64 {
	let m = (mask - sea_level).max(0.);
	return value * m;
}

fn sample_simple(x: f64, z: f64, cfg: &GeneratorLayer, noise: &impl NoiseFn<f64, 2>) -> f64 {
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
fn sample_rigid(x: f64, z: f64, cfg: &GeneratorLayer, noise: &impl NoiseFn<f64, 2>) -> f64 {
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
