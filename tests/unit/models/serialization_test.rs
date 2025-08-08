use tfdiff::models::*;
use crate::common::test_data::*;
use serde_json;

#[cfg(test)]
mod models_serialization_tests {
    use super::*;
    
    #[test]
    fn test_terraform_plan_serialization() {
        let plan = sample_terraform_plan();
        let json = serde_json::to_string(&plan).unwrap();
        let deserialized: TerraformPlan = serde_json::from_str(&json).unwrap();
        
        assert_eq!(plan.summary.add, deserialized.summary.add);
        assert_eq!(plan.summary.change, deserialized.summary.change);
        assert_eq!(plan.resources.len(), deserialized.resources.len());
    }
    
    #[test]
    fn test_action_type_serialization() {
        let actions = vec![
            ActionType::Create,
            ActionType::Update,
            ActionType::Destroy,
            ActionType::Read,
            ActionType::NoOp,
        ];
        
        for action in actions {
            let json = serde_json::to_string(&action).unwrap();
            let deserialized: ActionType = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, action);
        }
    }
    
    #[test]
    fn test_plan_mode_serialization() {
        let modes = vec![PlanMode::Plan, PlanMode::Apply];
        
        for mode in modes {
            let json = serde_json::to_string(&mode).unwrap();
            let deserialized: PlanMode = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, mode);
        }
    }
    
    #[test]
    fn test_warning_level_serialization() {
        let levels = vec![
            WarningLevel::Info,
            WarningLevel::Warning,
            WarningLevel::Error,
        ];
        
        for level in levels {
            let json = serde_json::to_string(&level).unwrap();
            let deserialized: WarningLevel = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, level);
        }
    }
    
    #[test]
    fn test_summary_serialization() {
        let summary = Summary {
            add: 5,
            change: 3,
            destroy: 2,
            read: 1,
        };
        
        let json = serde_json::to_string(&summary).unwrap();
        let deserialized: Summary = serde_json::from_str(&json).unwrap();
        
        assert_eq!(summary.add, deserialized.add);
        assert_eq!(summary.change, deserialized.change);
        assert_eq!(summary.destroy, deserialized.destroy);
        assert_eq!(summary.read, deserialized.read);
    }
    
    #[test]
    fn test_resource_serialization() {
        let resource = sample_create_resource();
        let json = serde_json::to_string(&resource).unwrap();
        let deserialized: Resource = serde_json::from_str(&json).unwrap();
        
        assert_eq!(resource.id, deserialized.id);
        assert_eq!(resource.name, deserialized.name);
        assert_eq!(resource.type_name, deserialized.type_name);
        assert_eq!(resource.provider, deserialized.provider);
        assert_eq!(deserialized.action, ActionType::Create);
        assert_eq!(resource.applied, deserialized.applied);
        assert_eq!(resource.attributes.len(), deserialized.attributes.len());
    }
    
    #[test]
    fn test_change_serialization() {
        let change = Change {
            path: vec!["instance_type".to_string()],
            before: Some(serde_json::Value::String("t2.micro".to_string())),
            after: Some(serde_json::Value::String("t3.small".to_string())),
            sensitive: false,
            computed: false,
        };
        
        let json = serde_json::to_string(&change).unwrap();
        let deserialized: Change = serde_json::from_str(&json).unwrap();
        
        assert_eq!(change.path, deserialized.path);
        assert_eq!(change.before, deserialized.before);
        assert_eq!(change.after, deserialized.after);
        assert_eq!(change.sensitive, deserialized.sensitive);
        assert_eq!(change.computed, deserialized.computed);
    }
    
    #[test]
    fn test_metadata_serialization() {
        let metadata = Metadata {
            terraform_version: Some("1.5.7".to_string()),
            timestamp: Some("2024-01-01T12:00:00Z".to_string()),
            duration: Some("5s".to_string()),
        };
        
        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: Metadata = serde_json::from_str(&json).unwrap();
        
        assert_eq!(metadata.terraform_version, deserialized.terraform_version);
        assert_eq!(metadata.timestamp, deserialized.timestamp);
        assert_eq!(metadata.duration, deserialized.duration);
    }
    
    #[test]
    fn test_warning_serialization() {
        let warning = Warning {
            message: "Test warning message".to_string(),
            level: WarningLevel::Warning,
        };
        
        let json = serde_json::to_string(&warning).unwrap();
        let deserialized: Warning = serde_json::from_str(&json).unwrap();
        
        assert_eq!(warning.message, deserialized.message);
        assert_eq!(deserialized.level, WarningLevel::Warning);
    }
    
    #[test]
    fn test_data_source_serialization() {
        let mut attributes = std::collections::HashMap::new();
        attributes.insert("name".to_string(), serde_json::Value::String("test".to_string()));
        
        let data_source = DataSource {
            id: "data.aws_ami.ubuntu".to_string(),
            name: "ubuntu".to_string(),
            type_name: "aws_ami".to_string(),
            provider: "aws".to_string(),
            attributes,
        };
        
        let json = serde_json::to_string(&data_source).unwrap();
        let deserialized: DataSource = serde_json::from_str(&json).unwrap();
        
        assert_eq!(data_source.id, deserialized.id);
        assert_eq!(data_source.name, deserialized.name);
        assert_eq!(data_source.type_name, deserialized.type_name);
        assert_eq!(data_source.provider, deserialized.provider);
        assert_eq!(data_source.attributes.len(), deserialized.attributes.len());
    }
    
    #[test]
    fn test_terraform_plan_default() {
        let plan = TerraformPlan::default();
        
        assert!(matches!(plan.mode, PlanMode::Plan));
        assert_eq!(plan.summary.add, 0);
        assert_eq!(plan.summary.change, 0);
        assert_eq!(plan.summary.destroy, 0);
        assert_eq!(plan.summary.read, 0);
        assert_eq!(plan.resources.len(), 0);
        assert_eq!(plan.data_sources.len(), 0);
        assert_eq!(plan.warnings.len(), 0);
    }
}