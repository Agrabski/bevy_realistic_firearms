use std::default;

use bevy::prelude::*;

pub struct FireControlPlugin;

impl Plugin for FireControlPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

pub enum FireControlSelectorPosition {
    Safe,
    Semi,
    Burst,
    FullAuto,
}

pub enum TriggerTravelLimiterPosition {
    Unlimited,
    Value(f32),
}

pub struct TriggerTravelLimiter {
    state: TriggerTravelLimiterPosition,
}

pub struct FireControlSelector {
    state: FireControlSelectorPosition,
}

pub enum BurstMechanismType {
    Stateless,
    StateFull,
}

pub struct BurstMechanism {
    mechanism_type: BurstMechanismType,
    shot_count: u8,
    burst_length: u8,
}

pub struct FireControlSystem {
    burst_mechanism: Option<BurstMechanism>,
}

#[derive(Debug, Reflect)]
pub enum TriggerType {
    SingleStage {
        threshold: f32,
    },
    TwoStage {
        first_stage_threshold: f32,
        second_stage_threshold: f32,
    },
}

impl Default for TriggerType {
    fn default() -> Self {
        Self::SingleStage { threshold: 50.0 }
    }
}

#[derive(Reflect, Debug)]
pub enum TriggerStage {
    FirstStage,
    SecondStage,
}

#[derive(Component, Default, Debug, Reflect)]
pub struct Trigger {
    pressed_value: f32,
    trigger_type: TriggerType,
}
pub const MAX_TRIGGER_PRESS: f32 = 100.0;
pub const MINIMUM_TRIGGER_PRESS: f32 = 0.0;

impl Trigger {
    pub fn set_pressed(
        &mut self,
        value: f32,
        firearm: Entity,
        event_writer: &mut EventWriter<TriggerPressChangedEvent>,
    ) {
        let new_value = value.clamp(MINIMUM_TRIGGER_PRESS, MAX_TRIGGER_PRESS);
        if new_value != value {
            warn!("Trigger press value {value} was out of bounds");
        }
        let old_value = self.pressed_value;
        self.pressed_value = new_value;
        match self.trigger_type {
            TriggerType::SingleStage { threshold } => {
                let previously_over_threshold = threshold < old_value;
                let now_over_threshold = threshold < new_value;
                match (previously_over_threshold, now_over_threshold) {
                    (true, false) => {
                        event_writer.send(TriggerPressChangedEvent {
                            firearm_id: firearm,
                            stage: None,
                        });
                    }
                    (false, true) => {
                        event_writer.send(TriggerPressChangedEvent {
                            firearm_id: firearm,
                            stage: Some(TriggerStage::FirstStage),
                        });
                    }
                    _ => {}
                }
            }
            TriggerType::TwoStage {
                first_stage_threshold,
                second_stage_threshold,
            } => todo!(),
        };
    }

    pub fn pressed_value(&self) -> f32 {
        self.pressed_value
    }
}

#[derive(Event, Debug)]
pub struct TriggerPressChangedEvent {
    firearm_id: Entity,
    stage: Option<TriggerStage>,
}

#[derive(Debug, Reflect, Default, Component)]
pub enum Interruptor {
    #[default]
    Idle,
    Triggered,
}
