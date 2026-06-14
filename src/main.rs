mod cli {
    pub mod terminal;
    pub mod parser;
    pub mod commands;
    pub mod commands_extended;
}
mod kernel_core {
    pub mod event_id;
    pub mod logical_time;
    pub mod event_hash;
    pub mod event_metadata;
    pub mod event;
    pub mod canonical_state;
    pub mod reducer;
    pub mod replay_validator;
    pub mod invariant_checker;
    pub mod constitution_checker;
    pub mod verification_suite;
    #[cfg(test)]
    pub mod replay_stress_test;
}
pub mod canonical_state;
pub mod memory;
pub mod world_model;
pub mod reasoning;
pub mod goal_system;
pub mod planner;
pub mod execution;
pub mod reflection;
pub mod self_improvement;
pub mod software_factory;
pub mod autonomy;
pub mod kernel_runtime;
pub mod governance;
pub mod sovereignty;
pub mod storage;
pub mod llm;
pub mod context_engine;

use cli::parser::Parser;
use cli::terminal::Terminal;
use cli::commands::CommandExecutor;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // إذا تم تمرير وسيط، تعامل معه كجملة واحدة واخرج
    if args.len() > 1 {
        let input = args[1..].join(" ");
        let mut executor = CommandExecutor::new();
        
        // تهيئة سريعة
        if Path::new("sps_state.db").exists() {
            executor.execute(cli::parser::Command::Init);
            executor.execute(cli::parser::Command::Load);
            executor.execute(cli::parser::Command::Boot);
        } else {
            executor.execute(cli::parser::Command::Init);
            executor.execute(cli::parser::Command::Boot);
        }
        
        // إرسال الجملة كأمر Chat
        executor.execute(cli::parser::Command::Chat { message: input });
        return;
    }

    // الوضع التفاعلي العادي (موجود بالفعل)
    Terminal::print_banner();
    let mut executor = CommandExecutor::new();

    if Path::new("sps_state.db").exists() {
        Terminal::print_line("🔄 Found saved state. Loading automatically...");
        executor.execute(cli::parser::Command::Init);
        executor.execute(cli::parser::Command::Load);
        executor.execute(cli::parser::Command::Boot);
    } else {
        Terminal::print_line("🆕 No saved state. Starting fresh...");
        executor.execute(cli::parser::Command::Init);
        executor.execute(cli::parser::Command::Boot);
    }

    loop {
        let input = Terminal::prompt();
        let cmd = Parser::parse(&input);
        if !executor.execute(cmd) {
            break;
        }
    }

    Terminal::print_line("Goodbye.");
}
