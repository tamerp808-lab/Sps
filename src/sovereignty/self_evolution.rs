use crate::cli::terminal::Terminal;
use crate::kernel_core::event::EventPayload;
use crate::kernel_runtime::upgrade_manager::UpgradeManager;

pub struct SelfEvolution;

impl SelfEvolution {
    pub fn tick(executor: &mut crate::cli::commands::CommandExecutor) {
        if !executor.is_running() { return; }

        let memory_count = executor.state().memory.semantic.len();
        let event_count = executor.state().kernel.event_count;

        // كل 500 حدث، نقترح ترقية نظامية
        if event_count > 0 && event_count % 500 == 0 {
            let version = format!("auto-upgrade-{}", event_count / 500);
            executor.apply_event(UpgradeManager::propose_upgrade(&version));
            Terminal::print_line(&format!("[EVOLVE] Upgrade proposed: {}", version));
        }

        // مراقبة الأداء: إذا زادت الذاكرة عن 50 حقيقة، نقترح تحسينًا
        if memory_count > 50 && memory_count % 25 == 0 {
            let proposal_desc = format!("Memory compaction at {} facts", memory_count);
            executor.apply_event(EventPayload::Custom {
                event_type: "SelfImprovementProposal".into(),
                data: proposal_desc.clone().into_bytes(),
            });
            Terminal::print_line(&format!("[EVOLVE] Proposal: {}", proposal_desc));
        }
    }
}
