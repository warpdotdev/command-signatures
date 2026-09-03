use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("robot")
        .add_generator(
            "for_i_in_robot_3",
            Generator::script(
                CommandBuilder::single_command(
                    "for i in $(find -E . -regex \".*.robot\" -type f); do cat -s $i ; done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "for_i_in_robot_2",
            Generator::script(
                CommandBuilder::single_command(
                    "for i in $(find -E . -regex \".*.robot\" -type f); do cat -s $i ; done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "for_i_in_robot_done",
            Generator::script(
                CommandBuilder::single_command(
                    "for i in $(find -E . -regex \".*.robot\" -type f); do cat -s $i ; done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "for_i_in_robot_s",
            Generator::script(
                CommandBuilder::single_command(
                    "for i in $(find -E . -regex \".*.robot\" -type f); do cat -s $i ; done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "for_i_in_robot_cat",
            Generator::script(
                CommandBuilder::single_command(
                    "for i in $(find -E . -regex \".*.robot\" -type f); do cat -s $i ; done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "for_i_in_robot_do",
            Generator::script(
                CommandBuilder::single_command(
                    "for i in $(find -E . -regex \".*.robot\" -type f); do cat -s $i ; done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "for_i_in_robot_type",
            Generator::script(
                CommandBuilder::single_command(
                    "for i in $(find -E . -regex \".*.robot\" -type f); do cat -s $i ; done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "for_i_in_robot_regex",
            Generator::script(
                CommandBuilder::single_command(
                    "for i in $(find -E . -regex \".*.robot\" -type f); do cat -s $i ; done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "for_i_in_robot_e",
            Generator::script(
                CommandBuilder::single_command(
                    "for i in $(find -E . -regex \".*.robot\" -type f); do cat -s $i ; done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "for_i_in_robot_find",
            Generator::script(
                CommandBuilder::single_command(
                    "for i in $(find -E . -regex \".*.robot\" -type f); do cat -s $i ; done",
                ),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "for_i_in_robot",
            Generator::script(
                CommandBuilder::single_command(
                    "for i in $(find -E . -regex \".*.robot\" -type f); do cat -s $i ; done",
                ),
                fig_parse::lines,
            ),
        )
}
