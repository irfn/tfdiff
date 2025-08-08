pub mod fixtures {
    use std::path::PathBuf;
    
    pub fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name)
    }
    
    pub fn load_fixture(name: &str) -> String {
        std::fs::read_to_string(fixture_path(name))
            .unwrap_or_else(|_| panic!("Failed to load fixture: {}", name))
    }
    
    pub fn load_terraform_fixture(name: &str) -> String {
        load_fixture(&format!("terraform/{}", name))
    }
    
    pub fn load_provider_fixture(provider: &str, name: &str) -> String {
        load_fixture(&format!("providers/{}/{}", provider, name))
    }
    
    pub fn load_edge_case_fixture(name: &str) -> String {
        load_fixture(&format!("edge_cases/{}", name))
    }
    
    pub fn load_malformed_fixture(name: &str) -> String {
        load_fixture(&format!("malformed/{}", name))
    }
}

pub mod assertions {
    use tfdiff::models::*;
    
    pub fn assert_resource_count(plan: &TerraformPlan, expected: usize) {
        assert_eq!(
            plan.resources.len(),
            expected,
            "Expected {} resources, found {}",
            expected,
            plan.resources.len()
        );
    }
    
    pub fn assert_summary_totals(summary: &Summary, add: usize, change: usize, destroy: usize) {
        assert_eq!(summary.add, add, "Expected {} additions, found {}", add, summary.add);
        assert_eq!(summary.change, change, "Expected {} changes, found {}", change, summary.change);
        assert_eq!(summary.destroy, destroy, "Expected {} destroys, found {}", destroy, summary.destroy);
    }
    
    pub fn assert_has_resource(plan: &TerraformPlan, resource_id: &str) {
        assert!(
            plan.resources.iter().any(|r| r.id == resource_id),
            "Expected to find resource with id: {}",
            resource_id
        );
    }
    
    pub fn assert_resource_action(plan: &TerraformPlan, resource_id: &str, expected_action: ActionType) {
        let resource = plan.resources.iter()
            .find(|r| r.id == resource_id)
            .unwrap_or_else(|| panic!("Resource {} not found", resource_id));
        
        assert_eq!(
            resource.action, expected_action,
            "Expected resource {} to have action {:?}, found {:?}",
            resource_id, expected_action, resource.action
        );
    }
}

pub mod test_data {
    use tfdiff::models::*;
    use std::collections::HashMap;
    
    pub fn sample_terraform_plan() -> TerraformPlan {
        TerraformPlan {
            mode: PlanMode::Plan,
            summary: Summary {
                add: 2,
                change: 1,
                destroy: 0,
                read: 0,
            },
            resources: vec![
                sample_create_resource(),
                sample_update_resource(),
            ],
            data_sources: Vec::new(),
            warnings: Vec::new(),
            metadata: Metadata {
                terraform_version: Some("1.5.7".to_string()),
                timestamp: Some("2024-01-01T12:00:00Z".to_string()),
                duration: Some("5s".to_string()),
            },
        }
    }
    
    pub fn sample_create_resource() -> Resource {
        let mut attributes = HashMap::new();
        attributes.insert("bucket".to_string(), serde_json::Value::String("test-bucket".to_string()));
        attributes.insert("force_destroy".to_string(), serde_json::Value::Bool(false));
        
        Resource {
            id: "aws_s3_bucket.test".to_string(),
            name: "test".to_string(),
            type_name: "aws_s3_bucket".to_string(),
            provider: "aws".to_string(),
            action: ActionType::Create,
            changes: Vec::new(),
            attributes,
            applied: false,
        }
    }
    
    pub fn sample_update_resource() -> Resource {
        let mut attributes = HashMap::new();
        attributes.insert("instance_type".to_string(), serde_json::Value::String("t3.small".to_string()));
        
        Resource {
            id: "aws_instance.web".to_string(),
            name: "web".to_string(),
            type_name: "aws_instance".to_string(),
            provider: "aws".to_string(),
            action: ActionType::Update,
            changes: vec![
                Change {
                    path: vec!["instance_type".to_string()],
                    before: Some(serde_json::Value::String("t2.micro".to_string())),
                    after: Some(serde_json::Value::String("t3.small".to_string())),
                    sensitive: false,
                    computed: false,
                }
            ],
            attributes,
            applied: false,
        }
    }
    
    pub fn sample_destroy_resource() -> Resource {
        let mut attributes = HashMap::new();
        attributes.insert("name".to_string(), serde_json::Value::String("legacy-server".to_string()));
        
        Resource {
            id: "aws_instance.legacy".to_string(),
            name: "legacy".to_string(),
            type_name: "aws_instance".to_string(),
            provider: "aws".to_string(),
            action: ActionType::Destroy,
            changes: Vec::new(),
            attributes,
            applied: false,
        }
    }
}