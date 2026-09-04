use super::{fig_token, output_parsers, template_filters};
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("robot")
        .add_generator(
            "for_i_in_robot",
            Generator::script(
                CommandBuilder::single_command(
                    r#"for i in $(find -E . -regex ".*.robot" -type f); do cat -s $i ; done"#,
                ),
                output_parsers::robot_tags,
            ),
        )
        .add_generator(
            "test_cases",
            Generator::script(
                CommandBuilder::single_command(
                    r#"for i in $(find -E . -regex ".*.robot" -type f); do cat -s $i ; done"#,
                ),
                output_parsers::robot_test_cases,
            ),
        )
        .add_generator(
            "variables",
            Generator::command_from_tokens(
                fig_token::robot_variables,
                output_parsers::robot_variables,
            ),
        )
        .add_filter("filter-robot", template_filters::robot())
        .add_filter("filter-xml", template_filters::xml())
        .add_filter("filter-py-yaml", template_filters::py_yaml())
        .add_filter("filter-zip", template_filters::zip())
}
