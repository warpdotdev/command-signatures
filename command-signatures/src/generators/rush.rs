use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("rush")
        .add_generator(
            "until_rush_json_do_cd",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                output_parsers::json_rush_projects,
            ),
        )
        .add_generator(
            "npms_search",
            Generator::command_from_tokens(
                super::fig_token::npms_search,
                output_parsers::npms_search_results,
            ),
        )
}
