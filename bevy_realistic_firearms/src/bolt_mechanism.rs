use bevy::prelude::*;

pub struct BoltMechanismPlugin;

impl Plugin for BoltMechanismPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

pub const MAX_BOLT_POSITION: f32 = 100.0;
pub const MIN_BOLT_POSITION: f32 = 100.0;

pub struct Bolt {
    state: f32,
}
