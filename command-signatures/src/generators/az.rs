use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("az")
        .add_generator(
            "resource_groups",
            Generator::script(
                CommandBuilder::single_command(
                    "az group list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Resource group"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "subscriptions",
            Generator::script(
                CommandBuilder::single_command(
                    "az account list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Subscription"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "installed_extensions",
            Generator::script(
                CommandBuilder::single_command(
                    "az extension list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| {
                            Suggestion::with_description(name.trim(), "Installed extension")
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "available_extensions",
            Generator::script(
                CommandBuilder::single_command(
                    "az extension list-available --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| {
                            Suggestion::with_description(name.trim(), "Available extension")
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "aks_clusters",
            Generator::script(
                CommandBuilder::single_command(
                    "az aks list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "AKS cluster"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "vms",
            Generator::script(
                CommandBuilder::single_command("az vm list --query \"[].name\" -o tsv 2>/dev/null"),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Virtual machine"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "webapps",
            Generator::script(
                CommandBuilder::single_command(
                    "az webapp list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Web app"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "acr_registries",
            Generator::script(
                CommandBuilder::single_command(
                    "az acr list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Container registry"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "storage_accounts",
            Generator::script(
                CommandBuilder::single_command(
                    "az storage account list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Storage account"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "functionapps",
            Generator::script(
                CommandBuilder::single_command(
                    "az functionapp list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Function app"))
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "keyvaults",
            Generator::script(
                CommandBuilder::single_command(
                    "az keyvault list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| {
                    output
                        .trim()
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|name| Suggestion::with_description(name.trim(), "Key vault"))
                        .collect_unordered_results()
                },
            ),
        )
}
