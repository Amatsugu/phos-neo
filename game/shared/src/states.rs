use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
	Loading,
	Startup,
	MainMenu,
	InGame,
	Paused,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplayState {
	Waiting,
	PlaceHQ,
	Playing,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssetLoadState {
	Loading,
	FinalizeAssets,
	LoadComplete,
}
