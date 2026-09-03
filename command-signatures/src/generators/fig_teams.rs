use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("fig-teams@latest")
        .add_generator(
            "teams_list",
            Generator::script(
                CommandBuilder::single_command("npx -y fig-teams@latest teams ls --json"),
                output_parsers::json_string_array,
            ),
        )
        .add_generator(
            "users_for_selected_team",
            Generator::command_from_tokens(
                users_for_selected_team,
                output_parsers::json_string_array,
            ),
        )
}

fn users_for_selected_team(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let team = tokens.iter().enumerate().find_map(|(i, token)| {
        if (*token == "-t" || *token == "--team") && i + 1 < tokens.len() {
            Some(tokens[i + 1])
        } else {
            None
        }
    });
    match team {
        Some(team) if !team.is_empty() => CommandBuilder::single_command(format!(
            "npx -y fig-teams@latest users get -t {} --json",
            shell_single_quote(team)
        )),
        _ => CommandBuilder::single_command("true"),
    }
}

fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', r"'\''"))
}
