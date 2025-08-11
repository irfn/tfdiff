use tfdiff::parser::{parse_terraform_output, diff::parse_resource_changes};
use tfdiff::{ActionType, PlanMode};

#[test]
fn test_parse_interleaved_output() {
    let input = r#"s-aws-load-balancer-controller-role]
base14-aws-use1-eks-scout  aws_iam_role_policy_attachment.scout-use1-apps-vpc-cni-policy: Refreshing state... [id=scout-use1-apps-vpc-cni-role-20250728073308617900000
010]
                             ~ update in-place

                           Terraform will perform the following actions:
base14-aws-use1-eks-scout    # aws_eks_addon.scout-use1-apps-eks-cluster_scout-use1-apps-metrics-server_F073EB1D (scout-use1-apps-eks-cluster/scout-use1-apps-metrics-
server) will be created
                             + resource "aws_eks_addon" "scout-use1-apps-eks-cluster_scout-use1-apps-metrics-server_F073EB1D" {
                                 + addon_name           = "metrics-server"
                                 + addon_version        = "v0.8.0-eksbuild.1"
                                 + arn                  = (known after apply)
                                 + cluster_name         = "scout-use1-apps-eks"
                                 + configuration_values = (known after apply)
                                 + created_at           = (known after apply)
                                   name       = "CHS3p-zinc-backupSinkUserPA"
                                 ~ roles      = [
                                     - "scout-use1-apps-clickhouse-sa-role",
                                   # (3 unchanged attributes hidden)
                               }

                                 + cluster_name         = "scout-use1-apps-eks"
                                 + configuration_values = (known after apply)
                                 + created_at           = (known after apply)
                                 + id                   = (known after apply)
                                 + modified_at          = (known after apply)
                                 + tags_all             = (known after apply)
                               }

                             # aws_iam_policy_attachment.p-zinc-backup_CHS3p-zinc-backupSinkUserPA_5F158766 (p-zinc-backup/CHS3p-zinc-backupSinkUserPA) will be updated in-place
                             ~ resource "aws_iam_policy_attachment" "p-zinc-backup_CHS3p-zinc-backupSinkUserPA_5F158766" {
                                   id         = "CHS3p-zinc-backupSinkUserPA"
                                   name       = "CHS3p-zinc-backupSinkUserPA"
                                 ~ roles      = [
                                     - "scout-use1-apps-clickhouse-sa-role",
                                       # (1 unchanged element hidden)
                                   ]
                                   # (3 unchanged attributes hidden)
                               }

                           Plan: 1 to add, 1 to change, 0 to destroy."#;

    let result = parse_terraform_output(input);
    assert!(result.is_ok(), "Failed to parse interleaved output: {:?}", result);
    
    let plan = result.unwrap();
    assert_eq!(plan.mode, PlanMode::Plan);
    assert_eq!(plan.summary.add, 1);
    assert_eq!(plan.summary.change, 1);
    assert_eq!(plan.summary.destroy, 0);
    
    // Check that we found both resources
    assert_eq!(plan.resources.len(), 2, "Should have found 2 resources");
    
    // Check the first resource (create)
    let create_resource = plan.resources.iter()
        .find(|r| r.type_name == "aws_eks_addon")
        .expect("Should find aws_eks_addon resource");
    assert_eq!(create_resource.action, ActionType::Create);
    assert!(create_resource.name.contains("scout-use1-apps-eks-cluster_scout-use1-apps-metrics-server"));
    
    // Check the second resource (update)
    let update_resource = plan.resources.iter()
        .find(|r| r.type_name == "aws_iam_policy_attachment")
        .expect("Should find aws_iam_policy_attachment resource");
    assert_eq!(update_resource.action, ActionType::Update);
    assert!(update_resource.name.contains("p-zinc-backup_CHS3p-zinc-backupSinkUserPA"));
}

#[test]
fn test_parse_complex_resource_names() {
    let input = r#"
    # aws_eks_addon.scout-use1-apps-eks-cluster_scout-use1-apps-metrics-server_F073EB1D will be created
    + resource "aws_eks_addon" "scout-use1-apps-eks-cluster_scout-use1-apps-metrics-server_F073EB1D" {
        + addon_name = "metrics-server"
    }
    
    # aws_iam_policy_attachment.p-zinc-backup_CHS3p-zinc-backupSinkUserPA_5F158766 will be updated in-place
    ~ resource "aws_iam_policy_attachment" "p-zinc-backup_CHS3p-zinc-backupSinkUserPA_5F158766" {
        ~ roles = []
    }
    "#;
    
    let lines: Vec<&str> = input.lines().collect();
    let resources = parse_resource_changes(&lines);
    
    assert_eq!(resources.len(), 2, "Should parse both resources with complex names");
    
    // Check resource with hash suffix
    let addon = resources.iter()
        .find(|r| r.type_name == "aws_eks_addon")
        .expect("Should find aws_eks_addon");
    assert!(addon.name.contains("F073EB1D"), "Should include hash suffix in name");
    
    let attachment = resources.iter()
        .find(|r| r.type_name == "aws_iam_policy_attachment")
        .expect("Should find aws_iam_policy_attachment");
    assert!(attachment.name.contains("5F158766"), "Should include hash suffix in name");
}

#[test]
fn test_parse_with_parenthetical_info() {
    let input = r#"
    # aws_eks_addon.example (scout-use1-apps-eks-cluster/metrics-server) will be created
    + resource "aws_eks_addon" "example" {
        + addon_name = "metrics-server"
    }
    
    # aws_iam_policy_attachment.example (p-zinc-backup/CHS3p-zinc-backupSinkUserPA) will be updated
    ~ resource "aws_iam_policy_attachment" "example" {
        ~ roles = []
    }
    "#;
    
    let lines: Vec<&str> = input.lines().collect();
    let resources = parse_resource_changes(&lines);
    
    assert_eq!(resources.len(), 2, "Should parse resources with parenthetical info");
    
    let addon = resources.iter()
        .find(|r| r.type_name == "aws_eks_addon")
        .expect("Should find aws_eks_addon");
    assert_eq!(addon.action, ActionType::Create);
    
    let attachment = resources.iter()
        .find(|r| r.type_name == "aws_iam_policy_attachment")
        .expect("Should find aws_iam_policy_attachment");
    assert_eq!(attachment.action, ActionType::Update);
}