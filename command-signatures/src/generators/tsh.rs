use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

/// Node entry from `tsh ls --format=json` output.
#[derive(serde::Deserialize)]
struct Node {
    #[serde(default)]
    spec: NodeSpec,
    #[serde(default)]
    metadata: NodeMetadata,
}

#[derive(serde::Deserialize, Default)]
struct NodeSpec {
    #[serde(default)]
    hostname: String,
}

#[derive(serde::Deserialize, Default)]
struct NodeMetadata {
    #[serde(default)]
    expires: String,
}

/// Cluster entry from `tsh clusters --format=json` output.
#[derive(serde::Deserialize)]
struct Cluster {
    #[serde(default)]
    cluster_name: String,
}

/// Status entry from `tsh status --format json` output.
#[derive(serde::Deserialize)]
struct Status {
    #[serde(default)]
    active: Option<ActiveStatus>,
}

#[derive(serde::Deserialize)]
struct ActiveStatus {
    #[serde(default)]
    username: String,
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tsh")
        .add_generator(
            "nodes",
            Generator::script(
                CommandBuilder::single_command("tsh ls --format=json"),
                |output| {
                    let nodes: serde_json::Result<Vec<Node>> = serde_json::from_str(output);
                    match nodes {
                        Ok(nodes) => nodes
                            .into_iter()
                            .filter(|node| !node.spec.hostname.is_empty())
                            .map(|node| {
                                let description = if node.metadata.expires.is_empty() {
                                    "Teleport node".to_string()
                                } else {
                                    format!("Access expires: {}", node.metadata.expires)
                                };
                                Suggestion::with_description(node.spec.hostname, description)
                            })
                            .collect_unordered_results(),
                        Err(e) => {
                            log::error!("Couldn't parse tsh ls output: {}", e);
                            GeneratorResults::default()
                        }
                    }
                },
            ),
        )
        .add_generator(
            "clusters",
            Generator::script(
                CommandBuilder::single_command("tsh clusters --format=json"),
                |output| {
                    let clusters: serde_json::Result<Vec<Cluster>> = serde_json::from_str(output);
                    match clusters {
                        Ok(clusters) => clusters
                            .into_iter()
                            .filter(|cluster| !cluster.cluster_name.is_empty())
                            .map(|cluster| Suggestion::new(cluster.cluster_name))
                            .collect_unordered_results(),
                        Err(e) => {
                            log::error!("Couldn't parse tsh clusters output: {}", e);
                            GeneratorResults::default()
                        }
                    }
                },
            ),
        )
        .add_generator(
            "status_user",
            Generator::script(
                CommandBuilder::single_command("tsh status --format json"),
                |output| {
                    let status: serde_json::Result<Status> = serde_json::from_str(output);
                    match status {
                        Ok(status) => {
                            if let Some(active) = status.active {
                                if !active.username.is_empty() {
                                    return vec![Suggestion::new(active.username)]
                                        .into_iter()
                                        .collect_unordered_results();
                                }
                            }
                            GeneratorResults::default()
                        }
                        Err(e) => {
                            log::error!("Couldn't parse tsh status output: {}", e);
                            GeneratorResults::default()
                        }
                    }
                },
            ),
        )
}
