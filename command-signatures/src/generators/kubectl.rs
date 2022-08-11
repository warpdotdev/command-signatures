use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

enum KubetctlStatus {
    ConnectedToCluster,
    Other,
    GeneralError,
}

impl KubetctlStatus {
    fn from_output(output: &str) -> Self {
        if output.contains("The connection to the server") {
            KubetctlStatus::ConnectedToCluster
        } else if output.contains("error:") {
            KubetctlStatus::GeneralError
        } else {
            KubetctlStatus::Other
        }
    }
}

fn type_without_name(type_name: &str) -> String {
    format!(
        "kubectl get {} -o custom-columns=:.metadata.name",
        type_name
    )
}

fn kubectl_post_process(output: &str) -> GeneratorResults {
    match KubetctlStatus::from_output(output) {
        KubetctlStatus::ConnectedToCluster | KubetctlStatus::GeneralError => {
            GeneratorResults::default()
        }
        KubetctlStatus::Other => output
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(Suggestion::new)
            .collect_unordered_results(),
    }
}

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("kubectl")
        .add_generator(
            "resource_type",
            Generator::script("kubectl api-resources -o name", kubectl_post_process),
        )
        .add_generator(
            "running_pods",
            Generator::script(
                "kubectl get pods --field-selector=status.phase=Running -o name",
                kubectl_post_process,
            ),
        )
        .add_generator(
            "deployments",
            Generator::script(type_without_name("deployments"), kubectl_post_process),
        )
        .add_generator(
            "node",
            Generator::script(type_without_name("nodes"), kubectl_post_process),
        )
        .add_generator(
            "cluster_role",
            Generator::script(type_without_name("clusterroles"), kubectl_post_process),
        )
        .add_generator(
            "role",
            Generator::script(type_without_name("roles"), kubectl_post_process),
        )
}
