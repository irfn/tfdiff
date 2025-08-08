use tfdiff::parser::*;

#[cfg(test)]
mod cleaner_tests {
    use super::*;
    
    #[test]
    fn test_clean_ansi_codes() {
        let input = "\x1b[32m+ resource\x1b[0m";
        let expected = "+ resource";
        
        assert_eq!(clean_ansi_codes(input), expected);
    }
    
    #[test]
    fn test_clean_ansi_codes_complex() {
        let input = "\x1b[1m\x1b[32m+\x1b[0m\x1b[0m resource \"aws_s3_bucket\" \"test\" {\x1b[33m~\x1b[0m";
        let expected = "+ resource \"aws_s3_bucket\" \"test\" {~";
        
        assert_eq!(clean_ansi_codes(input), expected);
    }
    
    #[test]
    fn test_clean_ansi_codes_empty() {
        let input = "";
        let expected = "";
        
        assert_eq!(clean_ansi_codes(input), expected);
    }
    
    #[test]
    fn test_clean_ansi_codes_no_ansi() {
        let input = "Plan: 1 to add, 0 to change, 0 to destroy.";
        let expected = "Plan: 1 to add, 0 to change, 0 to destroy.";
        
        assert_eq!(clean_ansi_codes(input), expected);
    }
    
    #[test]
    fn test_clean_spinner_chars() {
        let input = "⠋ Initializing the backend...";
        let expected = " Initializing the backend...";
        
        assert_eq!(clean_spinner_chars(input), expected);
    }
    
    #[test]
    fn test_clean_all_spinner_chars() {
        let spinner_chars = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏";
        let input = format!("{}Loading{}", spinner_chars, spinner_chars);
        let expected = "Loading";
        
        assert_eq!(clean_spinner_chars(&input), expected);
    }
    
    #[test]
    fn test_clean_cdk_prefixes() {
        let input = "base14-cd-staging.aws_s3_bucket.example";
        let expected = ".aws_s3_bucket.example";
        
        assert_eq!(clean_cdk_prefixes(input), expected);
    }
    
    #[test]
    fn test_clean_cdk_prefixes_multiple() {
        let input = "base14-cd-prod base14-cd123 base14-cd-test-env";
        let expected = "  ";
        
        assert_eq!(clean_cdk_prefixes(input), expected);
    }
    
    #[test]
    fn test_clean_input_integration() {
        let input = "\x1b[32m⠋ base14-cd-staging.aws_s3_bucket.example\x1b[0m\r\n";
        let result = clean_input(input).unwrap();
        let expected = " .aws_s3_bucket.example\n";
        
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_clean_input_normalize_line_endings() {
        let input = "line1\r\nline2\rline3\n";
        let result = clean_input(input).unwrap();
        let expected = "line1\nline2\nline3\n";
        
        assert_eq!(result, expected);
    }
}