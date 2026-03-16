use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuildUIState
{
	Init,
	Update,
	Cleanup,
}
