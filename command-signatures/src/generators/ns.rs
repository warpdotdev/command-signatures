use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("ns")
        .add_generator(
            "curl_https_api_github_com_repos_nativescript_nativescript_app_templates_contents_packages",
            Generator::script(
                CommandBuilder::single_command("curl https://api.github.com/repos/NativeScript/nativescript-app-templates/contents/packages"),
                fig_parse::lines,
            ),
        )
}
