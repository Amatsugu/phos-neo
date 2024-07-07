use bevy::math::{IVec2, UVec2};
use bevy::prelude::{FloatExt, Vec2};
use bevy::utils::default;
use bevy::utils::petgraph::data;
use noise::{NoiseFn, SuperSimplex};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::biome_painter::{self, BiomePainter};
use crate::map::biome_map::{self, BiomeChunk, BiomeData, BiomeMap};
use crate::prelude::*;

pub fn generate_heightmap(cfg: &GenerationConfig, seed: u32, painter: &BiomePainter) -> Map {
	let biomes = &generate_biomes(cfg, seed, painter);
	// let mut chunks: Vec<Chunk> = Vec::with_capacity(cfg.size.length_squared() as usize);
	let chunks = (0..cfg.size.y)
		.into_par_iter()
		.flat_map(|z| {
			(0..cfg.size.x).into_par_iter().map(move |x| {
				let biome_chunk = &biomes.chunks[x as usize + z as usize * cfg.size.x as usize];
				return generate_chunk(x, z, cfg, seed, &biome_chunk, painter);
			})
		})
		.collect();
	return Map {
		chunks,
		height: cfg.size.y as usize,
		width: cfg.size.x as usize,
		sea_level: cfg.sea_level as f32,
	};
}

pub fn generate_biomes(cfg: &GenerationConfig, seed: u32, biome_painter: &BiomePainter) -> BiomeMap {
	let mut map = BiomeMap::new(cfg.size, biome_painter.biomes.len());
	map.chunks = (0..cfg.size.y)
		.into_par_iter()
		.flat_map(|y| {
			(0..cfg.size.x).into_par_iter().map(move |x| {
				return generate_biome_chunk(x as usize, y as usize, cfg, seed, biome_painter);
			})
		})
		.collect();
	map.blend(cfg.biome_blend);
	return map;
}

pub fn generate_biome_chunk(
	chunk_x: usize,
	chunk_y: usize,
	cfg: &GenerationConfig,
	seed: u32,
	biome_painter: &BiomePainter,
) -> BiomeChunk {
	let mut chunk = BiomeChunk {
		offset: UVec2::new(chunk_x as u32, chunk_y as u32),
		data: [BiomeData::default(); Chunk::AREA],
		tiles: Vec::with_capacity(Chunk::AREA),
	};
	let noise_m = SuperSimplex::new(seed + 1);
	let noise_t = SuperSimplex::new(seed + 2);
	let noise_c = SuperSimplex::new(seed + 3);

	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let moisture = sample_point(
				x as f64 + chunk_x as f64 * Chunk::SIZE as f64,
				z as f64 + chunk_y as f64 * Chunk::SIZE as f64,
				&cfg.moisture_noise,
				&noise_m,
				cfg.size.as_vec2(),
				cfg.border_size,
			);
			let temperature = sample_point(
				x as f64 + chunk_x as f64 * Chunk::SIZE as f64,
				z as f64 + chunk_y as f64 * Chunk::SIZE as f64,
				&cfg.temperature_noise,
				&noise_t,
				cfg.size.as_vec2(),
				cfg.border_size,
			);
			let continentality = sample_point(
				x as f64 + chunk_x as f64 * Chunk::SIZE as f64,
				z as f64 + chunk_y as f64 * Chunk::SIZE as f64,
				&cfg.continent_noise,
				&noise_c,
				cfg.size.as_vec2(),
				cfg.border_size,
			);
			let data = BiomeData {
				moisture: moisture.clamp(0., 100.),
				temperature: temperature.clamp(0., 100.),
				continentality: continentality.clamp(0., 100.),
			};
			let mut b = vec![0.; biome_painter.biomes.len()];
			b[biome_painter.sample_biome_index(&data)] = 1.;

			chunk.data[x + z * Chunk::SIZE] = data;
			chunk.tiles.push(b);
		}
	}

	return chunk;
}

pub fn generate_chunk(
	chunk_x: u32,
	chunk_z: u32,
	cfg: &GenerationConfig,
	seed: u32,
	biome_chunk: &BiomeChunk,
	biome_painter: &BiomePainter,
) -> Chunk {
	let mut result: [f32; Chunk::SIZE * Chunk::SIZE] = [0.; Chunk::AREA];
	let mut data = [BiomeData::default(); Chunk::AREA];
	let mut biome_ids = [0; Chunk::AREA];
	let noise = SuperSimplex::new(seed);
	for z in 0..Chunk::SIZE {
		for x in 0..Chunk::SIZE {
			let biome_data = biome_chunk.get_biome_data(x, z);
			let biome_blend = biome_chunk.get_biome(x, z);
			let mut sample = 0.;
			for i in 0..biome_blend.len() {
				let blend = biome_blend[i];
				if blend == 0. {
					continue;
				}
				let biome = &biome_painter.biomes[i];
				sample += sample_point(
					x as f64 + chunk_x as f64 * Chunk::SIZE as f64,
					z as f64 + chunk_z as f64 * Chunk::SIZE as f64,
					&biome.noise,
					&noise,
					cfg.size.as_vec2(),
					cfg.border_size,
				) * blend;
			}
			let idx = x + z * Chunk::SIZE;
			biome_ids[idx] = biome_chunk.get_biome_id_dithered(x, z, &noise, cfg.biome_dither);
			result[idx] = sample;
			data[idx] = biome_data.clone();
		}
	}
	return Chunk {
		heights: result,
		biome_data: data,
		biome_id: biome_ids,
		chunk_offset: IVec2::new(chunk_x as i32, chunk_z as i32),
		..default()
	};
}

fn sample_point(x: f64, z: f64, cfg: &NoiseConfig, noise: &impl NoiseFn<f64, 2>, size: Vec2, border_size: f32) -> f32 {
	let x_s = x / cfg.scale;
	let z_s = z / cfg.scale;

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
			elevation += mask(first_layer, value);
		} else {
			elevation += value;
		}
	}

	let outer = size * Chunk::SIZE as f32;

	let p = Vec2::new(x as f32, z as f32);
	let d1 = p.x.min(p.y);
	let od = outer - p;
	let d2 = od.x.min(od.y);
	let d = d1.min(d2).min(border_size).remap(0., border_size, 0., 1.);

	return (elevation as f32) * d;
}

fn mask(mask: f64, value: f64) -> f64 {
	return value * mask;
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
