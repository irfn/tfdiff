use clap::Parser;
use std::path::PathBuf;
use std::io::{self, Read, Write};
use std::fs;
use std::process::Command;
use tfdiff::{parse_terraform_output, Result, TfdiffError};
use tfdiff::formatter::{format_terminal_output, format_json_output, format_html_output, format_markdown_output};

#[derive(Parser)]
#[command(name = "tfdiff")]
#[command(about = "Beautiful Terraform plan and apply output formatter")]
#[command(version)]
struct Cli {
    /// Input file (reads from stdin if not provided)
    input: Option<PathBuf>,
    
    /// Output format
    #[arg(short, long, value_enum, default_value = "terminal")]
    format: OutputFormat,
    
    /// Filter by action type
    #[arg(short = 'F', long, value_delimiter = ',')]
    filter: Vec<String>,
    
    /// Launch web UI
    #[arg(short, long)]
    web: bool,
    
    /// Port for web UI
    #[arg(short, long, default_value = "8080")]
    port: u16,
    
    /// Watch file for changes
    #[arg(short = 'W', long)]
    watch: bool,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Quiet mode (minimal output)
    #[arg(short, long)]
    quiet: bool,
    
    /// Show summary only
    #[arg(short, long)]
    summary: bool,

    /// Generate HTML and open in browser
    #[arg(short, long)]
    browser: bool,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormat {
    Terminal,
    Json,
    Html,
    Markdown,
}

fn open_in_browser(plan: &tfdiff::TerraformPlan) -> Result<()> {
    // Generate HTML content
    let html_content = format_html_output(plan);
    
    // Create temporary file
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join(format!("tfdiff_{}.html", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()));
    
    // Write HTML to temporary file
    let mut temp_file = fs::File::create(&temp_file_path)
        .map_err(TfdiffError::IoError)?;
    temp_file.write_all(html_content.as_bytes())
        .map_err(TfdiffError::IoError)?;
    
    // Open in browser based on operating system
    let result = if cfg!(target_os = "macos") {
        Command::new("open").arg(&temp_file_path).status()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", temp_file_path.to_str().unwrap()]).status()
    } else {
        // Linux and other Unix systems
        Command::new("xdg-open").arg(&temp_file_path).status()
    };
    
    match result {
        Ok(status) if status.success() => {
            println!("🌐 Opened Terraform diff in browser: {}", temp_file_path.display());
            Ok(())
        }
        Ok(_) => {
            eprintln!("❌ Failed to open browser");
            eprintln!("📄 HTML file saved to: {}", temp_file_path.display());
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Error opening browser: {}", e);
            eprintln!("📄 HTML file saved to: {}", temp_file_path.display());
            Ok(())
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Read input from file or stdin
    let input = if let Some(path) = cli.input {
        fs::read_to_string(path)
            .map_err(TfdiffError::IoError)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)
            .map_err(TfdiffError::IoError)?;
        buffer
    };
    
    // Handle web UI mode
    if cli.web {
        eprintln!("Web UI mode on port {} (not yet implemented)", cli.port);
        return Ok(());
    }
    
    // Parse the Terraform output
    let plan = parse_terraform_output(&input)?;
    
    // Apply filters if specified
    let mut filtered_plan = plan.clone();
    if !cli.filter.is_empty() {
        // Filter resources based on action type
        filtered_plan.resources.retain(|r| {
            let action_str = match r.action {
                tfdiff::ActionType::Create => "create",
                tfdiff::ActionType::Update => "update",
                tfdiff::ActionType::Destroy => "destroy",
                tfdiff::ActionType::Read => "read",
                tfdiff::ActionType::NoOp => "noop",
            };
            cli.filter.iter().any(|f| f == action_str)
        });
    }
    
    // Format and output based on selected format
    let output = match cli.format {
        OutputFormat::Terminal => format_terminal_output(&filtered_plan),
        OutputFormat::Json => format_json_output(&filtered_plan)?,
        OutputFormat::Html => format_html_output(&filtered_plan),
        OutputFormat::Markdown => format_markdown_output(&filtered_plan),
    };
    
    // Handle browser mode
    if cli.browser {
        return open_in_browser(&filtered_plan);
    }
    
    // Print output
    if cli.summary {
        // Show only summary for terminal format
        if matches!(cli.format, OutputFormat::Terminal) {
            println!("Summary: {} to add, {} to change, {} to destroy", 
                filtered_plan.summary.add,
                filtered_plan.summary.change,
                filtered_plan.summary.destroy
            );
        } else {
            println!("{}", output);
        }
    } else {
        println!("{}", output);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}