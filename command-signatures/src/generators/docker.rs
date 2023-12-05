use serde_json::Result;
use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResults, GeneratorResultsCollector, IconType,
    Suggestion, TemplateFilter,
};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DockerOutput {
    #[serde(rename = "ID")]
    id: Option<String>,

    image: Option<String>,

    name: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DockerImageOutput {
    // These fields can return `Null`, hence they are all optional.
    #[serde(default, rename = "ID")]
    id: Option<String>,

    #[serde(default)]
    repository: Option<String>,

    #[serde(default)]
    tag: Option<String>,

    size: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DockerVolumeOutput {
    name: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DockerContextOutput {
    name: Option<String>,

    description: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DockerSwarmOutput {
    id: Option<String>,

    status: Option<String>,
}

fn post_process_docker_ps(output: &str) -> GeneratorResults {
    output
        .trim()
        .split('\n')
        .filter_map(|line| {
            let parsed_output: Result<DockerOutput> = serde_json::from_str(line);
            if let Ok(output) = parsed_output {
                if let Some(id) = output.id {
                    Some(
                        Suggestion::with_description(id, "Container")
                            .with_icon(IconType::DockerContainer),
                    )
                } else {
                    None
                }
            } else {
                log::info!(
                    "unable to parse docker output: {:?}",
                    parsed_output.err().unwrap()
                );
                None
            }
        })
        .collect_unordered_results()
}

fn shared_post_process(output: &str) -> GeneratorResults {
    output
        .trim()
        .split('\n')
        .filter_map(|line| {
            let parsed_output: DockerOutput = serde_json::from_str(line).ok()?;

            match (parsed_output.name, parsed_output.id) {
                (Some(name), Some(id)) => Some(Suggestion::with_description(name, id)),
                _ => None,
            }
        })
        .collect_unordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("docker")
        .add_generator(
            "running_docker_containers",
            Generator::script("docker ps --format '{{ json . }}'", post_process_docker_ps),
        )
        .add_generator(
            "all_docker_containers",
            Generator::script(
                "docker ps -a --format '{{ json . }}'",
                post_process_docker_ps,
            ),
        )
        .add_generator(
            "paused_docker_containers",
            Generator::script(
                "docker ps --filter status=paused --format '{{ json . }}'",
                post_process_docker_ps,
            ),
        )
        .add_generator(
            "all_local_images",
            Generator::script("docker image ls --format '{{ json . }}'", |output| {
                if output.trim().is_empty() {
                    return GeneratorResults::default();
                }

                output
                    .lines()
                    .filter_map(|line| {
                        let docker_image_output: Result<DockerImageOutput> =
                            serde_json::from_str(line);
                        if let Ok(docker_image_output) = docker_image_output {
                            docker_image_output.repository.map(|repository| {
                                Suggestion::new(repository).with_icon(IconType::DockerImage)
                            })
                        } else {
                            log::warn!(
                                "Unable to deserialize docker image output with err {:?}",
                                docker_image_output.err().unwrap()
                            );
                            None
                        }
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "all_docker_contexts",
            Generator::script("docker context list --format '{{ json . }}'", |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        let docker_context_output: DockerContextOutput =
                            serde_json::from_str(line).ok()?;

                        match (
                            docker_context_output.name,
                            docker_context_output.description,
                        ) {
                            (Some(name), Some(description)) => {
                                Some(Suggestion::with_description(name, description))
                            }
                            _ => None,
                        }
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "list_docker_networks",
            Generator::script(
                "docker network list --format '{{ json . }}'",
                shared_post_process,
            ),
        )
        .add_generator(
            "list_docker_swarm_nodes",
            Generator::script("docker node list --format '{{ json . }}'", |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        let docker_swarm_output: DockerSwarmOutput =
                            serde_json::from_str(line).ok()?;
                        match (docker_swarm_output.id, docker_swarm_output.status) {
                            (Some(id), Some(status)) => {
                                Some(Suggestion::with_description(id, status))
                            }
                            _ => None,
                        }
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "list_docker_plugins",
            Generator::script(
                "docker plugin list --format '{{ json . }}'",
                shared_post_process,
            ),
        )
        .add_generator(
            "list_docker_secrets",
            Generator::script(
                "docker secret list --format '{{ json . }}'",
                shared_post_process,
            ),
        )
        .add_generator(
            "list_docker_services",
            Generator::script("docker service list --format '{{ json . }}'", |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        let docker_output: DockerOutput = serde_json::from_str(line).ok()?;

                        match (docker_output.name, docker_output.image) {
                            (Some(name), Some(image)) => {
                                Some(Suggestion::with_description(name, image))
                            }
                            _ => None,
                        }
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "list_docker_service_replicas",
            Generator::script("docker service list --format '{{ json . }}'", |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        let docker_output: DockerOutput = serde_json::from_str(line).ok()?;

                        match (docker_output.name, docker_output.image) {
                            (Some(name), Some(image)) => {
                                Some(Suggestion::with_description(format!("{}=", name), image))
                            }
                            _ => None,
                        }
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "list_docker_stacks",
            Generator::script("docker stack list --format '{{ json . }}'", |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        let docker_output: DockerOutput = serde_json::from_str(line).ok()?;
                        docker_output.name.map(Suggestion::new)
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "list_docker_volumes",
            Generator::script("docker volume list --format '{{ json . }}'", |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        let docker_output: DockerOutput = serde_json::from_str(line).ok()?;
                        docker_output.name.map(Suggestion::new)
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "docker_images",
            Generator::script("docker images -a --format '{{ json . }}'", |output| {
                if output.trim().is_empty() {
                    return GeneratorResults::default();
                }

                output
                    .lines()
                    .filter_map(|line| {
                        let docker_image_output: Result<DockerImageOutput> =
                            serde_json::from_str(line);
                        if let Ok(docker_image_output) = docker_image_output {
                            docker_image_output.repository.map(Suggestion::new)
                        } else {
                            log::warn!(
                                "Unable to deserialize docker image output with err {:?}",
                                docker_image_output.err().unwrap()
                            );
                            None
                        }
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "docker_volumes",
            Generator::script("docker volume ls --format '{{ json . }}'", |output| {
                if output.trim().is_empty() {
                    return GeneratorResults::default();
                }

                output
                    .lines()
                    .filter_map(|line| {
                        let docker_volume_output: Result<DockerVolumeOutput> =
                            serde_json::from_str(line);
                        if let Ok(docker_volume_output) = docker_volume_output {
                            docker_volume_output.name.map(Suggestion::new)
                        } else {
                            log::warn!(
                                "Unable to deserialize docker volume output with err {:?}",
                                docker_volume_output.err().unwrap()
                            );
                            None
                        }
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "remove_images",
            Generator::script("docker images -aq --format '{{ json . }}'", |output| {
                if output.trim().is_empty() {
                    return GeneratorResults::default();
                }

                output
                    .lines()
                    .filter_map(|line| {
                        let docker_image_output: Result<DockerImageOutput> =
                            serde_json::from_str(line);
                        if let Ok(docker_image_output) = docker_image_output {
                            docker_image_output.repository.map(|repository| {
                                Suggestion::new(repository).with_icon(IconType::DockerImage)
                            })
                        } else {
                            log::info!(
                                "Unable to deserialize docker image output with err {:?}",
                                docker_image_output.err().unwrap()
                            );
                            None
                        }
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "run_images",
            Generator::script("docker images --format '{{ json . }}'", |output| {
                if output.trim().is_empty() {
                    return GeneratorResults::default();
                }

                output
                    .lines()
                    .filter_map(|image| {
                        let docker_image_output: Result<DockerImageOutput> =
                            serde_json::from_str(image);
                        if let Ok(docker_image_output) = docker_image_output {
                            if let (Some(repo), Some(size), Some(tag), Some(id)) = (
                                docker_image_output.repository,
                                docker_image_output.size,
                                docker_image_output.tag,
                                docker_image_output.id,
                            ) {
                                Some(
                                    Suggestion::with_description(
                                        repo,
                                        format!("{}@{} -{}", id, tag, size),
                                    )
                                    .with_icon(IconType::DockerImage),
                                )
                            } else {
                                None
                            }
                        } else {
                            log::info!(
                                "Unable to deserialize docker image output with err {:?}",
                                docker_image_output.err().unwrap()
                            );
                            None
                        }
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "docker_image_with_tag_and_size",
            Generator::script(
                "docker images --format '{{.Repository}} {{.Size}} {{.Tag}} {{.ID}}'",
                |output| {
                    output
                        .split('\n')
                        .filter_map(|line| {
                            let words: Vec<&str> = line.split(' ').collect();
                            (words.len() >= 4).then(|| {
                                let id = words[1];
                                let tag = words[2];
                                let size = words[3];
                                Suggestion::with_description(
                                    words[0],
                                    format!("{}@{} - {}", id, tag, size),
                                )
                                .with_icon(IconType::DockerImage)
                            })
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_filter(
            "filter-docker-files",
            TemplateFilter(|suggestion, file_type| {
                (file_type.is_folder()
                    || suggestion.exact_string.ends_with(".yaml")
                    || suggestion.exact_string.ends_with(".yml"))
                .then_some(suggestion)
            }),
        )
}
