# Follow-up Prompt Template

Use this template to construct follow-up prompts for Oz runs that need to verify generators with missing or invalid screenshots. Fill in the `[BRACKETED]` placeholders.

---

We've been working on [TICKET_DESCRIPTION] in the command-signatures repo, in the [BRANCH_NAME] branch. Now we need to validate all the generators edited in this branch by providing screenshots of them in use and putting them in a comment on the existing PR ([PR_URL]).

The following generators still need valid screenshots showing they produce meaningful completions output:
[GENERATOR_LIST — one item per line, with a brief note about what each generator should produce. For example:
- `kubeconfig_contexts_generator` — should show context names from kubeconfig
- `namespaces_generator` — should show Kubernetes namespace completions]

[PRIOR_VERIFICATION_CONTEXT — describe what was already verified and what's missing. For example: "A prior attempt verified the --kubeconfig flag generator, but the KUBECONFIG environment variable and $KUBECONFIG shell variable methods still need verification." Or: "No valid screenshots were found for any of the generators listed above."]

First, read the add-command-spec skill in command-signatures/.agents/skills/add-command-spec/ to understand the full submission requirements including screenshot upload.

Set up a testing environment with the command installed and configured to produce meaningful results. Do that before attempting to run a local Warp build — the generators need real data to complete against (e.g. running services, existing configs, populated directories).

Use the test-local-warp skill in command-signatures/.agents/skills/test-local-warp/ to patch the command-signatures change into the local warp-internal build. Then, use a local Warp build to test each generator listed above. To trigger the completions menu, press the Tab key. We are NOT testing autocomplete (ghost text) — we are testing completions, which are dropdown menus that appear next to the cursor. Take a screenshot to show each generator invocation working and attach it as a comment on the PR; your work will not be accepted without it.

Each generator must have its own screenshot showing real, non-trivial output. Empty completions menus, zero results, or generic placeholders are not acceptable — the screenshot must show at least one generator-produced completion entry.
