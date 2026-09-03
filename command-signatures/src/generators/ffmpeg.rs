use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ffmpeg")
        .add_generator(
            "completions",
            Generator::script(
                CommandBuilder::single_command("ffmpeg -codecs"),
                fig_parse::second_whitespace_token,
            ),
        )
        .add_generator(
            "completions_devices",
            Generator::script(
                CommandBuilder::single_command("ffmpeg -devices"),
                fig_parse::second_whitespace_token,
            ),
        )
}
