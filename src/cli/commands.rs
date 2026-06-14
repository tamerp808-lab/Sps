use crate::canonical_state::CanonicalState;
use crate::cli::terminal::Terminal;
use crate::cli::commands_extended::ExtendedCommands;
use crate::sovereignty::self_awareness::SelfAwareness;
use crate::sovereignty::self_evolution::SelfEvolution;
use crate::sovereignty::governance_loop::GovernanceLoop;
use crate::sovereignty::dialogue_engine::DialogueEngine;
use crate::kernel_core::event::{Event, EventPayload};
use crate::kernel_core::event_metadata::EventSource;
use crate::kernel_core::reducer::{Reducer, KernelReducer};
use crate::storage::sqlite_store::SqliteStore;
use crate::llm::ollama_client::OllamaClient;
use crate::context_engine::context::ContextWindow;
use crate::memory::memory_manager::MemoryManager;
use crate::goal_system::goal::GoalManager;

pub struct CommandExecutor {
    state: CanonicalState,
    reducer: KernelReducer,
    last_event: Option<Event>,
    running: bool,
    initialized: bool,
    tick: u64,
    db: Option<SqliteStore>,
    llm: Option<OllamaClient>,
    context: ContextWindow,
}

impl CommandExecutor {
    pub fn new() -> Self {
        CommandExecutor {
            state: CanonicalState::initial(),
            reducer: KernelReducer,
            last_event: None,
            running: false,
            initialized: false,
            tick: 0,
            db: None,
            llm: None,
            context: ContextWindow::new(20),
        }
    }

    pub fn execute(&mut self, cmd: super::parser::Command) -> bool {
        let event_type = match &cmd {
            super::parser::Command::MemoryAdd { .. } => "MemorySemanticFactProposed",
            super::parser::Command::Plan { .. } => "PlanCreated",
            super::parser::Command::GoalCreate { .. } => "GoalCreated",
            super::parser::Command::Chat { .. } => "Chat",
            _ => "Unknown",
        };

        let result = match cmd {
            super::parser::Command::Init => {
                self.db = SqliteStore::new("sps_state.db").ok();
                self.llm = Some(OllamaClient::new("http://127.0.0.1:11434"));
                self.state = CanonicalState::initial();
                self.reducer = KernelReducer;
                self.last_event = None;
                self.initialized = true;
                self.running = false;
                self.tick = 0;
                Terminal::print_line("SPS initialized.");
                true
            }
            super::parser::Command::Model { name } => {
                if let Some(ref llm) = self.llm { llm.set_model(&name); Terminal::print_line(&format!("Model: {}", name)); }
                else { Terminal::print_line("LLM not initialized."); }
                true
            }
            super::parser::Command::Boot => {
                if !self.initialized { Terminal::print_line("Run 'init' first."); return true; }
                self.apply_event(EventPayload::Lifecycle { transition: "Boot".into(), from_state: "Offline".into(), to_state: "Running".into() });
                self.running = true;
                Terminal::print_line("Booted.");
                true
            }
            super::parser::Command::Status => {
                Terminal::print_line(&format!("Status: {} | Tick: {} | Events: {}", if self.running { "Running" } else { "Stopped" }, self.tick, self.state.kernel.event_count));
                if let Some(ref llm) = self.llm { Terminal::print_line(&format!("LLM model: {}", llm.current_model())); }
                true
            }

            // ربط جميع الأوامر الموسعة مباشرة
            super::parser::Command::MemoryAdd { content } => { ExtendedCommands::memory_add(self, &content); true }
            super::parser::Command::MemoryEpisode { description } => { ExtendedCommands::memory_episode(self, &description); true }
            super::parser::Command::MemorySearch { keyword } => { ExtendedCommands::memory_search(&self.state, &keyword); true }
            super::parser::Command::GoalCreate { description } => { ExtendedCommands::goal_create(self, &description); true }
            super::parser::Command::GoalList => { ExtendedCommands::goal_list(&self.state); true }
            super::parser::Command::Plan { goal } => { ExtendedCommands::goal_create(self, &goal); true }
            super::parser::Command::Execute => { Terminal::print_line("Execution step."); true }
            super::parser::Command::WorldEntities => { ExtendedCommands::world_entities(&self.state); true }
            super::parser::Command::WorldRelationAdd { from, relation, to } => { ExtendedCommands::world_relation_add(self, &from, &relation, &to); true }
            super::parser::Command::WorldCheck => { ExtendedCommands::world_check(self); true }
            super::parser::Command::ReasonCausal { cause, effect } => { ExtendedCommands::reason_causal(self, &cause, &effect); true }
            super::parser::Command::ReasonPattern => { ExtendedCommands::reason_pattern(&self.state); true }
            super::parser::Command::FactoryStart { project } => { ExtendedCommands::factory_start(self, &project); true }
            super::parser::Command::FactoryRequirements { description } => { ExtendedCommands::factory_requirements(self, &description); true }
            super::parser::Command::FactoryValidateCode { code } => { ExtendedCommands::factory_validate_code(&code); true }
            super::parser::Command::AutonomyGrant { agent, domain } => { ExtendedCommands::autonomy_grant(self, &agent, &domain); true }
            super::parser::Command::GovernancePolicy => { ExtendedCommands::governance_policy(self); true }
            super::parser::Command::GovernanceCheck { decision } => { ExtendedCommands::governance_check(&decision); true }
            super::parser::Command::RuntimeCheckpoint => { ExtendedCommands::runtime_checkpoint(self); true }
            super::parser::Command::RuntimeRecover { reason } => { ExtendedCommands::runtime_recover(self, &reason); true }
            super::parser::Command::RuntimeUpgrade { version } => { ExtendedCommands::runtime_upgrade(self, &version); true }
            super::parser::Command::ReplayVerify => { ExtendedCommands::replay_verify(self); true }
            super::parser::Command::InvariantCheck => { ExtendedCommands::invariant_check(self); true }
            super::parser::Command::FullVerification => { ExtendedCommands::full_verification(self); true }
            super::parser::Command::Approve { proposal_id } => { ExtendedCommands::approve_proposal(self, &proposal_id); true }
            super::parser::Command::Deny { proposal_id } => { ExtendedCommands::deny_proposal(self, &proposal_id); true }
            super::parser::Command::Reflect => { ExtendedCommands::reflect(self); true }
            super::parser::Command::Insight => { ExtendedCommands::insight(&self.state); true }
            super::parser::Command::Analyze => { ExtendedCommands::analyze(&self.state); true }
            super::parser::Command::Hash => { ExtendedCommands::show_hash(&self.state); true }
            super::parser::Command::Events => { ExtendedCommands::show_events(&self.state); true }
            super::parser::Command::Save => { self.auto_save(); true }
            super::parser::Command::Load => {
                if std::path::Path::new("sps_state.db").exists() { self.db = SqliteStore::new("sps_state.db").ok(); self.initialized = true; self.running = true; Terminal::print_line("Loaded."); }
                else { Terminal::print_line("No saved state."); }
                true
            }

            super::parser::Command::Chat { message } => {
                self.context.add("user", &message, self.tick);
                let resolved = self.context.resolve_reference(&message);
                let msg = resolved.to_lowercase();

                if msg.starts_with("تذكر") || msg.starts_with("احفظ") {
                    let content = message.splitn(2, ' ').nth(1).unwrap_or("");
                    self.apply_event(MemoryManager::propose_semantic_fact("user".into(), "said".into(), content.to_string(), 0.9));
                    Terminal::print_line(&format!("[INTENT] تذكر: {}", content));
                } else if msg.starts_with("خطط") || msg.starts_with("هدف") {
                    let goal_desc = message.splitn(2, ' ').nth(1).unwrap_or("");
                    self.apply_event(GoalManager::propose_create(format!("goal-{}", self.tick), goal_desc.to_string(), 5, vec!["UserBenefit".into()], None));
                    Terminal::print_line(&format!("[INTENT] هدف: {}", goal_desc));
                } else {
                    let response = DialogueEngine::respond(self, &resolved);
                    Terminal::print_line(&format!("SPS: {}", response));
                    self.apply_event(MemoryManager::propose_semantic_fact("user".into(), "said".into(), resolved.clone(), 0.9));
                    if response.starts_with("🌐") {
                        let clean = response.replace("🌐 ", "");
                        self.apply_event(MemoryManager::propose_semantic_fact("llm".into(), "said".into(), clean, 0.7));
                    } else {
                        self.apply_event(MemoryManager::propose_semantic_fact("sps".into(), "replied".into(), response.clone(), 0.95));
                    }
                }
                true
            }

            super::parser::Command::Shutdown => {
                if !self.running { Terminal::print_line("Not running."); return true; }
                self.apply_event(EventPayload::Lifecycle { transition: "Shutdown".into(), from_state: "Running".into(), to_state: "Offline".into() });
                self.running = false;
                Terminal::print_line("Shutdown complete.");
                true
            }
            super::parser::Command::Help => { Terminal::print_help(); true }
            super::parser::Command::Exit => { self.auto_save(); return false; }
            super::parser::Command::Unknown(input) => { Terminal::print_line(&format!("Unknown: {}. Type 'help'.", input)); true }
        };

        if self.running {
            GovernanceLoop::evaluate(self, event_type);
            SelfAwareness::tick(self);
            SelfEvolution::tick(self);
        }

        result
    }

