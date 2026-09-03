use std::collections::HashMap;
use std::iter::FromIterator;
use warp_completion_metadata::DynamicCompletionData;

mod amplify;
mod ansible_doc;
mod assimp;
mod autojump;
mod bat;
mod black;
mod checkov;
mod cmd_n;
mod common;
mod copilot;
mod cordova;
mod deno;
mod deployctl;
mod deta;
mod dtm;
mod eb;
mod elm;
mod elm_review;
mod eslint;
mod expo;
mod expo_cli;
mod ffmpeg;
mod fig_parse;
mod fig_teams_latest;
mod fisher;
mod fly;
mod flyctl;
mod fnm;
mod fvm;
mod gpg;
mod hexo;
mod hugo;
mod hyper;
mod id;
mod ignite_cli;
mod kool;
mod lerna;
mod limactl;
mod mackup;
mod mdfind;
mod meteor;
mod mix;
mod mosh;
mod networkquality;
mod npx;
mod nr;
mod ns;
mod okteto;
mod op;
mod open;
mod pandoc;
mod pre_commit;
mod projj;
mod quickmail;
mod r;
mod rancher;
mod rclone;
mod redwood;
mod robot;
mod rush;
mod rushx;
mod rustup;
mod scc;
mod sftp;
mod shortcuts;
mod softwareupdate;
mod stepzen;
mod sysctl;
mod tailscale;
mod tccutil;
mod terragrunt;
mod tfenv;
mod tfsec;
mod tokei;
mod trex;
mod turbo;
mod vite;
mod vr;
mod vsce;
mod vultr_cli;
mod watson;
mod wd;
mod wifi_password;
mod yo;
mod youtube_dl;

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
mod dotnet;
mod firebase;
mod flutter;
mod gh;
mod git;
mod go;
mod gt;
mod heroku;
mod ip;
mod journalctl;
#[cfg(test)]
mod journalctl_tests;
mod just;
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
mod nmap;
mod node;
mod npm;
mod nx;
mod oc;
mod pacman;
mod paru;
mod pass;
mod phpunit_watcher;
mod pip;
mod pkill;
#[cfg(test)]
mod pkill_tests;
mod powershell;
mod pprof;
mod pyenv;
mod react_native;
mod ros2;
mod scp;
mod screen;
mod sdk;
mod ssh;
mod systemctl;
mod tar;
mod tcpdump;
mod terraform;
mod timedatectl;
mod tmux;
mod tmuxinator;
mod tsh;
mod uv;
mod vagrant;
mod yay;
mod yc;

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
        dotnet::generator(),
        docker::generator(),
        docker_compose::generator(),
        firebase::generator(),
        flutter::generator(),
        gh::generator(),
        git::generator(),
        gt::generator(),
        go::generator(),
        heroku::generator(),
        ip::generator(),
        journalctl::generator(),
        just::generator(),
        make::generator(),
        man::generator(),
        ng::generator(),
        nextflow::generator(),
        nmap::generator(),
        npm::npm_generators(),
        npm::yarn_generators(),
        nx::generator(),
        pacman::generator(),
        paru::generator(),
        pass::generator(),
        phpunit_watcher::generator(),
        pip::generator(),
        pip::pip3_generator(),
        pkill::generator(),
        npm::pnpm_generators(),
        pprof::generator(),
        pyenv::generator(),
        react_native::generator(),
        scp::generator(),
        ssh::generator(),
        tar::generator(),
        tcpdump::generator(),
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
        uv::generator(),
        vagrant::generator(),
        gcloud::gcloud_generators(),
        gcloud::gsutil_generators(),
        yay::generator(),
        yc::generator(),
        amplify::generator(),
        ansible_doc::generator(),
        apt::generator(),
        assimp::generator(),
        autojump::generator(),
        bat::generator(),
        black::generator(),
        checkov::generator(),
        copilot::generator(),
        cordova::generator(),
        deno::generator(),
        deployctl::generator(),
        deta::generator(),
        dtm::generator(),
        eb::generator(),
        elm::generator(),
        elm_review::generator(),
        eslint::generator(),
        expo::generator(),
        expo_cli::generator(),
        ffmpeg::generator(),
        fig_teams_latest::generator(),
        fisher::generator(),
        fly::generator(),
        flyctl::generator(),
        fnm::generator(),
        fvm::generator(),
        gpg::generator(),
        hexo::generator(),
        hugo::generator(),
        hyper::generator(),
        id::generator(),
        ignite_cli::generator(),
        kool::generator(),
        lerna::generator(),
        limactl::generator(),
        mackup::generator(),
        mdfind::generator(),
        meteor::generator(),
        mix::generator(),
        mosh::generator(),
        cmd_n::generator(),
        networkquality::generator(),
        npx::generator(),
        nr::generator(),
        ns::generator(),
        okteto::generator(),
        op::generator(),
        open::generator(),
        pandoc::generator(),
        pre_commit::generator(),
        projj::generator(),
        quickmail::generator(),
        r::generator(),
        rancher::generator(),
        rclone::generator(),
        redwood::generator(),
        robot::generator(),
        rush::generator(),
        rushx::generator(),
        rustup::generator(),
        scc::generator(),
        sftp::generator(),
        shortcuts::generator(),
        softwareupdate::generator(),
        stepzen::generator(),
        sysctl::generator(),
        tailscale::generator(),
        tccutil::generator(),
        terragrunt::generator(),
        tfenv::generator(),
        tfsec::generator(),
        tokei::generator(),
        trex::generator(),
        turbo::generator(),
        vite::generator(),
        vr::generator(),
        vsce::generator(),
        vultr_cli::generator(),
        watson::generator(),
        wd::generator(),
        wifi_password::generator(),
        yo::generator(),
        youtube_dl::generator(),
    ];

    HashMap::from_iter(command_signature_generators.map(Into::into))
}
