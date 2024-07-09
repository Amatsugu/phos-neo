use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeneratorState {
	Startup,
	GenerateHeightmap,
	SpawnMap,
	Idle,
	Regenerate,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssetLoadState {
	StartLoading,
	Loading,
	FinalizeAssets,
	LoadComplete,
}
