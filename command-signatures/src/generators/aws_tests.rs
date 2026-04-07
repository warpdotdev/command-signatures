use warp_completion_metadata::Suggestion;

use super::aws::parse_ec2_ids;

#[test]
fn test_parse_ec2_instance_ids() {
    let output = "i-0abcd1234efgh5678\ti-0ijkl9012mnop3456\n";
    let results: Vec<Suggestion> = parse_ec2_ids(output, "EC2 instance").collect();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].exact_string, "i-0abcd1234efgh5678");
    assert_eq!(results[0].description.as_deref(), Some("EC2 instance"));
    assert_eq!(results[1].exact_string, "i-0ijkl9012mnop3456");
    assert_eq!(results[1].description.as_deref(), Some("EC2 instance"));
}

#[test]
fn test_parse_ec2_ids_filters_none_values() {
    let output = "sg-12345678\tNone\tsg-87654321";
    let results: Vec<Suggestion> = parse_ec2_ids(output, "Security group").collect();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].exact_string, "sg-12345678");
    assert_eq!(results[1].exact_string, "sg-87654321");
}

#[test]
fn test_parse_ec2_ids_empty_output() {
    let results: Vec<Suggestion> = parse_ec2_ids("", "VPC").collect();
    assert!(results.is_empty());
}

#[test]
fn test_parse_ec2_ids_whitespace_only() {
    let results: Vec<Suggestion> = parse_ec2_ids("  \n\t  \n", "Subnet").collect();
    assert!(results.is_empty());
}

#[test]
fn test_parse_ec2_ids_single_id() {
    let output = "vol-0abc1234def56789\n";
    let results: Vec<Suggestion> = parse_ec2_ids(output, "EBS volume").collect();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].exact_string, "vol-0abc1234def56789");
    assert_eq!(results[0].description.as_deref(), Some("EBS volume"));
}

#[test]
fn test_parse_ec2_ids_mixed_whitespace_separators() {
    let output = "snap-111\tsnap-222  snap-333\nsnap-444";
    let results: Vec<Suggestion> = parse_ec2_ids(output, "EBS snapshot").collect();
    assert_eq!(results.len(), 4);
    let ids: Vec<&str> = results.iter().map(|s| s.exact_string.as_str()).collect();
    assert_eq!(ids, vec!["snap-111", "snap-222", "snap-333", "snap-444"]);
}

#[test]
fn test_parse_ec2_ids_all_none() {
    let output = "None\tNone\nNone";
    let results: Vec<Suggestion> = parse_ec2_ids(output, "AMI").collect();
    assert!(results.is_empty());
}

#[test]
fn test_parse_ec2_ids_preserves_description() {
    let output = "subnet-abc123";
    let results: Vec<Suggestion> = parse_ec2_ids(output, "Subnet").collect();
    assert_eq!(results[0].description.as_deref(), Some("Subnet"));

    let results: Vec<Suggestion> = parse_ec2_ids(output, "Key pair").collect();
    assert_eq!(results[0].description.as_deref(), Some("Key pair"));
}
