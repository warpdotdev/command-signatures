use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("rush")
        .add_generator(
            "until_rush_json_do_cd_14",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_13",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_12",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_11",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_10",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_9",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_8",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_7",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_6",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_5",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_4",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_3",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_2",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_cat",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_done",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_pwd",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd_f",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "until_rush_json_do_cd",
            Generator::script(
                CommandBuilder::single_command(
                    "until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json",
                ),
                fig_parse::lines,
            ),
        )
}
