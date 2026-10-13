use warp_completion_metadata::{DynamicCompletionData, Generator, GeneratorProcess, Shell};

fn environment_generator() -> Generator {
    let (_, data): (String, DynamicCompletionData) = super::generator().into();
    data.generators()[&"get_conda_environments".into()].clone()
}

#[test]
fn environment_generator_invokes_conda_env_list() {
    let generator = environment_generator();
    let GeneratorProcess::ShellCommand(command) = &generator.process else {
        panic!("environment completion should not depend on input tokens");
    };

    for shell in [Shell::Posix, Shell::Powershell, Shell::CmdExe] {
        assert_eq!(command.build(shell), "conda env list");
    }
}

#[test]
fn environment_generator_suggests_names_from_conda_output() {
    let generator = environment_generator();
    let output = "\n# conda environments:\n#\nbase                 * /opt/conda\nmy-env                 /opt/conda/envs/my-env\n";

    let results = generator.on_complete(output);
    let names: Vec<&str> = results
        .suggestions
        .iter()
        .map(|suggestion| suggestion.exact_string.as_str())
        .collect();

    assert_eq!(names, ["base", "my-env"]);
    assert!(!results.is_ordered);
    assert!(generator.on_complete("").suggestions.is_empty());
}
