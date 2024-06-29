use crate::states::MenuState;
use bevy::prelude::*;

pub struct DespawnPuglin;

#[derive(Component)]
pub struct DespawnAt(f32);

#[derive(Component)]
pub struct DespawnAfter(Timer);

#[derive(Component)]
pub struct Despawn;

impl Plugin for DespawnPuglin {
	fn build(&self, app: &mut App) {
		app.add_systems(PostUpdate, despawn_at);
		app.add_systems(
			PreUpdate,
			(despawn, despawn_after).run_if(not(in_state(MenuState::Paused))),
		);
	}
}

fn despawn_at(mut commands: Commands, time: Res<Time>, entities: Query<(Entity, &DespawnAt), Without<DespawnAfter>>) {
	for (entity, at) in entities.iter() {
		let d = at.0 - time.elapsed_seconds();
		commands
			.entity(entity)
			.insert(DespawnAfter(Timer::from_seconds(d, TimerMode::Once)));
	}
}

fn despawn_after(mut commands: Commands, mut entities: Query<(&mut DespawnAfter, Entity)>, time: Res<Time>) {
	for (mut after, entity) in &mut entities.iter_mut() {
		after.0.tick(time.delta());
		if after.0.finished() {
			commands.entity(entity).despawn();
		}
	}
}

fn despawn(mut commands: Commands, entities: Query<Entity, With<Despawn>>) {
	for entity in entities.iter() {
		commands.entity(entity).despawn();
	}
}
