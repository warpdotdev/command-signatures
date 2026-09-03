use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("watson")
        .add_generator(
            "projects_7",
            Generator::script(
                CommandBuilder::single_command("watson projects"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "tags_7",
            Generator::script(
                CommandBuilder::single_command("watson tags"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "projects_6",
            Generator::script(
                CommandBuilder::single_command("watson projects"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "tags_6",
            Generator::script(
                CommandBuilder::single_command("watson tags"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "log_json",
            Generator::script(
                CommandBuilder::single_command("watson log --json --reverse"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "projects_5",
            Generator::script(
                CommandBuilder::single_command("watson projects"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "tags_5",
            Generator::script(
                CommandBuilder::single_command("watson tags"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "projects_4",
            Generator::script(
                CommandBuilder::single_command("watson projects"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "tags_4",
            Generator::script(
                CommandBuilder::single_command("watson tags"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "log_watson",
            Generator::script(
                CommandBuilder::single_command("watson log --json --reverse"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "projects_3",
            Generator::script(
                CommandBuilder::single_command("watson projects"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "tags_3",
            Generator::script(
                CommandBuilder::single_command("watson tags"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "projects_2",
            Generator::script(
                CommandBuilder::single_command("watson projects"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "tags_2",
            Generator::script(
                CommandBuilder::single_command("watson tags"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "projects_watson",
            Generator::script(
                CommandBuilder::single_command("watson projects"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "tags_watson",
            Generator::script(
                CommandBuilder::single_command("watson tags"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "log",
            Generator::script(
                CommandBuilder::single_command("watson log --json --reverse"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "projects",
            Generator::script(
                CommandBuilder::single_command("watson projects"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "tags",
            Generator::script(
                CommandBuilder::single_command("watson tags"),
                fig_parse::lines,
            ),
        )
}
