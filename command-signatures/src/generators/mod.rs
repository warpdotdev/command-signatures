use std::collections::HashMap;
use std::iter::FromIterator;
use warp_completion_metadata::DynamicCompletionData;

mod common;

/// Used for debian-based package managers like apt-get, aptitude, etc.
mod apt;
mod asdf;
mod aws;
#[cfg(test)]
mod aws_tests;
mod az;
mod bazel;
mod bosh;
mod brew;
mod bun;
mod cargo;
mod claude;
mod codex;
mod conda;
mod defaults;
mod dnf;
mod docker;
mod docker_compose;
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
mod openssl;
mod pacman;
mod pass;
mod phpunit_watcher;
mod pip;
mod powershell;
mod pyenv;
mod react_native;
mod ros2;
mod scp;
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

/// Used for gcloud and gsutil completions.
mod gcloud;

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
        bun::generator(),
        conda::generator(),
        defaults::generator(),
        dnf::generator(),
        docker::generator(),
        docker_compose::generator(),
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
        scp::generator(),
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
        openssl::generator(),
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
        gcloud::gcloud_generators(),
        gcloud::gsutil_generators(),
    ];

    HashMap::from_iter(command_signature_generators.map(Into::into))
}
