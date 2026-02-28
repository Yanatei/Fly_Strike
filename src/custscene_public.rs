use bevy::prelude::*;
use crate::config::*;
use crate::event::*;

#[derive(Resource)]
pub struct CutSceneTimers{
    pub timers: [Timer; 3],
    pub cur_index: usize,
}

impl Default for CutSceneTimers {
    fn default() -> Self {
        Self {
            timers: [
                Timer::new(BEFORE_CUTSCENE_DURATION, TimerMode::Once),
                Timer::new(IN_CUSTSCENE_DURATION, TimerMode::Once),
                Timer::new(AFTER_CUTSCENE_DURATION, TimerMode::Once),
            ], 
            cur_index: 0,
        }
    }
}

#[derive(Resource, Clone)]
pub struct CutSceneStateDef{
    pub state: [CutsceneStepState; 3],
}

impl Default for CutSceneStateDef {
    fn default() -> Self {
        Self { 
            state: [
                CutsceneStepState::BeforeCutscene,
                CutsceneStepState::InCutscene,
                CutsceneStepState::AfterCutscene,
            ],
        }
    }
}