use std::collections::HashMap;
use std::iter::FromIterator;
use warp_completion_metadata::DynamicCompletionData;

mod common;

/// Used for debian-based package managers like apt-get, aptitude, etc.
mod apt;
mod asdf;
mod aws;
mod az;
mod bazel;
mod bosh;
mod brew;
mod cargo;
mod claude;
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
mod nextflow;
mod ng;
mod node;
mod npm;
mod nx;
mod oc;
mod pacman;
mod pass;
mod phpunit_watcher;
mod pip;
mod powershell;
mod pyenv;
mod react_native;
mod ros2;
mod screen;
mod sdk;
mod ssh;
mod systemctl;
mod tar;
mod terraform;
mod timedatectl;
mod tmux;
mod tmuxinator;
mod tsh;

/// Returns dynamic command signature data, keyed on the command the data corresponds to.
pub fn dynamic_command_signature_data() -> HashMap<String, DynamicCompletionData> {
    let command_signature_generators = [
        aws::generator(),
        asdf::generator(),
        apt::apt_get_generators(),
        apt::aptitude_generators(),
        az::generator(),
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
        nextflow::generator(),
        npm::npm_generators(),
        npm::yarn_generators(),
        nx::generator(),
        pacman::generator(),
        pass::generator(),
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
        claude::generator(),
        codex::generator(),
        kubectl::generator(),
        kubecolor::generator(),
        oc::generator(),
        kill::generator(),
        killall::generator(),
        lsof::generator(),
        tmuxinator::generator(),
        systemctl::generator(),
        timedatectl::generator(),
        tmux::generator(),
        tsh::generator(),
        node::generator(),
        ros2::generator(),
        screen::generator(),
        sdk::generator(),
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
