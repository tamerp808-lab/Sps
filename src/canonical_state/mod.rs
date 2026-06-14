use serde::{Serialize, Deserialize};

pub mod world_state;
pub mod memory_state;
pub mod cognition_state;
pub mod goal_state;
pub mod planner_state;
pub mod execution_state;
pub mod reflection_state;
pub mod evolution_state;

use crate::kernel_core::canonical_state as kernel_state;
use crate::kernel_core::event::Event;
use crate::kernel_core::event_hash::EventHash;
use crate::kernel_core::reducer::{Reducer, KernelReducer};

use world_state::WorldState;
use memory_state::MemoryState;
use cognition_state::CognitionState;
use goal_state::GoalState;
use planner_state::PlannerState;
use execution_state::ExecutionState;
use reflection_state::ReflectionState;
use evolution_state::EvolutionState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalState {
    pub kernel: kernel_state::CanonicalState,
    pub world: WorldState,
    pub memory: MemoryState,
    pub cognition: CognitionState,
    pub goals: GoalState,
    pub planner: PlannerState,
    pub execution: ExecutionState,
    pub reflection: ReflectionState,
    pub evolution: EvolutionState,
}

impl CanonicalState {
    pub fn initial() -> Self {
        CanonicalState {
            kernel: kernel_state::CanonicalState::initial(),
            world: WorldState::empty(),
            memory: MemoryState::empty(),
            cognition: CognitionState::empty(),
            goals: GoalState::empty(),
            planner: PlannerState::empty(),
            execution: ExecutionState::empty(),
            reflection: ReflectionState::empty(),
            evolution: EvolutionState::empty(),
        }
    }

    pub fn compute_full_hash(&self) -> EventHash {
        kernel_state::compute_state_hash(&self.kernel)
    }
}

pub struct FullStateReducer {
    pub kernel_reducer: KernelReducer,
}

impl Reducer for FullStateReducer {
    type State = CanonicalState;
    type Event = Event;

    fn apply(&self, state: &Self::State, event: &Self::Event) -> Self::State {
        let new_kernel = self.kernel_reducer.apply(&state.kernel, event);
        CanonicalState {
            kernel: new_kernel,
            world: state.world.clone(),
            memory: state.memory.clone(),
            cognition: state.cognition.clone(),
            goals: state.goals.clone(),
            planner: state.planner.clone(),
            execution: state.execution.clone(),
            reflection: state.reflection.clone(),
            evolution: state.evolution.clone(),
        }
    }

    fn initial_state(&self) -> Self::State { CanonicalState::initial() }
    fn schema_version(&self) -> u32 { 2 }
}
