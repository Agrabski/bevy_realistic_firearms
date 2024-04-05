pub mod fire_control;
pub mod bolt_mechanism;

use std::default;

use bevy::prelude::*;

pub struct RealisticFirearmsPlugin {}

impl Plugin for RealisticFirearmsPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

#[derive(Reflect, Default, Debug)]
pub struct Round {}

#[derive(Component, Default, Debug, Reflect)]
pub struct Chamber {
    content: Option<Round>,
}

