use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
	Startup,
	MainMenu,
	Loading,
	Playing,
	Paused,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplayState {
	Waiting,
	PlaceHQ,
	Playing,
}
