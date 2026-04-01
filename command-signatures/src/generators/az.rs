use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

fn az_post_process(output: &str, description: &str) -> GeneratorResults {
    output
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Suggestion::with_description(line.trim(), description))
        .collect_unordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("az")
        .add_generator(
            "resource_groups",
            Generator::script(
                CommandBuilder::single_command(
                    "az group list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| az_post_process(output, "Resource group"),
            ),
        )
        .add_generator(
            "subscriptions",
            Generator::script(
                CommandBuilder::single_command(
                    "az account list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| az_post_process(output, "Subscription"),
            ),
        )
        .add_generator(
            "locations",
            Generator::script(
                CommandBuilder::single_command(
                    "az account list-locations --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| az_post_process(output, "Location"),
            ),
        )
        .add_generator(
            "vms",
            Generator::script(
                CommandBuilder::single_command("az vm list --query \"[].name\" -o tsv 2>/dev/null"),
                |output| az_post_process(output, "Virtual Machine"),
            ),
        )
        .add_generator(
            "aks_clusters",
            Generator::script(
                CommandBuilder::single_command(
                    "az aks list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| az_post_process(output, "AKS cluster"),
            ),
        )
        .add_generator(
            "storage_accounts",
            Generator::script(
                CommandBuilder::single_command(
                    "az storage account list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| az_post_process(output, "Storage account"),
            ),
        )
        .add_generator(
            "container_registries",
            Generator::script(
                CommandBuilder::single_command(
                    "az acr list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| az_post_process(output, "Container registry"),
            ),
        )
        .add_generator(
            "key_vaults",
            Generator::script(
                CommandBuilder::single_command(
                    "az keyvault list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| az_post_process(output, "Key Vault"),
            ),
        )
        .add_generator(
            "webapps",
            Generator::script(
                CommandBuilder::single_command(
                    "az webapp list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| az_post_process(output, "Web app"),
            ),
        )
        .add_generator(
            "functionapps",
            Generator::script(
                CommandBuilder::single_command(
                    "az functionapp list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| az_post_process(output, "Function app"),
            ),
        )
        .add_generator(
            "nsgs",
            Generator::script(
                CommandBuilder::single_command(
                    "az network nsg list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| az_post_process(output, "Network security group"),
            ),
        )
        .add_generator(
            "vnets",
            Generator::script(
                CommandBuilder::single_command(
                    "az network vnet list --query \"[].name\" -o tsv 2>/dev/null",
                ),
                |output| az_post_process(output, "Virtual network"),
            ),
        )
}
