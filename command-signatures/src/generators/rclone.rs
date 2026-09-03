use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("rclone")
        .add_generator(
            "listremotes_27",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_26",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_25",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_24",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_23",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_22",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_21",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_20",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_19",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_18",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_17",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_16",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_15",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_14",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_13",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_12",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_11",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_10",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_9",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_8",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_7",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_6",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_5",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_4",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_3",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_2",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes_rclone",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "listremotes",
            Generator::script(
                CommandBuilder::single_command("rclone listremotes"),
                fig_parse::lines,
            ),
        )
}
