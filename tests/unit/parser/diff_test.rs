use tfdiff::parser::diff::*;
use tfdiff::models::*;

#[cfg(test)]
mod diff_parser_tests {
    use super::*;
    
    #[test]
    fn test_parse_resource_changes_create() {
        let lines = vec![
            "# aws_s3_bucket.example will be created",
            "+ resource \"aws_s3_bucket\" \"example\" {",
            "+   bucket = \"test-bucket\"",
            "+   id     = (known after apply)",
            "  }",
        ];
        
        let resources = parse_resource_changes(&lines);
        assert_eq!(resources.len(), 1);
        
        let resource = &resources[0];
        assert_eq!(resource.name, "example");
        assert_eq!(resource.type_name, "aws_s3_bucket");
        assert!(matches!(resource.action, ActionType::Create));
        assert_eq!(resource.id, "aws_s3_bucket.example");
    }
    
    #[test]
    fn test_parse_resource_changes_update() {
        let lines = vec![
            "# aws_instance.web will be updated in-place",
            "~ resource \"aws_instance\" \"web\" {",
            "~   instance_type = \"t2.micro\" -> \"t3.small\"",
            "    tags          = {",
            "        \"Name\" = \"WebServer\"",
            "    }",
            "  }",
        ];
        
        let resources = parse_resource_changes(&lines);
        assert_eq!(resources.len(), 1);
        
        let resource = &resources[0];
        assert_eq!(resource.name, "web");
        assert_eq!(resource.type_name, "aws_instance");
        assert!(matches!(resource.action, ActionType::Update));
    }
    
    #[test]
    fn test_parse_resource_changes_destroy() {
        let lines = vec![
            "# aws_security_group.legacy will be destroyed",
            "- resource \"aws_security_group\" \"legacy\" {",
            "-   description = \"Legacy security group\"",
            "-   id          = \"sg-12345678\"",
            "  }",
        ];
        
        let resources = parse_resource_changes(&lines);
        assert_eq!(resources.len(), 1);
        
        let resource = &resources[0];
        assert_eq!(resource.name, "legacy");
        assert_eq!(resource.type_name, "aws_security_group");
        assert!(matches!(resource.action, ActionType::Destroy));
    }
    
    #[test]
    fn test_parse_resource_changes_multiple_resources() {
        let lines = vec![
            "# aws_s3_bucket.first will be created",
            "+ resource \"aws_s3_bucket\" \"first\" {",
            "+   bucket = \"first-bucket\"",
            "  }",
            "",
            "# aws_instance.second will be updated in-place", 
            "~ resource \"aws_instance\" \"second\" {",
            "~   instance_type = \"t2.micro\" -> \"t3.small\"",
            "  }",
        ];
        
        let resources = parse_resource_changes(&lines);
        assert_eq!(resources.len(), 2);
        
        assert_eq!(resources[0].name, "first");
        assert!(matches!(resources[0].action, ActionType::Create));
        
        assert_eq!(resources[1].name, "second");
        assert!(matches!(resources[1].action, ActionType::Update));
    }
    
    #[test]
    fn test_parse_resource_changes_no_resources() {
        let lines = vec![
            "Terraform will perform the following actions:",
            "",
            "Plan: 0 to add, 0 to change, 0 to destroy.",
        ];
        
        let resources = parse_resource_changes(&lines);
        assert_eq!(resources.len(), 0);
    }
    
    #[test]
    fn test_parse_resource_changes_malformed_lines() {
        let lines = vec![
            "Some random output",
            "# malformed resource line without proper format",
            "+ some attribute = value",
        ];
        
        let resources = parse_resource_changes(&lines);
        assert_eq!(resources.len(), 0);
    }
    
    #[test]
    fn test_parse_resource_action_line_variations() {
        let test_cases = vec![
            ("# aws_s3_bucket.example will be created", Some(ActionType::Create)),
            ("# aws_instance.server will be updated", Some(ActionType::Update)), 
            ("# aws_security_group.sg will be destroyed", Some(ActionType::Destroy)),
            ("# some invalid format", None),
            ("not a resource line", None),
        ];
        
        for (line, expected) in test_cases {
            let lines = vec![line];
            let resources = parse_resource_changes(&lines);
            
            match expected {
                Some(expected_action) => {
                    assert_eq!(resources.len(), 1);
                    assert_eq!(resources[0].action, expected_action);
                }
                None => {
                    assert_eq!(resources.len(), 0);
                }
            }
        }
    }
}