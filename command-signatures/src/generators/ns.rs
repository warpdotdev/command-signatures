use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

fn nativescript_templates() -> Generator {
    Generator::script(
        CommandBuilder::single_command(
            "curl -sfL -H 'User-Agent: warp-completions' -H 'Accept: application/vnd.github+json' https://api.github.com/repos/NativeScript/nativescript-app-templates/contents/packages",
        ),
        output_parsers::json_nativescript_templates,
    )
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ns")
        .add_generator("nativescript_templates", nativescript_templates())
}

pub fn tns_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("tns")
        .add_generator("nativescript_templates", nativescript_templates())
}

pub fn nativescript_generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("nativescript")
        .add_generator("nativescript_templates", nativescript_templates())
}
