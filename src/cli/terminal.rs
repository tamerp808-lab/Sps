use std::io::{self, Write};

pub struct Terminal;

impl Terminal {
    pub fn prompt() -> String { print!("sps> "); io::stdout().flush().unwrap(); let mut input = String::new(); io::stdin().read_line(&mut input).unwrap(); input.trim().to_string() }
    pub fn print_line(msg: &str) { println!("{}", msg); }
    pub fn print_banner() {
        println!("╔══════════════════════════════════════╗");
        println!("║  SPS — Sovereign Processing System  ║");
        println!("╚══════════════════════════════════════╝");
    }
    pub fn print_help() {
        println!("Commands:");
        println!("  init / boot / status / shutdown / exit");
        println!("  memory add|episode|search <args>");
        println!("  goal create <desc> / goal list");
        println!("  plan <goal> / execute");
        println!("  world entities / world relate <from> <rel> <to> / world check");
        println!("  reason causal <cause> <effect> / reason pattern");
        println!("  factory start <project> / factory require <desc> / factory validate <code>");
        println!("  autonomy grant <agent> <domain>");
        println!("  governance policy / governance check <class>");
        println!("  runtime checkpoint / runtime recover <reason> / runtime upgrade <ver>");
        println!("  replay-verify / invariant-check / constitution-audit");
        println!("  reflect / insight / analyze / hash / events / save / load");
    }
}
