use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

enum KubectlPostProcess {
    ConnectedToCluster,
    Other,
    GeneralError,
}

fn check_kubectl_post_process(output: &str) -> KubectlPostProcess {
    if output.contains("The connection to the server") {
        KubectlPostProcess::ConnectedToCluster
    } else if output.contains("error:") {
        KubectlPostProcess::GeneralError
    } else {
        KubectlPostProcess::Other
    }
}

fn type_without_name(type_name: &str) -> String {
    format!(
        "kubectl get {} -o custom-columns=:.metadata.name",
        type_name
    )
}

fn kubectl_post_process(output: &str) -> GeneratorResults {
    match check_kubectl_post_process(output) {
        KubectlPostProcess::ConnectedToCluster | KubectlPostProcess::GeneralError => {
            GeneratorResults::default()
        }
        KubectlPostProcess::Other => output
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
            Generator::new("kubectl api-resources -o name", kubectl_post_process),
        )
        .add_generator(
            "running_pods",
            Generator::new(
                "kubectl get pods --field-selector=status.phase=Running -o name",
                kubectl_post_process,
            ),
        )
        .add_generator(
            "deployments",
            Generator::new(type_without_name("deployments"), kubectl_post_process),
        )
        .add_generator(
            "node",
            Generator::new(type_without_name("nodes"), kubectl_post_process),
        )
        .add_generator(
            "cluster_role",
            Generator::new(type_without_name("clusterroles"), kubectl_post_process),
        )
        .add_generator(
            "role",
            Generator::new(type_without_name("roles"), kubectl_post_process),
        )
}
