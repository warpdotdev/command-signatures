use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ffmpeg")
        .add_generator(
            "completions",
            Generator::script(
                CommandBuilder::single_command("ffmpeg -codecs"),
                output_parsers::second_whitespace_token,
            ),
        )
        .add_generator(
            "completions_devices",
            Generator::script(
                CommandBuilder::single_command("ffmpeg -devices"),
                output_parsers::second_whitespace_token,
            ),
        )
}
