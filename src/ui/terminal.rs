use crate::formatter::format_terminal_output;
use crate::TerraformPlan;

pub fn display_terminal_ui(plan: &TerraformPlan) {
    let output = format_terminal_output(plan);
    println!("{}", output);
}

pub fn display_progress_bar(message: &str) {
    println!("⏳ {}", message);
}

pub fn display_error(error: &str) {
    eprintln!("❌ Error: {}", error);
}

pub fn display_success(message: &str) {
    println!("✅ {}", message);
}