use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeneratorState
{
	Startup,
	GenerateHeightmap,
	WaitForHeightmap,
	BuildChunkMesh,
	SpawnMap,
	Idle,
	Regenerate,
	Cleanup,
}
