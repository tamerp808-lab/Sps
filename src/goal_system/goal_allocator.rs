// src/goal_system/goal_allocator.rs
// Phase 6 — Goal System
// Zone B — Cognitive

use crate::kernel_core::event::EventPayload;

pub struct GoalAllocator;

impl GoalAllocator {
    /// Allocates resources to goals based on priority.
    pub fn allocate(goals: &[(String, u64, u64)], available_resources: u64) -> Vec<(String, u64)> {
        let mut sorted = goals.to_vec();
        sorted.sort_by(|a, b| b.1.cmp(&a.1)); // by priority desc
        let mut remaining = available_resources;
        let mut allocated = Vec::new();
        for (id, _priority, requested) in sorted {
            let grant = remaining.min(requested);
            allocated.push((id.clone(), grant));
            remaining -= grant;
            if remaining == 0 { break; }
        }
        allocated
    }

    pub fn propose_allocation(allocation: Vec<(String, u64)>) -> EventPayload {
        let data = allocation.iter().map(|(id, res)| format!("{}={}", id, res)).collect::<Vec<_>>().join(",");
        EventPayload::Custom {
            event_type: "GoalResourcesAllocated".into(),
            data: data.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_to_highest_priority() {
        let goals = vec![("g1".into(), 10, 50), ("g2".into(), 20, 30)];
        let alloc = GoalAllocator::allocate(&goals, 60);
        assert_eq!(alloc[0], ("g2".into(), 30));
        assert_eq!(alloc[1], ("g1".into(), 30)); // remaining
    }
}
