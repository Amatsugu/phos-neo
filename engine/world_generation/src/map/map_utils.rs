use std::ops::Add;

use bevy::{math::VectorSpace, prelude::*};
use image::ImageBuffer;
use rayon::prelude::*;

use crate::{biome_painter::BiomePainter, hex_utils::HexCoord};

use super::{
	biome_map::{self, BiomeMap},
	chunk::Chunk,
	map::Map,
};

pub fn render_image(
	size: UVec2,
	data: &Vec<f32>,
	color1: LinearRgba,
	color2: LinearRgba,
) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
	let mut image = ImageBuffer::new(size.x * Chunk::SIZE as u32, size.y * Chunk::SIZE as u32);
	update_image(size, data, color1, color2, &mut image);

	return image;
}

pub fn update_image(
	size: UVec2,
	data: &Vec<f32>,
	color1: LinearRgba,
	color2: LinearRgba,
	image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>,
) {
	let min = *data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);
	let max = *data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&1.0);

	let w = size.x * Chunk::SIZE as u32;

	image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
		let idx = (y * w + x) as usize;
		let v = data[idx];
		let t = v.remap(min, max, 0.0, 1.0);
		let col = LinearRgba::lerp(&color1, color2, t);
		*pixel = to_pixel(&col);
	});
}

fn to_pixel(col: &LinearRgba) -> image::Rgba<u8> {
	return image::Rgba([
		(col.red * 255.0) as u8,
		(col.green * 255.0) as u8,
		(col.blue * 255.0) as u8,
		255,
	]);
}
pub fn render_map(map: &Map, smooth: f32) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
	let mut image = ImageBuffer::new(
		map.width as u32 * Chunk::SIZE as u32,
		map.height as u32 * Chunk::SIZE as u32,
	);
	update_map(map, smooth, &mut image);
	return image;
}
pub fn update_map(map: &Map, smooth: f32, image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
	image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
		let coord = HexCoord::from_grid_pos(x as usize, y as usize);
		let right = coord.get_neighbor(1);
		let height = map.sample_height(&coord);

		let mut color = Hsla::hsl(138.0, 1.0, 0.4);
		if height < map.sealevel {
			color.hue = 217.0;
		}

		if map.is_in_bounds(&right) {
			let h2 = map.sample_height(&right);
			color = get_height_color_blend(color, height, h2, smooth);
		}

		*pixel = to_pixel(&color.into());
	});
}

fn get_height_color_blend(base_color: Hsla, height: f32, height2: f32, smooth: f32) -> Hsla {
	let mut color = base_color;
	let mut d = height2 - height;
	if smooth == 0.0 || d.abs() > smooth {
		if d > 0.0 {
			color.lightness += 0.1;
		} else if d < 0.0 {
			color.lightness -= 0.1;
		}
	} else {
		if d.abs() <= smooth {
			d /= smooth;
			if d > 0.0 {
				let c2: LinearRgba = color.with_lightness(color.lightness + 0.1).into();
				color = LinearRgba::lerp(&color.into(), c2, d).into();
			} else {
				let c2: LinearRgba = color.with_lightness(color.lightness - 0.1).into();
				color = LinearRgba::lerp(&color.into(), c2, d.abs()).into();
			}
		}
	}

	return color;
}

pub fn render_biome_noise_map(map: &BiomeMap, multi: Vec3) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
	let mut image = ImageBuffer::new(map.width as u32, map.height as u32);
	update_biome_noise_map(map, multi, &mut image);
	return image;
}

pub fn update_biome_noise_map(map: &BiomeMap, multi: Vec3, image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
	image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
		let tile = map.get_biome_data(x as usize, y as usize);

		let color = LinearRgba::rgb(
			(tile.temperature / 100.0) * multi.x,
			(tile.continentality / 100.0) * multi.y,
			(tile.moisture / 100.0) * multi.z,
		);
		*pixel = to_pixel(&color);
	});
}

pub fn render_biome_map(map: &Map, biome_map: &BiomeMap) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
	let mut image = ImageBuffer::new(
		map.width as u32 * Chunk::SIZE as u32,
		map.height as u32 * Chunk::SIZE as u32,
	);
	update_biome_map(map, biome_map, &mut image);
	return image;
}

pub fn update_biome_map(map: &Map, biome_map: &BiomeMap, image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
	let map_biome_count = map.biome_count as f32;
	image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
		let coord = HexCoord::from_grid_pos(x as usize, y as usize);
		let biome_blend = biome_map.get_biome(x as i32, y as i32).unwrap();
		let right = coord.get_neighbor(1);
		let mut color = Oklaba::BLACK;
		for i in 0..biome_blend.len() {
			let mut c: Oklaba = Hsla::hsl((i as f32 / map_biome_count) * 360.0, 0.8, 0.7).into();
			c *= biome_blend[i];
			color = Oklaba::add(c, color.into()).into();
		}
		if map.is_in_bounds(&right) {
			let h1 = map.sample_height(&coord);
			let h2 = map.sample_height(&right);
			color = get_height_color_blend(color.into(), h1, h2, 0.5).into();
		}

		*pixel = to_pixel(&color.into());
	});
}
