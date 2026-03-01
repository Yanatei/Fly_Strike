use std::collections::HashMap;

use bevy::prelude::*;
use crate::config::*;
use crate::event::*;

#[derive(Resource)]
pub struct CutSceneTimers{
    pub timers: HashMap<CutsceneStepState,Timer>,
    pub cur_state: CutsceneStepState,
}

impl Default for CutSceneTimers {
    fn default() -> Self {
        let mut timers:HashMap<CutsceneStepState, Timer> = HashMap::new();
        
        timers.insert(CutsceneStepState::BeforeCutscene, Timer::new(BEFORE_CUTSCENE_DURATION, TimerMode::Once));
        timers.insert(CutsceneStepState::InCutscene, Timer::new(IN_CUSTSCENE_DURATION, TimerMode::Once));
        timers.insert(CutsceneStepState::AfterCutscene, Timer::new(AFTER_CUTSCENE_DURATION, TimerMode::Once));
        timers.insert(CutsceneStepState::InGameOverCutscene, Timer::new(OVER_CUTSCENE_DURATION_LIMIT, TimerMode::Once));

        Self {
            timers: timers,
            cur_state: CutsceneStepState::None,
        }
    }
}

impl CutSceneTimers {
    pub fn get_cur_timer(&mut self) -> Option<&mut Timer> {
        self.timers.get_mut(&self.cur_state)
    }
}

#[derive(Resource, Clone)]
pub struct CutSceneStateDef{
    pub state: [CutsceneStepState; 3],
    pub cur_index: usize,
}

impl Default for CutSceneStateDef {
    fn default() -> Self {
        Self { 
            state: [
                CutsceneStepState::BeforeCutscene,
                CutsceneStepState::InCutscene,
                CutsceneStepState::AfterCutscene,
            ],
            cur_index: 0,
        }
    }
}

#[derive(Resource, Clone)]
pub struct CutSceneGameOverStateDef{
    pub state: [CutsceneStepState; 3],
    pub cur_index: usize,
}

impl Default for CutSceneGameOverStateDef {
    fn default() -> Self {
        Self { 
            state: [
                CutsceneStepState::BeforeCutscene,
                CutsceneStepState::InGameOverCutscene,
                CutsceneStepState::AfterCutscene,
            ],
            cur_index: 0,
        }
    }
}