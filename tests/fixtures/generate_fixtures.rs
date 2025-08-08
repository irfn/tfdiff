use std::fs;
use std::path::Path;

pub fn generate_large_plan_fixture(resource_count: usize) -> String {
    let mut content = String::new();
    
    content.push_str("Terraform used the selected providers to generate the following execution plan. Resource actions are indicated with the following symbols:\n");
    content.push_str("  + create\n");
    content.push_str("  ~ update in-place\n");
    content.push_str("  - destroy\n\n");
    content.push_str("Terraform will perform the following actions:\n\n");
    
    for i in 0..resource_count {
        content.push_str(&format!(
            "  # aws_instance.server_{} will be created\n", i
        ));
        content.push_str(&format!(
            "  + resource \"aws_instance\" \"server_{}\" {{\n", i
        ));
        content.push_str("      + ami                     = \"ami-12345678\"\n");
        content.push_str("      + arn                     = (known after apply)\n");
        content.push_str("      + associate_public_ip_address = (known after apply)\n");
        content.push_str("      + availability_zone       = (known after apply)\n");
        content.push_str("      + cpu_core_count          = (known after apply)\n");
        content.push_str("      + cpu_threads_per_core    = (known after apply)\n");
        content.push_str("      + disable_api_termination = (known after apply)\n");
        content.push_str("      + ebs_optimized          = (known after apply)\n");
        content.push_str("      + get_password_data      = false\n");
        content.push_str("      + host_id                = (known after apply)\n");
        content.push_str("      + id                     = (known after apply)\n");
        content.push_str("      + instance_initiated_shutdown_behavior = (known after apply)\n");
        content.push_str("      + instance_state         = (known after apply)\n");
        content.push_str("      + instance_type          = \"t3.micro\"\n");
        content.push_str("      + ipv6_address_count     = (known after apply)\n");
        content.push_str("      + ipv6_addresses         = (known after apply)\n");
        content.push_str(&format!("      + key_name               = \"server-key-{}\"\n", i));
        content.push_str("      + monitoring             = (known after apply)\n");
        content.push_str("      + outpost_arn            = (known after apply)\n");
        content.push_str("      + password_data          = (known after apply)\n");
        content.push_str("      + placement_group        = (known after apply)\n");
        content.push_str("      + placement_partition_number = (known after apply)\n");
        content.push_str("      + primary_network_interface_id = (known after apply)\n");
        content.push_str("      + private_dns_name       = (known after apply)\n");
        content.push_str("      + private_ip             = (known after apply)\n");
        content.push_str("      + public_dns             = (known after apply)\n");
        content.push_str("      + public_ip              = (known after apply)\n");
        content.push_str("      + secondary_private_ips  = (known after apply)\n");
        content.push_str("      + security_groups        = (known after apply)\n");
        content.push_str("      + source_dest_check      = true\n");
        content.push_str("      + subnet_id              = (known after apply)\n");
        content.push_str("      + tags                   = {\n");
        content.push_str(&format!("          + \"Name\" = \"server-{}\"\n", i));
        content.push_str("          + \"Environment\" = \"test\"\n");
        content.push_str(&format!("          + \"Index\" = \"{}\"\n", i));
        content.push_str("        }\n");
        content.push_str("      + tags_all               = (known after apply)\n");
        content.push_str("      + tenancy                = (known after apply)\n");
        content.push_str("      + user_data              = (known after apply)\n");
        content.push_str("      + user_data_base64       = (known after apply)\n");
        content.push_str("      + user_data_replace_on_change = false\n");
        content.push_str("      + vpc_security_group_ids = (known after apply)\n");
        content.push_str("    }\n\n");
    }
    
    content.push_str(&format!("Plan: {} to add, 0 to change, 0 to destroy.\n", resource_count));
    
    content
}

pub fn generate_json_plan_fixture(resource_count: usize) -> String {
    let mut resources = Vec::new();
    let mut resource_changes = Vec::new();
    
    for i in 0..resource_count {
        let resource_json = format!(r#"
        {{
            "address": "aws_instance.server_{}",
            "mode": "managed",
            "type": "aws_instance",
            "name": "server_{}",
            "provider_name": "registry.terraform.io/hashicorp/aws",
            "schema_version": 1,
            "values": {{
                "ami": "ami-12345678",
                "instance_type": "t3.micro",
                "key_name": "server-key-{}",
                "tags": {{
                    "Name": "server-{}",
                    "Environment": "test",
                    "Index": "{}"
                }}
            }}
        }}"#, i, i, i, i, i);
        
        let change_json = format!(r#"
        {{
            "address": "aws_instance.server_{}",
            "mode": "managed",
            "type": "aws_instance",
            "name": "server_{}",
            "provider_name": "registry.terraform.io/hashicorp/aws",
            "change": {{
                "actions": ["create"],
                "before": null,
                "after": {{
                    "ami": "ami-12345678",
                    "instance_type": "t3.micro",
                    "key_name": "server-key-{}",
                    "tags": {{
                        "Name": "server-{}",
                        "Environment": "test",
                        "Index": "{}"
                    }}
                }},
                "after_unknown": {{
                    "arn": true,
                    "id": true,
                    "private_ip": true,
                    "public_ip": true
                }}
            }}
        }}"#, i, i, i, i, i);
        
        resources.push(resource_json);
        resource_changes.push(change_json);
    }
    
    format!(r#"{{
  "format_version": "1.2",
  "terraform_version": "1.5.7",
  "planned_values": {{
    "root_module": {{
      "resources": [{}]
    }}
  }},
  "resource_changes": [{}],
  "configuration": {{
    "provider_config": {{
      "aws": {{
        "name": "aws",
        "full_name": "registry.terraform.io/hashicorp/aws",
        "version_constraint": "~> 5.0",
        "expressions": {{
          "region": {{
            "constant_value": "us-east-1"
          }}
        }}
      }}
    }}
  }}
}}"#, resources.join(","), resource_changes.join(","))
}

pub fn save_generated_fixtures() -> std::io::Result<()> {
    let sizes = vec![10, 100, 500, 1000];
    
    for size in sizes {
        // Generate text fixtures
        let text_fixture = generate_large_plan_fixture(size);
        let text_path = format!("tests/fixtures/generated/plan_{}_resources.txt", size);
        fs::create_dir_all(Path::new(&text_path).parent().unwrap())?;
        fs::write(&text_path, text_fixture)?;
        
        // Generate JSON fixtures  
        let json_fixture = generate_json_plan_fixture(size);
        let json_path = format!("tests/fixtures/generated/plan_{}_resources.json", size);
        fs::write(&json_path, json_fixture)?;
        
        println!("Generated fixtures for {} resources", size);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_small_fixture() {
        let fixture = generate_large_plan_fixture(2);
        assert!(fixture.contains("aws_instance.server_0"));
        assert!(fixture.contains("aws_instance.server_1"));
        assert!(fixture.contains("Plan: 2 to add, 0 to change, 0 to destroy"));
    }
    
    #[test] 
    fn test_generate_json_fixture() {
        let fixture = generate_json_plan_fixture(1);
        assert!(fixture.contains("aws_instance.server_0"));
        assert!(fixture.contains("\"format_version\": \"1.2\""));
    }
}