use super::podman::{
    post_process_podman_images, post_process_podman_machines, post_process_podman_networks,
    post_process_podman_pods, post_process_podman_ps, post_process_podman_secrets,
    post_process_podman_volumes,
};

/// Captured from `podman ps -a --format '{{ json . }}'` on podman 5.4.2.
const PS_OUTPUT: &str = r#"{"AutoRemove":false,"Command":["sleep","300"],"Created":"2026-09-09T18:06:22.031132143Z","CreatedAt":"","CIDFile":"","Exited":false,"ExitedAt":-62135596800,"ExitCode":0,"ExposedPorts":null,"Id":"961ebd04c842","Image":"docker.io/library/alpine:latest","ImageID":"d529dd0c6e55","IsInfra":false,"Labels":null,"Mounts":[],"Names":["test-ctr"],"Namespaces":{},"Networks":[],"Pid":0,"Pod":"","PodName":"","Ports":null,"Restarts":0,"Size":null,"StartedAt":-62135596800,"State":"created","Status":""}"#;

#[test]
fn test_post_process_podman_ps_parses_real_output() {
    let results = post_process_podman_ps(PS_OUTPUT);

    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(results.suggestions[0].exact_string, "test-ctr");
    assert_eq!(
        results.suggestions[0].display_name.as_deref(),
        Some("test-ctr (docker.io/library/alpine:latest)")
    );
}

#[test]
fn test_post_process_podman_ps_empty_output() {
    assert!(post_process_podman_ps("").suggestions.is_empty());
}

/// `podman images --format '{{ json . }}'` mixes casing: `repository`/`tag` are
/// lower-cased while most other fields (e.g. `Id`, `Size`) are PascalCase.
const IMAGES_OUTPUT: &str = r#"{"repository":"docker.io/library/alpine","tag":"latest","Id":"d529dd0c6e55","ParentId":"","RepoTags":["docker.io/library/alpine:latest"],"RepoDigests":[],"Created":1781568089,"Size":8709729,"SharedSize":0,"VirtualSize":8709729,"Labels":null,"Containers":2,"Digest":"sha256:abc","History":[],"Names":[]}"#;

#[test]
fn test_post_process_podman_images_parses_real_output() {
    let results = post_process_podman_images(IMAGES_OUTPUT);

    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(
        results.suggestions[0].exact_string,
        "docker.io/library/alpine"
    );
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("latest")
    );
}

#[test]
fn test_post_process_podman_images_skips_none_repository() {
    let output = r#"{"repository":"<none>","tag":"<none>"}"#;
    assert!(post_process_podman_images(output).suggestions.is_empty());
}

#[test]
fn test_post_process_podman_pods_parses_pipe_separated_rows() {
    let output = "mypod|Running\n\nother-pod|Exited";
    let results = post_process_podman_pods(output);
    let names: Vec<&str> = results
        .suggestions
        .iter()
        .map(|suggestion| suggestion.exact_string.as_str())
        .collect();

    assert_eq!(names, vec!["mypod", "other-pod"]);
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("Running")
    );
}

#[test]
fn test_post_process_podman_pods_empty_output() {
    assert!(post_process_podman_pods("").suggestions.is_empty());
}

/// Captured from `podman volume ls --format '{{ json . }}'` on podman 5.4.2.
const VOLUME_OUTPUT: &str = r#"{"Name":"test-vol","Driver":"local","Mountpoint":"/x","CreatedAt":"2026-09-09T18:06:40Z","Labels":{},"Scope":"local","Options":{},"MountCount":0,"NeedsCopyUp":true,"NeedsChown":true,"LockNumber":2}"#;

#[test]
fn test_post_process_podman_volumes_parses_real_output() {
    let results = post_process_podman_volumes(VOLUME_OUTPUT);

    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(results.suggestions[0].exact_string, "test-vol");
    assert_eq!(results.suggestions[0].description.as_deref(), Some("local"));
}

/// Captured from `podman network ls --format '{{ json . }}'` on podman 5.4.2. Unlike most
/// other podman list commands, network fields are lower snake_case, not PascalCase.
const NETWORK_OUTPUT: &str = r#"{"name":"podman","id":"2f259bab93aa","driver":"bridge","network_interface":"podman0","created":"2026-09-09T18:06:41Z","subnets":[],"ipv6_enabled":false,"internal":false,"dns_enabled":false,"ipam_options":{}}"#;

#[test]
fn test_post_process_podman_networks_parses_real_output() {
    let results = post_process_podman_networks(NETWORK_OUTPUT);

    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(results.suggestions[0].exact_string, "podman");
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("bridge")
    );
}

/// Captured from `podman secret ls --format '{{ json . }}'` on podman 5.4.2.
const SECRET_OUTPUT: &str = r#"{"ID":"a9b29ea1a8fc","Name":"test-secret","Driver":"file","CreatedAt":"now","UpdatedAt":"now"}"#;

#[test]
fn test_post_process_podman_secrets_parses_real_output() {
    let results = post_process_podman_secrets(SECRET_OUTPUT);

    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(results.suggestions[0].exact_string, "test-secret");
    assert_eq!(results.suggestions[0].description.as_deref(), Some("file"));
}

/// `podman machine list --format json` prints a single JSON array of objects, matching
/// the example in the official `podman-machine-list` documentation.
const MACHINE_OUTPUT: &str = r#"[{"Name":"podman-machine-default","Default":false,"Created":"2021-12-27T10:36:14Z","Running":false,"LastUp":"2021-12-27T11:22:50Z","Stream":"default","VMType":"qemu","CPUs":1,"Memory":"2147483648","DiskSize":"10737418240"}]"#;

#[test]
fn test_post_process_podman_machines_parses_real_output() {
    let results = post_process_podman_machines(MACHINE_OUTPUT);

    assert_eq!(results.suggestions.len(), 1);
    assert_eq!(
        results.suggestions[0].exact_string,
        "podman-machine-default"
    );
    assert_eq!(
        results.suggestions[0].description.as_deref(),
        Some("qemu - stopped")
    );
}

#[test]
fn test_post_process_podman_machines_empty_output() {
    assert!(post_process_podman_machines("").suggestions.is_empty());
}

#[test]
fn test_post_process_podman_machines_empty_array() {
    assert!(post_process_podman_machines("[]").suggestions.is_empty());
}
