use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generators() -> Vec<CommandSignatureGenerators> {
    vec![
        CommandSignatureGenerators::new("amplify")
            .add_generator("fig_a6bda67c52de819c", Generator::script(CommandBuilder::single_command("amplify env list --json"), fig_parse::lines))
,
        CommandSignatureGenerators::new("ansible-doc")
            .add_generator("fig_2761c7896e2d0e9d", Generator::script(CommandBuilder::single_command("ansible-doc --list --json 2>/dev/null"), fig_parse::lines))
,
        CommandSignatureGenerators::new("apt")
            .add_generator("fig_9b1f20c5990857db", Generator::script(CommandBuilder::single_command("apt list --upgradable"), fig_parse::lines))
            .add_generator("fig_4344dead34cb612a", Generator::script(CommandBuilder::single_command("apt list --installed"), fig_parse::lines))
,
        CommandSignatureGenerators::new("assimp")
            .add_generator("fig_f457ea7f4a72816e", Generator::script(CommandBuilder::single_command("assimp listext"), fig_parse::lines))
            .add_generator("fig_a80032f600b7bb60", Generator::script(CommandBuilder::single_command("assimp listexport"), fig_parse::lines))
,
        CommandSignatureGenerators::new("bat")
            .add_generator("fig_9c0acb4e77fae486", Generator::script(CommandBuilder::single_command("bat --list-languages"), fig_parse::lines))
            .add_generator("fig_9e82e39409621fa1", Generator::script(CommandBuilder::single_command("bat --wrap unknow  2>&1 >/dev/null | grep possible"), fig_parse::lines))
            .add_generator("fig_2780d0d6a24942c4", Generator::script(CommandBuilder::single_command("bat --color unknow  2>&1 >/dev/null | grep possible"), fig_parse::lines))
            .add_generator("fig_304f0e27ddd65100", Generator::script(CommandBuilder::single_command("bat --italic-text unknow  2>&1 >/dev/null | grep possible"), fig_parse::lines))
            .add_generator("fig_b029eab1813245c8", Generator::script(CommandBuilder::single_command("bat --decorations unknow  2>&1 >/dev/null | grep possible"), fig_parse::lines))
            .add_generator("fig_65ad2f65cb860e59", Generator::script(CommandBuilder::single_command("bat --paging unknow  2>&1 >/dev/null | grep possible"), fig_parse::lines))
            .add_generator("fig_0e9fb7ceec9e850d", Generator::script(CommandBuilder::single_command("bat --list-themes"), fig_parse::lines))
,
        CommandSignatureGenerators::new("black")
            .add_generator("fig_1a3c4d9fa0ba1f8c", Generator::script(CommandBuilder::single_command("gh release list --repo psf/black"), fig_parse::lines))
,
        CommandSignatureGenerators::new("cargo")
            .add_generator("fig_2b50fd08d8bf8e58", Generator::script(CommandBuilder::single_command("cargo read-manifest"), fig_parse::lines))
,
        CommandSignatureGenerators::new("checkov")
            .add_generator("fig_52b8a3e3cbdf3a04", Generator::script(CommandBuilder::single_command("git branch --no-color"), fig_parse::lines))
,
        CommandSignatureGenerators::new("copilot")
            .add_generator("fig_0d5d21be574f6f04", Generator::script(CommandBuilder::single_command("cat copilot/.workspace"), fig_parse::lines))
,
        CommandSignatureGenerators::new("cordova")
            .add_generator("fig_d96319ff520ea7ef", Generator::script(CommandBuilder::single_command("cat package.json"), fig_parse::lines))
            .add_generator("fig_45362b6a49c5c7ca", Generator::script(CommandBuilder::single_command("cordova plugin list"), fig_parse::lines))
,
        CommandSignatureGenerators::new("deno")
            .add_generator("fig_d9673fa715023b00", Generator::script(CommandBuilder::single_command("\\find ~/.deno/bin -maxdepth 1 -perm -111 -type f"), fig_parse::lines))
            .add_generator("fig_ed5e717093b43fe5", Generator::script(CommandBuilder::single_command("deno lint --rules --json"), fig_parse::lines))
            .add_generator("fig_42aa434b06dcbb6f", Generator::script(CommandBuilder::single_command("curl -sL 'https://cdn.deno.land/deno/meta/versions.json'"), fig_parse::lines))
,
        CommandSignatureGenerators::new("deployctl")
            .add_generator("fig_88660498d1230dce", Generator::script(CommandBuilder::single_command("curl -sL 'https://cdn.deno.land/deploy/meta/versions.json'"), fig_parse::lines))
,
        CommandSignatureGenerators::new("deta")
            .add_generator("fig_a8f9886cfee9a30d", Generator::script(CommandBuilder::single_command("echo node12, node14, python3.7, python3.9"), fig_parse::lines))
,
        CommandSignatureGenerators::new("dtm")
            .add_generator("fig_368b1e31a9a30a95", Generator::script(CommandBuilder::single_command("dtm list plugins"), fig_parse::lines))
,
        CommandSignatureGenerators::new("eb")
            .add_generator("fig_bb72fc80a10d0df3", Generator::script(CommandBuilder::single_command("eb list"), fig_parse::lines))
,
        CommandSignatureGenerators::new("elm-review")
            .add_generator("fig_6ae1e504576d7b99", Generator::script(CommandBuilder::single_command("echo"), fig_parse::lines))
,
        CommandSignatureGenerators::new("elm")
            .add_generator("fig_988ac1395f607de2", Generator::script(CommandBuilder::single_command("curl -sH 'accept-encoding: gzip' https://package.elm-lang.org/search.json | gunzip"), fig_parse::lines))
,
        CommandSignatureGenerators::new("eslint")
            .add_generator("fig_63e211045aef6726", Generator::script(CommandBuilder::single_command("{ ls node_modules ; ls $(npm root -g) ; ls $(yarn global dir)/node_modules/ ; } | cat"), fig_parse::lines))
,
        CommandSignatureGenerators::new("expo-cli")
            .add_generator("fig_41013b42bf3f0010", Generator::script(CommandBuilder::single_command("sysctl -n hw.ncpu"), fig_parse::lines))
            .add_generator("fig_ca78611a7473d104", Generator::script(CommandBuilder::single_command("xcrun xctrace list devices"), fig_parse::lines))
            .add_generator("fig_5b53e359bb4cb629", Generator::script(CommandBuilder::single_command("xcodebuild -project ios/*.xcodeproj -list -json"), fig_parse::lines))
,
        CommandSignatureGenerators::new("expo")
            .add_generator("fig_41013b42bf3f0010", Generator::script(CommandBuilder::single_command("sysctl -n hw.ncpu"), fig_parse::lines))
            .add_generator("fig_ca78611a7473d104", Generator::script(CommandBuilder::single_command("xcrun xctrace list devices"), fig_parse::lines))
            .add_generator("fig_5b53e359bb4cb629", Generator::script(CommandBuilder::single_command("xcodebuild -project ios/*.xcodeproj -list -json"), fig_parse::lines))
,
        CommandSignatureGenerators::new("ffmpeg")
            .add_generator("fig_e62be4b99868cf0b", Generator::script(CommandBuilder::single_command("ffmpeg -devices"), fig_parse::lines))
            .add_generator("fig_7d9b4cd61ad25b0b", Generator::script(CommandBuilder::single_command("ffmpeg -codecs"), fig_parse::lines))
,
        CommandSignatureGenerators::new("fig-teams@latest")
            .add_generator("fig_a207d884bbe1fac9", Generator::script(CommandBuilder::single_command("npx -y fig-teams@latest teams ls --json"), fig_parse::lines))
,
        CommandSignatureGenerators::new("fisher")
            .add_generator("fig_4c4b18b661fd781e", Generator::script(CommandBuilder::single_command("fish -c 'fisher list'"), fig_parse::lines))
,
        CommandSignatureGenerators::new("fly")
            .add_generator("fig_67abb2a19f084c78", Generator::script(CommandBuilder::single_command("flyctl list orgs"), fig_parse::lines))
            .add_generator("fig_3bb10439301e0755", Generator::script(CommandBuilder::single_command("flyctl list apps"), fig_parse::lines))
,
        CommandSignatureGenerators::new("flyctl")
            .add_generator("fig_67abb2a19f084c78", Generator::script(CommandBuilder::single_command("flyctl list orgs"), fig_parse::lines))
            .add_generator("fig_3bb10439301e0755", Generator::script(CommandBuilder::single_command("flyctl list apps"), fig_parse::lines))
,
        CommandSignatureGenerators::new("fnm")
            .add_generator("fig_56d6996e21a5a5aa", Generator::script(CommandBuilder::single_command("fnm ls-remote"), fig_parse::lines))
            .add_generator("fig_0102eb24f016678e", Generator::script(CommandBuilder::single_command("fnm ls"), fig_parse::lines))
,
        CommandSignatureGenerators::new("fvm")
            .add_generator("fig_8dee91404fdc01f5", Generator::script(CommandBuilder::single_command("fvm releases"), fig_parse::lines))
,
        CommandSignatureGenerators::new("gh")
            .add_generator("fig_c9ac79fbac716e6f", Generator::script(CommandBuilder::single_command("gh alias list"), fig_parse::lines))
            .add_generator("fig_4abe07c48e12e163", Generator::script(CommandBuilder::single_command("gh pr list"), fig_parse::lines))
            .add_generator("fig_0e6e30cf5927f49c", Generator::script(CommandBuilder::single_command("git --no-optional-locks branch -r --no-color --sort=-committerdate"), fig_parse::lines))
            .add_generator("fig_e11e38577c2f8e38", Generator::script(CommandBuilder::single_command("gh api graphql --paginate -f query='query($endCursor: String) { viewer { repositories(first: 100, after: $endCursor) { nodes { isPrivate, nameWithOwner, description } pageInfo { hasNextPage endCursor }}}}' --jq '.data.viewer.repositories.nodes[]'"), fig_parse::lines))
,
        CommandSignatureGenerators::new("gpg")
            .add_generator("fig_16dafc81c97cb1fa", Generator::script(CommandBuilder::single_command("gpg --version"), fig_parse::lines))
,
        CommandSignatureGenerators::new("hexo")
            .add_generator("fig_50aabb68ed0f35c5", Generator::script(CommandBuilder::single_command("hexo list post | grep -E ^Draft"), fig_parse::lines))
,
        CommandSignatureGenerators::new("git")
            .add_generator("fig_3b10d93dcfe29807", Generator::script(CommandBuilder::single_command("git --no-optional-locks config --get-regexp '^alias.'"), fig_parse::lines))
            .add_generator("fig_f61f822cf9ea2a65", Generator::script(CommandBuilder::single_command("git --no-optional-locks diff --cached --name-only"), fig_parse::lines))
            .add_generator("fig_5595f7f38d82b874", Generator::script(CommandBuilder::single_command("git rev-list --all --oneline"), fig_parse::lines))
            .add_generator("fig_80b60dafb19712af", Generator::script(CommandBuilder::single_command("git --no-optional-locks log --oneline"), fig_parse::lines))
            .add_generator("fig_4a69b82e41edaf39", Generator::script(CommandBuilder::single_command("git config --get-regexp '.*'"), fig_parse::lines))
            .add_generator("fig_4e22624a863e51de", Generator::script(CommandBuilder::single_command("git --no-optional-locks branch --no-color --sort=-committerdate"), fig_parse::lines))
            .add_generator("fig_79af8d547f41374e", Generator::script(CommandBuilder::single_command("git --no-optional-locks status --short"), fig_parse::lines))
            .add_generator("fig_716f05141c440a5f", Generator::script(CommandBuilder::single_command("git --no-optional-locks remote -v"), fig_parse::lines))
            .add_generator("fig_2fb72c855e8985af", Generator::script(CommandBuilder::single_command("git --no-optional-locks stash list"), fig_parse::lines))
            .add_generator("fig_6bdc8dba50da70cf", Generator::script(CommandBuilder::single_command("git --no-optional-locks branch -a --no-color --sort=-committerdate"), fig_parse::lines))
            .add_generator("fig_485e822b9a19c3a6", Generator::script(CommandBuilder::single_command("git --no-optional-locks tag --list --sort=-committerdate"), fig_parse::lines))
,
        CommandSignatureGenerators::new("hugo")
            .add_generator("fig_6ed0a0ad0c7e1156", Generator::script(CommandBuilder::single_command("ls ./archetypes/"), fig_parse::lines))
,
        CommandSignatureGenerators::new("hyper")
            .add_generator("fig_201917a52ae9dbbb", Generator::script(CommandBuilder::single_command("hyper list"), fig_parse::lines))
,
        CommandSignatureGenerators::new("id")
            .add_generator("fig_63813e420cdc5054", Generator::script(CommandBuilder::single_command("dscl . -list /Users | grep -v '^_'"), fig_parse::lines))
,
        CommandSignatureGenerators::new("ignite-cli")
            .add_generator("fig_ed789b87a42ea71f", Generator::script(CommandBuilder::single_command("ls ignite/templates"), fig_parse::lines))
,
        CommandSignatureGenerators::new("autojump")
            .add_generator("fig_9a89528d46db86fc", Generator::script(CommandBuilder::single_command("cat \"$HOME\"/Library/autojump/autojump.txt"), fig_parse::lines))
,
        CommandSignatureGenerators::new("kool")
            .add_generator("fig_f3117c1d9e79445f", Generator::script(CommandBuilder::single_command("docker-compose config --services"), fig_parse::lines))
            .add_generator("fig_88c427559b19dc18", Generator::script(CommandBuilder::single_command("kool run --help"), fig_parse::lines))
,
        CommandSignatureGenerators::new("lerna")
            .add_generator("fig_019af1751ffa9ad0", Generator::script(CommandBuilder::single_command("lerna ls"), fig_parse::lines))
            .add_generator("fig_f5132077c28c3980", Generator::script(CommandBuilder::single_command("lerna list -p | while read p; do  \\cat $p/package.json && echo END done"), fig_parse::lines))
            .add_generator("fig_52b8a3e3cbdf3a04", Generator::script(CommandBuilder::single_command("git branch --no-color"), fig_parse::lines))
            .add_generator("fig_30dba532147823c4", Generator::script(CommandBuilder::single_command("git remote"), fig_parse::lines))
,
        CommandSignatureGenerators::new("limactl")
            .add_generator("fig_c6b690bcc9c5a620", Generator::script(CommandBuilder::single_command("limactl list --quiet"), fig_parse::lines))
,
        CommandSignatureGenerators::new("mackup")
            .add_generator("fig_42f99a1d346db6aa", Generator::script(CommandBuilder::single_command("mackup list"), fig_parse::lines))
,
        CommandSignatureGenerators::new("mdfind")
            .add_generator("fig_d2ebc96bde282fb3", Generator::script(CommandBuilder::single_command("ls -1A ~/Library/Saved\\ Searches/*.savedSearch"), fig_parse::lines))
,
        CommandSignatureGenerators::new("meteor")
            .add_generator("fig_2b0622c36af38fe3", Generator::script(CommandBuilder::single_command("meteor create --list"), fig_parse::lines))
            .add_generator("fig_d128bd75226ed13c", Generator::script(CommandBuilder::single_command("cat ./.meteor/packages"), fig_parse::lines))
            .add_generator("fig_7fc46205f5dc4d47", Generator::script(CommandBuilder::single_command("meteor list-platforms"), fig_parse::lines))
,
        CommandSignatureGenerators::new("mix")
            .add_generator("fig_ef03f0d608fe7620", Generator::script(CommandBuilder::single_command("mix help"), fig_parse::lines))
,
        CommandSignatureGenerators::new("mosh")
            .add_generator("fig_70606b1a8c4e6453", Generator::script(CommandBuilder::single_command("cat ~/.ssh/known_hosts"), fig_parse::lines))
            .add_generator("fig_9dd5b5d832e7bf8d", Generator::script(CommandBuilder::single_command("cat ~/.ssh/config"), fig_parse::lines))
,
        CommandSignatureGenerators::new("n")
            .add_generator("fig_82f23ed48ca70c3e", Generator::script(CommandBuilder::single_command("n lsr --all"), fig_parse::lines))
,
        CommandSignatureGenerators::new("ns")
            .add_generator("fig_bd277ba8b8c4ab5b", Generator::script(CommandBuilder::single_command("curl https://api.github.com/repos/NativeScript/nativescript-app-templates/contents/packages"), fig_parse::lines))
,
        CommandSignatureGenerators::new("networkquality")
            .add_generator("fig_ba1f83fc4e9b0299", Generator::script(CommandBuilder::single_command("networksetup -listallhardwareports"), fig_parse::lines))
,
        CommandSignatureGenerators::new("npx")
            .add_generator("fig_8092d1a4b0d1ca29", Generator::script(CommandBuilder::single_command("until [[ -d node_modules/ ]] || [[ $PWD = '/' ]]; do cd ..; done; ls -1 node_modules/.bin/"), fig_parse::lines))
,
        CommandSignatureGenerators::new("nr")
            .add_generator("fig_9bb017c6db958ccb", Generator::script(CommandBuilder::single_command("until [[ -f package.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat package.json"), fig_parse::lines))
,
        CommandSignatureGenerators::new("okteto")
            .add_generator("fig_b5630da12bc5f958", Generator::script(CommandBuilder::single_command("okteto context list"), fig_parse::lines))
            .add_generator("fig_f7ac6b96ee508363", Generator::script(CommandBuilder::single_command("okteto namespace list"), fig_parse::lines))
,
        CommandSignatureGenerators::new("op")
            .add_generator("fig_7f3a8dadbab7c7ee", Generator::script(CommandBuilder::single_command("op account list --format json"), fig_parse::lines))
,
        CommandSignatureGenerators::new("open")
            .add_generator("fig_51126c9d6185b363", Generator::script(CommandBuilder::single_command("mdfind kMDItemContentTypeTree=com.apple.application-bundle -onlyin /"), fig_parse::lines))
            .add_generator("fig_b5e99242fa244718", Generator::script(CommandBuilder::single_command("mdfind kMDItemContentTypeTree=com.apple.application-bundle -onlyin / | while read line; do echo $(mdls -name kMDItemCFBundleIdentifier -r \"$line\") $line; done"), fig_parse::lines))
,
        CommandSignatureGenerators::new("pandoc")
            .add_generator("fig_6a82badd3be4ac01", Generator::script(CommandBuilder::single_command("pandoc --list-input-formats"), fig_parse::lines))
            .add_generator("fig_53812bf771b1d781", Generator::script(CommandBuilder::single_command("pandoc --list-output-formats"), fig_parse::lines))
            .add_generator("fig_61a52e243fb5ef99", Generator::script(CommandBuilder::single_command("pandoc --list-input-formats && pandoc --list-output-formats"), fig_parse::lines))
,
        CommandSignatureGenerators::new("pnpm")
            .add_generator("fig_9bb017c6db958ccb", Generator::script(CommandBuilder::single_command("until [[ -f package.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat package.json"), fig_parse::lines))
            .add_generator("fig_52b8a3e3cbdf3a04", Generator::script(CommandBuilder::single_command("git branch --no-color"), fig_parse::lines))
,
        CommandSignatureGenerators::new("pre-commit")
            .add_generator("fig_6bdc8dba50da70cf", Generator::script(CommandBuilder::single_command("git --no-optional-locks branch -a --no-color --sort=-committerdate"), fig_parse::lines))
            .add_generator("fig_4e22624a863e51de", Generator::script(CommandBuilder::single_command("git --no-optional-locks branch --no-color --sort=-committerdate"), fig_parse::lines))
            .add_generator("fig_5595f7f38d82b874", Generator::script(CommandBuilder::single_command("git rev-list --all --oneline"), fig_parse::lines))
            .add_generator("fig_716f05141c440a5f", Generator::script(CommandBuilder::single_command("git --no-optional-locks remote -v"), fig_parse::lines))
            .add_generator("fig_c9fd775c10b61ba0", Generator::script(CommandBuilder::single_command("cat .pre-commit-config.yaml"), fig_parse::lines))
,
        CommandSignatureGenerators::new("projj")
            .add_generator("fig_78b88fa3fdc56655", Generator::script(CommandBuilder::single_command("cat ~/.projj/cache.json"), fig_parse::lines))
            .add_generator("fig_dd2b81e9a864ba6d", Generator::script(CommandBuilder::single_command("cat ~/.projj/config.json"), fig_parse::lines))
,
        CommandSignatureGenerators::new("quickmail")
            .add_generator("fig_ddedede53fa5ef07", Generator::script(CommandBuilder::single_command("quickmail template listall"), fig_parse::lines))
,
        CommandSignatureGenerators::new("r")
            .add_generator("fig_ea9273c002d89c3b", Generator::script(CommandBuilder::single_command("Rscript -e '.libPaths()'"), fig_parse::lines))
,
        CommandSignatureGenerators::new("rancher")
            .add_generator("fig_674f752b1e190e22", Generator::script(CommandBuilder::single_command("rancher server ls"), fig_parse::lines))
,
        CommandSignatureGenerators::new("rclone")
            .add_generator("fig_302f5400485699e4", Generator::script(CommandBuilder::single_command("rclone listremotes"), fig_parse::lines))
,
        CommandSignatureGenerators::new("redwood")
            .add_generator("fig_396d80903f237678", Generator::script(CommandBuilder::single_command("until [[ -f redwood.toml ]] || [[ $PWD = '/' ]]; do cd ..; done; ls -1p scripts/"), fig_parse::lines))
,
        CommandSignatureGenerators::new("robot")
            .add_generator("fig_4ee42619cc04c73a", Generator::script(CommandBuilder::single_command("for i in $(find -E . -regex \".*.robot\" -type f); do cat -s $i ; done"), fig_parse::lines))
,
        CommandSignatureGenerators::new("rush")
            .add_generator("fig_e2342085e7f60878", Generator::script(CommandBuilder::single_command("until [[ -f rush.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat rush.json"), fig_parse::lines))
,
        CommandSignatureGenerators::new("rushx")
            .add_generator("fig_9bb017c6db958ccb", Generator::script(CommandBuilder::single_command("until [[ -f package.json ]] || [[ $PWD = '/' ]]; do cd ..; done; cat package.json"), fig_parse::lines))
,
        CommandSignatureGenerators::new("rustup")
            .add_generator("fig_311ec124efa2920a", Generator::script(CommandBuilder::single_command("find $(rustup docs --path | sed -e \"s|index\\.html|std|\") $(rustup docs --path | sed -e \"s|index\\.html|alloc|\") $(rustup docs --path | sed -e \"s|index\\.html|core|\") | grep \"\\.html\" | sed -E -e \"s|^(.*)/html/||\" -e \"s|\\.html||\" -e \"s|/|::|g\" -e \"s/constant\\.|trait\\.|struct\\.|macro\\.|fn\\.|keyword\\.|primitive\\.|type\\.|enum\\.|union\\.|traitalias\\.|::index$|^(.*)::all$//\" -e \"/^$/d\""), fig_parse::lines))
,
        CommandSignatureGenerators::new("scc")
            .add_generator("fig_7fca5e0c65e10c10", Generator::script(CommandBuilder::single_command("scc --languages"), fig_parse::lines))
,
        CommandSignatureGenerators::new("sftp")
            .add_generator("fig_70606b1a8c4e6453", Generator::script(CommandBuilder::single_command("cat ~/.ssh/known_hosts"), fig_parse::lines))
            .add_generator("fig_9dd5b5d832e7bf8d", Generator::script(CommandBuilder::single_command("cat ~/.ssh/config"), fig_parse::lines))
,
        CommandSignatureGenerators::new("shortcuts")
            .add_generator("fig_7aa42dcd1aee28f4", Generator::script(CommandBuilder::single_command("shortcuts list"), fig_parse::lines))
            .add_generator("fig_ee29ece79bef9b0f", Generator::script(CommandBuilder::single_command("shortcuts list --folders"), fig_parse::lines))
,
        CommandSignatureGenerators::new("softwareupdate")
            .add_generator("fig_a76756b62c8d576b", Generator::script(CommandBuilder::single_command("softwareupdate --list"), fig_parse::lines))
,
        CommandSignatureGenerators::new("stepzen")
            .add_generator("fig_a408a5e463729f62", Generator::script(CommandBuilder::single_command("stepzen list schemas"), fig_parse::lines))
            .add_generator("fig_afd777452a5ae6ad", Generator::script(CommandBuilder::single_command("curl https://api.github.com/repos/steprz/stepzen-schemas/contents"), fig_parse::lines))
,
        CommandSignatureGenerators::new("sysctl")
            .add_generator("fig_68b2fb427a5a3fee", Generator::script(CommandBuilder::single_command("sysctl -A -N"), fig_parse::lines))
,
        CommandSignatureGenerators::new("tailscale")
            .add_generator("fig_7f8d83b9c7d73c83", Generator::script(CommandBuilder::single_command("tailscale status --json"), fig_parse::lines))
,
        CommandSignatureGenerators::new("tccutil")
            .add_generator("fig_1e1fdbc4122f7861", Generator::script(CommandBuilder::single_command("mdfind kMDItemContentTypeTree=com.apple.application-bundle -onlyin /Applications | while read line; do echo $(mdls -name kMDItemCFBundleIdentifier -r \"$line\") $line; done"), fig_parse::lines))
,
        CommandSignatureGenerators::new("terragrunt")
            .add_generator("fig_13299139dd3a48c4", Generator::script(CommandBuilder::single_command("terragrunt state list"), fig_parse::lines))
            .add_generator("fig_8319b5ce43cff700", Generator::script(CommandBuilder::single_command("terragrunt workspace list"), fig_parse::lines))
,
        CommandSignatureGenerators::new("tfenv")
            .add_generator("fig_ede31bef6477adf1", Generator::script(CommandBuilder::single_command("tfenv list-remote"), fig_parse::lines))
            .add_generator("fig_eec33909c6c01629", Generator::script(CommandBuilder::single_command("tfenv list"), fig_parse::lines))
,
        CommandSignatureGenerators::new("tfsec")
            .add_generator("fig_7b4e416b3425cb8f", Generator::script(CommandBuilder::single_command("terraform workspace list"), fig_parse::lines))
,
        CommandSignatureGenerators::new("tokei")
            .add_generator("fig_609f69d8957c7be8", Generator::script(CommandBuilder::single_command("tokei --languages"), fig_parse::lines))
,
        CommandSignatureGenerators::new("trex")
            .add_generator("fig_189afcae6f128955", Generator::script(CommandBuilder::single_command("cat import_map.json"), fig_parse::lines))
            .add_generator("fig_8b97abd6534774c8", Generator::script(CommandBuilder::single_command("cat run.json"), fig_parse::lines))
,
        CommandSignatureGenerators::new("turbo")
            .add_generator("fig_40a40a435b66aa08", Generator::script(CommandBuilder::single_command("until [[ ( -f turbo.json || $PWD = '/' ) ]]; do cd ..; done; cat turbo.json"), fig_parse::lines))
            .add_generator("fig_6bdc8dba50da70cf", Generator::script(CommandBuilder::single_command("git --no-optional-locks branch -a --no-color --sort=-committerdate"), fig_parse::lines))
,
        CommandSignatureGenerators::new("vite")
            .add_generator("fig_c1fc2c418b970865", Generator::script(CommandBuilder::single_command("\\ls -l1A.env.*"), fig_parse::lines))
,
        CommandSignatureGenerators::new("vr")
            .add_generator("fig_7a28b9920d79855c", Generator::script(CommandBuilder::single_command("NO_COLOR=1 vr"), fig_parse::lines))
,
        CommandSignatureGenerators::new("vsce")
            .add_generator("fig_6bdc8dba50da70cf", Generator::script(CommandBuilder::single_command("git --no-optional-locks branch -a --no-color --sort=-committerdate"), fig_parse::lines))
,
        CommandSignatureGenerators::new("vultr-cli")
            .add_generator("fig_6dbd3b541dded6b5", Generator::script(CommandBuilder::single_command("vultr-cli instance list"), fig_parse::lines))
,
        CommandSignatureGenerators::new("watson")
            .add_generator("fig_861c2808151d45be", Generator::script(CommandBuilder::single_command("watson projects"), fig_parse::lines))
            .add_generator("fig_f8dcad28a84e3857", Generator::script(CommandBuilder::single_command("watson tags"), fig_parse::lines))
            .add_generator("fig_4ad54aa09fe5e753", Generator::script(CommandBuilder::single_command("watson log --json --reverse"), fig_parse::lines))
,
        CommandSignatureGenerators::new("wd")
            .add_generator("fig_bb17e57a5a4aeda3", Generator::script(CommandBuilder::single_command("cat ~/.warprc"), fig_parse::lines))
,
        CommandSignatureGenerators::new("wifi-password")
            .add_generator("fig_d33532774d410cf3", Generator::script(CommandBuilder::single_command("networksetup -listallhardwareports | awk '/Wi-Fi/{getline; print $2}' | xargs networksetup -listpreferredwirelessnetworks"), fig_parse::lines))
,
        CommandSignatureGenerators::new("yo")
            .add_generator("fig_aead680bbb4fb775", Generator::script(CommandBuilder::single_command("yo --generators"), fig_parse::lines))
,
        CommandSignatureGenerators::new("youtube-dl")
            .add_generator("fig_c40ef97662880110", Generator::script(CommandBuilder::single_command("pbpaste"), fig_parse::lines))
,
    ]
}
