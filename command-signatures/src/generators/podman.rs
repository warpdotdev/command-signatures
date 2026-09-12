use serde::Deserialize;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, IconType, Suggestion,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PodmanContainerOutput {
    names: Vec<String>,
    image: String,
}

pub(crate) fn post_process_podman_ps(output: &str) -> GeneratorResults {
    output
        .trim()
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let container: PodmanContainerOutput = serde_json::from_str(line).ok()?;
            let name = container.names.first()?.clone();
            Some(
                Suggestion::with_description(name.clone(), "Container")
                    .with_display_name(Some(format!("{} ({})", name, container.image)))
                    .with_icon(IconType::DockerContainer),
            )
        })
        .collect_unordered_results()
}

/// `podman images --format '{{ json . }}'` mixes casing: `repository`/`tag` are
/// lower-cased while most other fields (e.g. `Id`, `Size`) are PascalCase.
#[derive(Debug, Deserialize)]
struct PodmanImageOutput {
    #[serde(default)]
    repository: Option<String>,
    #[serde(default)]
    tag: Option<String>,
}

pub(crate) fn post_process_podman_images(output: &str) -> GeneratorResults {
    output
        .trim()
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let image: PodmanImageOutput = serde_json::from_str(line).ok()?;
            let repository = image.repository?;
            if repository == "<none>" {
                return None;
            }
            let tag = image.tag.filter(|tag| tag != "<none>");
            Some(match tag {
                Some(tag) => Suggestion::with_description(repository, tag),
                None => Suggestion::new(repository),
            })
            .map(|suggestion| suggestion.with_icon(IconType::DockerImage))
        })
        .collect_unordered_results()
}

/// Parses the pipe-separated output of `podman pod ps --format '{{.Name}}|{{.Status}}'`.
pub(crate) fn post_process_podman_pods(output: &str) -> GeneratorResults {
    output
        .trim()
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut parts = line.splitn(2, '|');
            let name = parts.next()?.trim();
            if name.is_empty() {
                return None;
            }
            let status = parts.next().unwrap_or("").trim();
            Some(Suggestion::with_description(name, status))
        })
        .collect_unordered_results()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PodmanVolumeOutput {
    name: String,
    driver: String,
}

pub(crate) fn post_process_podman_volumes(output: &str) -> GeneratorResults {
    output
        .trim()
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let volume: PodmanVolumeOutput = serde_json::from_str(line).ok()?;
            Some(Suggestion::with_description(volume.name, volume.driver))
        })
        .collect_unordered_results()
}

/// `podman network ls --format '{{ json . }}'` uses lower snake_case field names.
#[derive(Debug, Deserialize)]
struct PodmanNetworkOutput {
    name: String,
    driver: String,
}

pub(crate) fn post_process_podman_networks(output: &str) -> GeneratorResults {
    output
        .trim()
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let network: PodmanNetworkOutput = serde_json::from_str(line).ok()?;
            Some(Suggestion::with_description(network.name, network.driver))
        })
        .collect_unordered_results()
}

#[derive(Debug, Deserialize)]
struct PodmanSecretOutput {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Driver")]
    driver: String,
}

pub(crate) fn post_process_podman_secrets(output: &str) -> GeneratorResults {
    output
        .trim()
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let secret: PodmanSecretOutput = serde_json::from_str(line).ok()?;
            Some(Suggestion::with_description(secret.name, secret.driver))
        })
        .collect_unordered_results()
}

/// `podman machine list --format json` prints a single JSON array (unlike the
/// newline-delimited JSON objects most other `podman ... ls` commands print).
#[derive(Debug, Deserialize)]
struct PodmanMachineOutput {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "VMType")]
    vm_type: String,
    #[serde(rename = "Running")]
    running: bool,
}

pub(crate) fn post_process_podman_machines(output: &str) -> GeneratorResults {
    let trimmed = output.trim();
    if trimmed.is_empty() {
        return GeneratorResults::default();
    }

    let machines: Vec<PodmanMachineOutput> = match serde_json::from_str(trimmed) {
        Ok(machines) => machines,
        Err(err) => {
            log::warn!(
                "Unable to deserialize podman machine output with err {:?}",
                err
            );
            return GeneratorResults::default();
        }
    };

    machines
        .into_iter()
        .map(|machine| {
            let status = if machine.running {
                "running"
            } else {
                "stopped"
            };
            Suggestion::with_description(machine.name, format!("{} - {}", machine.vm_type, status))
        })
        .collect_unordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("podman")
        .add_generator(
            "running_podman_containers",
            Generator::script(
                CommandBuilder::single_command("podman ps --format '{{ json . }}'"),
                post_process_podman_ps,
            ),
        )
        .add_generator(
            "all_podman_containers",
            Generator::script(
                CommandBuilder::single_command("podman ps -a --format '{{ json . }}'"),
                post_process_podman_ps,
            ),
        )
        .add_generator(
            "paused_podman_containers",
            Generator::script(
                CommandBuilder::single_command(
                    "podman ps --filter status=paused --format '{{ json . }}'",
                ),
                post_process_podman_ps,
            ),
        )
        .add_generator(
            "podman_images",
            Generator::script(
                CommandBuilder::single_command("podman images --format '{{ json . }}'"),
                post_process_podman_images,
            ),
        )
        .add_generator(
            "podman_pods",
            Generator::script(
                CommandBuilder::single_command("podman pod ps --format '{{.Name}}|{{.Status}}'"),
                post_process_podman_pods,
            ),
        )
        .add_generator(
            "podman_volumes",
            Generator::script(
                CommandBuilder::single_command("podman volume ls --format '{{ json . }}'"),
                post_process_podman_volumes,
            ),
        )
        .add_generator(
            "podman_networks",
            Generator::script(
                CommandBuilder::single_command("podman network ls --format '{{ json . }}'"),
                post_process_podman_networks,
            ),
        )
        .add_generator(
            "podman_secrets",
            Generator::script(
                CommandBuilder::single_command("podman secret ls --format '{{ json . }}'"),
                post_process_podman_secrets,
            ),
        )
        .add_generator(
            "podman_machines",
            Generator::script(
                CommandBuilder::single_command("podman machine list --format json"),
                post_process_podman_machines,
            ),
        )
}
