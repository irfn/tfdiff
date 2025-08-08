use crate::Result;
use regex::Regex;

pub fn clean_input(input: &str) -> Result<String> {
    let mut cleaned = input.to_string();
    
    // Remove ANSI escape codes
    cleaned = clean_ansi_codes(&cleaned);
    
    // Remove spinner characters
    cleaned = clean_spinner_chars(&cleaned);
    
    // Clean CDK/CDKTF prefixes
    cleaned = clean_cdk_prefixes(&cleaned);
    
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

pub fn clean_cdk_prefixes(input: &str) -> String {
    // Remove CDK/CDKTF prefixes like "base14-cd"
    let cdk_regex = Regex::new(r"base14-cd[a-zA-Z0-9-]*").unwrap();
    cdk_regex.replace_all(input, "").to_string()
}