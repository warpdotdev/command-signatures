use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn gcloud_generators() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("gcloud")
        .add_generator(
            "gcloud_projects",
            Generator::script(
                CommandBuilder::single_command(
                    "gcloud projects list --format='value(projectId)' 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "GCP project"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "gcloud_configurations",
            Generator::script(
                CommandBuilder::single_command(
                    "gcloud config configurations list --format='value(name)' 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Configuration"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "gcloud_accounts",
            Generator::script(
                CommandBuilder::single_command(
                    "gcloud auth list --format='value(account)' 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Account"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "gcloud_components",
            Generator::script(
                CommandBuilder::single_command(
                    "gcloud components list --format='value(id)' 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Component"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "gcloud_zones",
            Generator::script(
                CommandBuilder::single_command(
                    "gcloud compute zones list --format='value(name)' 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Zone"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "gcloud_regions",
            Generator::script(
                CommandBuilder::single_command(
                    "gcloud compute regions list --format='value(name)' 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Region"))
                        .collect_unordered_results()
                },
            ),
        )
}

pub fn gsutil_generators() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("gsutil")
        .add_generator(
            "gcs_buckets",
            Generator::script(
                CommandBuilder::single_command("gsutil ls 2>/dev/null"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|bucket| {
                            Suggestion::with_description(bucket.trim(), "Cloud Storage bucket")
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "gcloud_projects",
            Generator::script(
                CommandBuilder::single_command(
                    "gcloud projects list --format='value(projectId)' 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "GCP project"))
                        .collect_unordered_results()
                },
            ),
        )
}
