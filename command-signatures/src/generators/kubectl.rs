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

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("kubectl").add_generator(
        "resource_type",
        Generator::new(
            "kubectl api-resources -o name",
            |output| match check_kubectl_post_process(output) {
                KubectlPostProcess::ConnectedToCluster | KubectlPostProcess::GeneralError => {
                    GeneratorResults::default()
                }
                KubectlPostProcess::Other => output
                    .trim()
                    .split('\n')
                    .map(|line| Suggestion::new(line.trim()))
                    .collect_unordered_results(),
            },
        ),
    )
}
