use crate::Result;
use regex::Regex;

pub fn clean_input(input: &str) -> Result<String> {
    let mut cleaned = input.to_string();
    
    // Remove ANSI escape codes
    cleaned = clean_ansi_codes(&cleaned);
    
    // Remove spinner characters
    cleaned = clean_spinner_chars(&cleaned);
    
    // Fix broken lines (lines that are split in the middle of resource declarations)
    cleaned = fix_broken_lines(&cleaned);
    
    // Clean CDK/CDKTF prefixes and other prefixes
    cleaned = clean_prefixes(&cleaned);
    
    // Normalize line endings
    cleaned = cleaned.replace("\r\n", "\n").replace('\r', "\n");
    
    Ok(cleaned)
}

pub fn clean_ansi_codes(input: &str) -> String {
    // More comprehensive regex to handle all ANSI escape sequences
    let ansi_regex = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]|\x1b\[m|\x1b").unwrap();
    ansi_regex.replace_all(input, "").to_string()
}

pub fn clean_spinner_chars(input: &str) -> String {
    let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let mut result = input.to_string();
    
    for &ch in &spinner_chars {
        result = result.replace(ch, "");
    }
    
    result
}

pub fn fix_broken_lines(input: &str) -> String {
    // Fix lines that are broken in the middle, particularly resource declarations
    let lines: Vec<&str> = input.lines().collect();
    let mut fixed_lines = Vec::new();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i];
        
        // Check if this line looks like it might be part of a broken resource declaration
        // If a line contains a partial resource declaration pattern (ending mid-word or mid-path)
        if line.contains("# aws_") && !line.contains(" will be ") && i + 1 < lines.len() {
            // This might be a broken line, check the next line
            let next_line = lines[i + 1];
            if next_line.contains(") will be ") {
                // Merge the lines
                let merged = format!("{}{}", line.trim_end(), next_line.trim());
                fixed_lines.push(merged);
                i += 2; // Skip the next line since we merged it
                continue;
            }
        }
        
        fixed_lines.push(line.to_string());
        i += 1;
    }
    
    fixed_lines.join("\n")
}

pub fn clean_prefixes(input: &str) -> String {
    // Clean up the lines by removing common prefixes and cleaning up formatting
    let lines: Vec<&str> = input.lines().collect();
    let mut cleaned_lines = Vec::new();
    
    for line in lines {
        let mut cleaned = line.to_string();
        
        // Remove common prefixes like "base14-aws-use1-eks-scout" or similar
        let prefix_regex = Regex::new(r"^base14-[a-zA-Z0-9-]+\s+").unwrap();
        cleaned = prefix_regex.replace(&cleaned, "").to_string();
        
        // Also handle lines that are indented with spaces but have content
        // Keep the indentation but clean the prefix
        if cleaned.trim_start() != cleaned {
            let indent = cleaned.len() - cleaned.trim_start().len();
            let spaces = " ".repeat(indent);
            let content = cleaned.trim_start();
            let content_cleaned = prefix_regex.replace(content, "").to_string();
            cleaned = format!("{}{}", spaces, content_cleaned);
        }
        
        cleaned_lines.push(cleaned);
    }
    
    cleaned_lines.join("\n")
}

// Keep the old function for backward compatibility with different behavior
pub fn clean_cdk_prefixes(input: &str) -> String {
    // Remove CDK/CDKTF prefixes like "base14-cd"
    let cdk_regex = Regex::new(r"base14-cd[a-zA-Z0-9-]*").unwrap();
    cdk_regex.replace_all(input, "").to_string()
}