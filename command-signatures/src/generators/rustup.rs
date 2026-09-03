use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("rustup")
        .add_generator(
            "find_docs_s_index_html",
            Generator::script(
                CommandBuilder::single_command(r#"find $(rustup docs --path | sed -e "s|index\.html|std|") $(rustup docs --path | sed -e "s|index\.html|alloc|") $(rustup docs --path | sed -e "s|index\.html|core|") | grep "\.html" | sed -E -e "s|^(.*)/html/||" -e "s|\.html||" -e "s|/|::|g" -e "s/constant\.|trait\.|struct\.|macro\.|fn\.|keyword\.|primitive\.|type\.|enum\.|union\.|traitalias\.|::index$|^(.*)::all$//" -e "/^$/d""#),
                fig_parse::lines,
            ),
        )
}
