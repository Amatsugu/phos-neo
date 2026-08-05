use hex::prelude::HexCoord;

#[derive(Default)]
pub struct ValidationResult
{
	pub is_valid: bool,
	pub display: Vec<ValidationDisplay>,
}

impl ValidationResult
{
	pub fn merge_with(mut self, mut other: Self) -> Self
	{
		self.display.append(&mut other.display);
		self.is_valid = self.is_valid && other.is_valid;
		return self;
	}
}

#[derive(Clone, Copy)]
pub enum ValidationType
{
	Footprint,
	Invalid,
	Powered,
	Buffed,
}

#[derive(Clone, Copy)]
pub enum ValidationDisplay
{
	Tile
	{
		coord: HexCoord,
		validation: ValidationType,
	},
	Hex
	{
		center: HexCoord,
		radius: usize,
		validation: ValidationType,
	},
}
