pub mod assets;
pub mod components;
#[cfg(debug_assertions)]
pub mod units_debug_plugin;
pub mod units_plugin;
pub mod units_spacial_set;

#[derive(Clone, Copy)]
pub enum UnitType {
	Basic,
}
