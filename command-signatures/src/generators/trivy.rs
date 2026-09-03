use super::{fig_token, output_parsers};
use warp_completion_metadata::{CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("trivy")
        .add_generator(
            "severity_remaining",
            Generator::command_from_tokens(
                fig_token::trivy_severity_remaining,
                output_parsers::lines,
            ),
        )
        .add_generator(
            "scanners_remaining",
            Generator::command_from_tokens(
                fig_token::trivy_scanners_remaining,
                output_parsers::lines,
            ),
        )
        .add_generator(
            "pkg_types_remaining",
            Generator::command_from_tokens(
                fig_token::trivy_pkg_types_remaining,
                output_parsers::lines,
            ),
        )
}
