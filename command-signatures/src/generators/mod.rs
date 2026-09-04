use std::collections::HashMap;
use std::iter::FromIterator;
use warp_completion_metadata::DynamicCompletionData;

mod amplify;
mod ansible_doc;
mod ansible_playbook;
mod assimp;
mod autojump;
mod bat;
mod black;
mod checkov;
mod chown;
mod cloudflared;
mod cmd_n;
mod common;
mod copilot;
mod cordova;
mod dd;
mod degit;
mod deno;
mod deployctl;
mod deta;
mod direnv;
mod dtm;
mod eb;
mod elixir;
mod elm;
mod elm_review;
mod esbuild;
mod eslint;
mod expo;
mod expo_cli;
mod ffmpeg;
mod fig_teams;
mod fig_token;
mod file;
mod fisher;
mod fly;
mod flyctl;
mod fnm;
mod fvm;
mod git_flow;
mod gpg;
mod hexo;
mod hugo;
mod hyper;
mod id;
mod iex;
mod ignite_cli;
mod java;
mod julia;
mod kool;
mod lerna;
mod limactl;
mod mackup;
mod mdfind;
mod meteor;
mod mix;
mod mosh;
mod networkquality;
mod ni;
mod npx;
mod nr;
mod ns;
mod okteto;
mod op;
mod open;
mod osascript;
mod output_parsers;
mod pandoc;
mod pdfunite;
mod pm2;
mod pod;
mod pre_commit;
mod projj;
mod python;
mod quickmail;
mod r;
mod rancher;
mod rclone;
mod redwood;
mod robot;
mod rollup;
mod rscript;
mod rush;
mod rushx;
mod rustup;
mod scc;
mod sftp;
mod shortcuts;
mod softwareupdate;
mod sqlite3;
mod st2;
mod stepzen;
mod subl;
mod sysctl;
mod tailscale;
mod tccutil;
mod template_filters;
mod terragrunt;
mod tfenv;
mod tfsec;
mod tokei;
mod trex;
mod trivy;
mod ts_node;
mod tsc;
mod turbo;
mod vite;
mod vr;
mod vsce;
mod vultr_cli;
mod wasm_bindgen;
mod watson;
mod wd;
mod wifi_password;
mod xcodeproj;
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
        git::hub_generator(),
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
        ni::generator(),
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
        python::generator(),
        python::python3_generator(),
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
        ansible_playbook::generator(),
        apt::generator(),
        assimp::generator(),
        autojump::generator(),
        bat::generator(),
        black::generator(),
        checkov::generator(),
        chown::generator(),
        copilot::generator(),
        cordova::generator(),
        deno::generator(),
        deployctl::generator(),
        deta::generator(),
        direnv::generator(),
        dtm::generator(),
        eb::generator(),
        elm::generator(),
        elm_review::generator(),
        eslint::generator(),
        expo::generator(),
        expo_cli::generator(),
        ffmpeg::generator(),
        fig_teams::generator(),
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
        ns::tns_generator(),
        ns::nativescript_generator(),
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
        st2::generator(),
        stepzen::generator(),
        sysctl::generator(),
        tailscale::generator(),
        tccutil::generator(),
        terragrunt::generator(),
        tfenv::generator(),
        tfsec::generator(),
        tokei::generator(),
        trex::generator(),
        trivy::generator(),
        ts_node::generator(),
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
        cloudflared::generator(),
        dd::generator(),
        degit::generator(),
        elixir::generator(),
        esbuild::generator(),
        file::generator(),
        git_flow::generator(),
        iex::generator(),
        java::generator(),
        julia::generator(),
        osascript::generator(),
        pdfunite::generator(),
        pm2::generator(),
        pod::generator(),
        rollup::generator(),
        rscript::generator(),
        sqlite3::generator(),
        subl::generator(),
        tsc::generator(),
        wasm_bindgen::generator(),
        xcodeproj::generator(),
    ];

    HashMap::from_iter(command_signature_generators.map(Into::into))
}
