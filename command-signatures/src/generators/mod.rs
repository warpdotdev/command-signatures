use std::collections::HashMap;
use std::iter::FromIterator;
use warp_completion_metadata::{Filters, Generators};

mod bosh;
mod brew;
mod conda;
mod defaults;
mod docker;
mod firebase;
mod flutter;
mod gh;
mod git;
mod go;
mod heroku;
mod kill;
mod killall;
mod make;
mod man;
mod ng;
mod npm;
mod nx;
mod phpunit_watcher;
mod pip;
mod pyenv;
mod react_native;
mod ssh;
mod tar;
mod terraform;
mod tmux;
mod tmuxinator;

pub fn generators() -> HashMap<String, (Generators, Filters)> {
    let generators = [
        bosh::generator(),
        brew::generator(),
        conda::generator(),
        defaults::generator(),
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
        phpunit_watcher::generator(),
        pip::generator(),
        pip::pip3_generator(),
        pyenv::generator(),
        react_native::generator(),
        ssh::generator(),
        tar::generator(),
        terraform::generator(),
        kill::generator(),
        killall::generator(),
        tmuxinator::generator(),
        tmux::generator(),
    ];

    HashMap::from_iter(generators.map(Into::into))
}
