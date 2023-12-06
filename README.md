# command-signatures

## Cargo Dependency Tips

This repo is for the `warp-command-signatures` crate that is used as a dependency in `warp-internal`. In `warp-internal/Cargo.toml` you should see a line like this under `[workspace.dependencies]`:

```
warp-command-signatures = { git = "ssh://git@github.com/warpdotdev/command-signatures.git", rev = "191f33ec01345b6fbabe06ebb73acf1a5ad062b9", default-features = false }
```

### Local Development

When testing changes in `command-signatures` you can update that dependency to point to a local path. That will look something like this, depending on your directory structure:

```
warp-command-signatures = { path = "../command-signatures/command-signatures", default-features = false }
```

Note that this `command-signatures` repo contains a Cargo workspace with two crates inside it and above we're telling `warp-internal` to get it's `warp-command-signatures` dependency from the `command-signatures` crate inside the `command-signatures` directory.

### Ready for Release

When your changes to `command-signatures` are complete, you can go back to the original `git` dependency definition in `warp-internal` and update the `rev` (revision) to point to the latest `command-signatures` commit you created. That's how you tell `warp-internal` to start using your latest changes.
