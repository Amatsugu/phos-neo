#[derive(Default)]
pub struct ValidationResult
{
	pub is_valid: bool,
	pub display: Vec<ValidationDisplay>,
}

impl ValidationResult
{
	pub fn merge_with(self, other: Self) -> Self
	{
		Self {
			is_valid: self.is_valid && other.is_valid,
			..Default::default()
		}
	}
}

pub enum ValidationDisplay
{
	Footprint(),
	Invalid(),
	Powered(),
	Buffed(),
}
