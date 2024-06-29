use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
	Startup,
	MainMenu,
	Loading,
	InGame,
	Paused,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplayState {
	Waiting,
	PlaceHQ,
	Playing,
}
