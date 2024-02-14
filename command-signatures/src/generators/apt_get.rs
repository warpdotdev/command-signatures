use regex::Regex;
use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorResultsCollector, Importance, Order, Priority,
    Suggestion,
};

use lazy_static::lazy_static;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("apt-get").add_generator(
        "list_all_packages",
        Generator::script(
            r#"dpkg-query --show --showformat '${Package}\n'"#,
            |output| {
                println!("completed dpkg-query");
                let mut targets = Vec::new();
                let mut current_path = String::new();
                for package_name in output.lines() {
                    println!("{:?}", package_name);
                    targets.push(Suggestion::with_description(
                        format!("{}", package_name),
                        "package",
                    ));
                }
                targets.into_iter().collect_unordered_results()
            },
        ),
    )
}
