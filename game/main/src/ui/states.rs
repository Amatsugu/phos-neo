use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuildUIState
{
	Init,
	DrawMenu,
	Update,
	Cleanup,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum UICaptureState
{
	#[default]
	None,
	Cursor,
	Keyboard,
}
