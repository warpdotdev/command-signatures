use std::collections::HashMap;
use std::iter::FromIterator;
use warp_completion_metadata::{Aliases, Filters, Generators};

/// Used for debian-based package managers like apt-get, aptitude, etc.
mod apt;
mod bazel;
mod bosh;
mod brew;
mod cargo;
mod conda;
mod defaults;
mod dnf;
mod docker;
mod firebase;
mod flutter;
mod gh;
mod git;
mod go;
mod heroku;
mod kill;
mod killall;
mod kubecolor;
mod kubectl;
mod kubectx;
mod kubens;
mod make;
mod man;
mod ng;
mod node;
mod npm;
mod nx;
mod pacman;
mod phpunit_watcher;
mod pip;
mod pyenv;
mod react_native;
mod screen;
mod ssh;
mod tar;
mod terraform;
mod tmux;
mod tmuxinator;

pub fn command_signature_generators() -> HashMap<String, (Generators, Filters, Aliases)> {
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
        kubectl::generator(),
        kubecolor::generator(),
        kill::generator(),
        killall::generator(),
        tmuxinator::generator(),
        tmux::generator(),
        node::generator(),
        screen::generator(),
    ];

    HashMap::from_iter(command_signature_generators.map(Into::into))
}
