use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("fly")
        .add_generator(
            "flyctl_list_orgs_12",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_orgs_11",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_80",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_79",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_78",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_77",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_76",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_75",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_74",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_73",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_72",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_71",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_70",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_69",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_68",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_67",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_66",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_65",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_64",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_63",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_62",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_61",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_60",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_orgs_10",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_59",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_58",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_57",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_56",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_orgs_9",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_55",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_54",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_53",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_52",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_51",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_50",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_49",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_48",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_47",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_46",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_45",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_44",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_orgs_8",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_orgs_7",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_43",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_42",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_orgs_6",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_41",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_40",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_39",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_38",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_orgs_5",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_37",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_36",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_35",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_34",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_orgs_4",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_33",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_32",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_orgs_3",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_31",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_30",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_29",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_28",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_27",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_26",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_orgs_2",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_25",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_24",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_23",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_22",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_21",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_20",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_19",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_18",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_17",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_16",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_15",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_14",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_13",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_12",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_11",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_10",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_9",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_8",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_7",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_6",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_orgs",
            Generator::script(
                CommandBuilder::single_command("flyctl list orgs"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_5",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_4",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_3",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps_2",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
        .add_generator(
            "flyctl_list_apps",
            Generator::script(
                CommandBuilder::single_command("flyctl list apps"),
                fig_parse::lines,
            ),
        )
}
