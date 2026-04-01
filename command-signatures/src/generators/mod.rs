use std::collections::HashMap;
use std::iter::FromIterator;
use warp_completion_metadata::DynamicCompletionData;

mod common;

/// Used for debian-based package managers like apt-get, aptitude, etc.
mod apt;
mod bazel;
mod bosh;
mod brew;
mod cargo;
mod codex;
mod conda;
mod defaults;
mod dnf;
mod docker;
mod firebase;
mod flutter;
mod gh;
mod git;
mod go;
mod gt;
mod heroku;
mod kill;
mod killall;
mod kubecolor;
mod kubectl;
mod kubectx;
mod kubens;
mod lsof;
mod make;
mod man;
mod ng;
mod node;
mod npm;
mod nx;
mod pacman;
mod phpunit_watcher;
mod pip;
mod powershell;
mod pyenv;
mod react_native;
mod screen;
mod ssh;
mod systemctl;
mod tar;
mod terraform;
mod timedatectl;
mod tmux;
mod tmuxinator;

/// Returns dynamic command signature data, keyed on the command the data corresponds to.
pub fn dynamic_command_signature_data() -> HashMap<String, DynamicCompletionData> {
    let command_signature_generators = [
        apt::apt_get_generators(),
        apt::aptitude_generators(),
        bosh::generator(),
        brew::generator(),
        conda::generator(),
        defaults::generator(),
        dnf::generator(),
        docker::generator(),
        firebase::generator(),
        flutter::generator(),
        gh::generator(),
        git::generator(),
        gt::generator(),
        go::generator(),
        heroku::generator(),
        make::generator(),
        man::generator(),
        ng::generator(),
        npm::npm_generators(),
        npm::yarn_generators(),
        nx::generator(),
        pacman::generator(),
        phpunit_watcher::generator(),
        pip::generator(),
        pip::pip3_generator(),
        npm::pnpm_generators(),
        pyenv::generator(),
        react_native::generator(),
        ssh::generator(),
        tar::generator(),
        terraform::generator(),
        kubectx::generator(),
        kubens::generator(),
        bazel::generator(),
        cargo::generator(),
        codex::generator(),
        kubectl::generator(),
        kubecolor::generator(),
        kill::generator(),
        killall::generator(),
        lsof::generator(),
        tmuxinator::generator(),
        systemctl::generator(),
        timedatectl::generator(),
        tmux::generator(),
        node::generator(),
        screen::generator(),
        powershell::get_help_generator(),
        powershell::get_process_generator(),
        powershell::debug_process_generator(),
        powershell::wait_process_generator(),
        powershell::enter_ps_host_process_generator(),
        powershell::get_variable_generator(),
        powershell::clear_variable_generator(),
        powershell::set_variable_generator(),
        powershell::remove_variable_generator(),
    ];

    HashMap::from_iter(command_signature_generators.map(Into::into))
}
