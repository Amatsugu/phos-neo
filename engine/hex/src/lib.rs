mod chunk;
mod hex_coord;
pub mod prelude
{
	pub use crate::chunk::*;
	pub use crate::hex_coord::*;
	pub use crate::*;
}

pub const OUTER_RADIUS: f32 = 1.;
pub const INNER_RADIUS: f32 = OUTER_RADIUS * (SQRT_3 / 2.);
pub const SHORT_DIAGONAL: f32 = 1. * SQRT_3;
pub const LONG_DIAGONAL: f32 = 2. * OUTER_RADIUS;
pub const SQRT_3: f32 = 1.7320508076;

#[cfg(test)]
mod tests;
