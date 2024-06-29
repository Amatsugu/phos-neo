use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildingIdentifier(u32);
