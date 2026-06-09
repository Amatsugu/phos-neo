use bevy::prelude::*;

#[derive(Debug, EntityEvent)]
#[entity_event(propagate)]
pub struct Hover(pub Entity);

#[derive(Debug, EntityEvent)]
#[entity_event(propagate)]
pub struct Press(pub Entity);

#[derive(Debug, EntityEvent)]
#[entity_event(propagate)]
pub struct Blur(pub Entity);
