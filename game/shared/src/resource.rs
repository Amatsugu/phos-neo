use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceIdentifier {
	pub id: u32,
	pub qty: u32,
}
