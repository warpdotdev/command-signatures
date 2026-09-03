use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("stepzen")
        .add_generator(
            "curl_https_api_github_com_repos_steprz_stepzen_schemas_contents",
            Generator::script(
                CommandBuilder::single_command(
                    "curl https://api.github.com/repos/steprz/stepzen-schemas/contents",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "list_schemas",
            Generator::script(
                CommandBuilder::single_command("stepzen list schemas"),
                fig_parse::lines,
            ),
        )
}
