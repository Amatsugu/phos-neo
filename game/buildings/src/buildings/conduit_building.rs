use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceConduitInfo {
	pub range: usize,
	pub connection_range: usize,
}
