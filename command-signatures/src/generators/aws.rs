use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub(super) fn parse_ec2_ids<'a>(
    output: &'a str,
    description: &'a str,
) -> impl Iterator<Item = Suggestion> + 'a {
    output
        .split_whitespace()
        .filter(|s| !s.is_empty() && *s != "None")
        .map(move |id| Suggestion::with_description(id, description))
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("aws")
        .add_generator(
            "profiles",
            Generator::script(
                CommandBuilder::single_command(
                    "cat ~/.aws/config ~/.aws/credentials 2>/dev/null | grep '^\\[' | sed 's/\\[profile //;s/\\[//;s/\\]//' | sort -u",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "AWS profile"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "ec2_instances",
            Generator::script(
                CommandBuilder::single_command(
                    "aws ec2 describe-instances --query 'Reservations[*].Instances[*].InstanceId' --output text --no-cli-pager 2>/dev/null",
                ),
                |output| parse_ec2_ids(output, "EC2 instance").collect_unordered_results(),
            ),
        )
        .add_generator(
            "ec2_running_instances",
            Generator::script(
                CommandBuilder::single_command(
                    "aws ec2 describe-instances --filters Name=instance-state-name,Values=running --query 'Reservations[*].Instances[*].InstanceId' --output text --no-cli-pager 2>/dev/null",
                ),
                |output| parse_ec2_ids(output, "Running instance").collect_unordered_results(),
            ),
        )
        .add_generator(
            "ec2_stopped_instances",
            Generator::script(
                CommandBuilder::single_command(
                    "aws ec2 describe-instances --filters Name=instance-state-name,Values=stopped --query 'Reservations[*].Instances[*].InstanceId' --output text --no-cli-pager 2>/dev/null",
                ),
                |output| parse_ec2_ids(output, "Stopped instance").collect_unordered_results(),
            ),
        )
        .add_generator(
            "ec2_security_groups",
            Generator::script(
                CommandBuilder::single_command(
                    "aws ec2 describe-security-groups --query 'SecurityGroups[*].GroupId' --output text --no-cli-pager 2>/dev/null",
                ),
                |output| parse_ec2_ids(output, "Security group").collect_unordered_results(),
            ),
        )
        .add_generator(
            "ec2_key_pairs",
            Generator::script(
                CommandBuilder::single_command(
                    "aws ec2 describe-key-pairs --query 'KeyPairs[*].KeyName' --output text --no-cli-pager 2>/dev/null",
                ),
                |output| parse_ec2_ids(output, "Key pair").collect_unordered_results(),
            ),
        )
        .add_generator(
            "ec2_vpcs",
            Generator::script(
                CommandBuilder::single_command(
                    "aws ec2 describe-vpcs --query 'Vpcs[*].VpcId' --output text --no-cli-pager 2>/dev/null",
                ),
                |output| parse_ec2_ids(output, "VPC").collect_unordered_results(),
            ),
        )
        .add_generator(
            "ec2_subnets",
            Generator::script(
                CommandBuilder::single_command(
                    "aws ec2 describe-subnets --query 'Subnets[*].SubnetId' --output text --no-cli-pager 2>/dev/null",
                ),
                |output| parse_ec2_ids(output, "Subnet").collect_unordered_results(),
            ),
        )
        .add_generator(
            "ec2_volumes",
            Generator::script(
                CommandBuilder::single_command(
                    "aws ec2 describe-volumes --query 'Volumes[*].VolumeId' --output text --no-cli-pager 2>/dev/null",
                ),
                |output| parse_ec2_ids(output, "EBS volume").collect_unordered_results(),
            ),
        )
        .add_generator(
            "ec2_snapshots",
            Generator::script(
                CommandBuilder::single_command(
                    "aws ec2 describe-snapshots --owner-ids self --query 'Snapshots[*].SnapshotId' --output text --no-cli-pager 2>/dev/null",
                ),
                |output| parse_ec2_ids(output, "EBS snapshot").collect_unordered_results(),
            ),
        )
        .add_generator(
            "ec2_images",
            Generator::script(
                CommandBuilder::single_command(
                    "aws ec2 describe-images --owners self --query 'Images[*].ImageId' --output text --no-cli-pager 2>/dev/null",
                ),
                |output| parse_ec2_ids(output, "AMI").collect_unordered_results(),
            ),
        )
}