    pub fn apply_event(&mut self, payload: EventPayload) {
        let event = if let Some(ref prev) = self.last_event {
            Event::new_after(prev, payload, EventSource::User { user_id: Some("cli".into()) }, None, None)
        } else {
            let genesis = Event::genesis();
            self.state.kernel = self.reducer.apply(&self.state.kernel, &genesis);
            self.last_event = Some(genesis.clone());
            Event::new_after(&genesis, payload, EventSource::User { user_id: Some("cli".into()) }, None, None)
        };
        self.state.kernel = self.reducer.apply(&self.state.kernel, &event);
        self.last_event = Some(event.clone());
        if let Some(ref db) = self.db {
            let _ = db.save_event(&event.id.to_string(), event.id.epoch, event.id.seq, &format!("{:?}", event.payload), &event.parent_hash.to_string(), &event.event_hash.to_string());
        }
        self.tick += 1;
    }

    fn auto_save(&self) {
        if let Some(ref db) = self.db {
            for (id, fact) in &self.state.memory.semantic {
                let _ = db.save_fact(id, &fact.subject, &fact.predicate, &fact.object, fact.confidence.0);
            }
            for (id, goal) in &self.state.goals.active_goals {
                let _ = db.conn.execute(
                    "INSERT OR REPLACE INTO goals VALUES (?1, ?2, ?3)",
                    rusqlite::params![id, goal.description, format!("{:?}", goal.status)],
                );
            }
            Terminal::print_line("[AUTO] State saved.");
        }
    }

    pub fn event_log(&self) -> Vec<Event> {
        let mut events = Vec::new();
        if self.state.kernel.event_count > 0 { events.push(Event::genesis()); if let Some(ref last) = self.last_event { events.push(last.clone()); } }
        events
    }

    pub fn is_running(&self) -> bool { self.initialized && self.running }
    pub fn tick(&self) -> u64 { self.tick }
    pub fn state(&self) -> &CanonicalState { &self.state }
    pub fn llm(&self) -> &Option<OllamaClient> { &self.llm }
}
