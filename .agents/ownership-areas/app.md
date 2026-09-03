# App

Ownership areas for the App team.

### Conversation and Session Restoration
- **Owners**: @seemeroland <roland@warp.dev>
- **Matches**: Restoring previous agent conversations, session persistence across restarts, conversation history loading

### Cloud-synced conversations
- **Owners**: @seemeroland <roland@warp.dev>
- **Matches**: Conversation transcript view, conversation details panel, conversation sharing

### Planning
- **Owners**: @seemeroland <roland@warp.dev>
- **Matches**: Agent plan creation and editing, plan approval workflows, plan syncing and sharing

### Autosuggestion
- **Owners**: @seemeroland <roland@warp.dev>
- **Matches**: Inline ghost text suggestions, autocomplete in the input editor (excluding command completions)

### Command Completions
- **Owners**: @acarl005 <andy@warp.dev>
- **Matches**: Command completions and suggestions triggered by slash or command entry, autocomplete specific to commands

### MCP (Model Context Protocol)
- **Owners**: @peicodes <pei@warp.dev>, @vkodithala <varoon@warp.dev>
- **Matches**: MCP server connections, MCP tools and resources, third-party integrations via MCP

### File-based MCPs
- **Owners**: @vkodithala <varoon@warp.dev>
- **Matches**: MCP servers configured via files (e.g. ~/.warp/.mcp.json), file-based MCP configuration and management

### Skills
- **Owners**: @peicodes <pei@warp.dev>, @vkodithala <varoon@warp.dev>
- **Matches**: Using, creating, finding agent skills.md files

### Conversation Management
- **Owners**: @harryalbert <harry@warp.dev>
- **Matches**: Managing multiple agent conversations, conversation list, switching between conversations, conversation search in the command palette, forking conversations

### Conversation Rewind
- **Owners**: @alokedesai <aloke@warp.dev>
- **Matches**: Rewind conversation to earlier state, including undoing code changes made by agent after rewind point

### Credit Usage Footer
- **Owners**: @harryalbert <harry@warp.dev>
- **Matches**: Footer at end of AI blocks showing credits used

### Input UI
- **Owners**: @harryalbert <harry@warp.dev>
- **Matches**: UI in the input area (not input text editor) including context window usage indicator, AI model selector, inline input menu infrastructure (not including behavior of specific inline input menus owned by someone else)

### Custom Models
- **Owners**: @danielpeng2 <daniel@warp.dev>, @dagmfactory <dagm@warp.dev>
- **Matches**: Custom model configuration and behavior for BYOK/custom inference endpoints, including custom endpoint model selection, custom model persistence in profiles, and custom model preferences. Excludes generic model selector UI unless the issue is specific to custom models or custom endpoints

### Natural Language Detection
- **Owners**: @evelyn-with-warp <evelyn@warp.dev>
- **Matches**: Classifying typed input as natural-language agent prompts vs. shell commands

### Image Attachment and Voice Input
- **Owners**: @Advait-M <advait@warp.dev>
- **Matches**: Attaching images as context to agent prompts, voice-to-text input

### Code Editor / LSP / Notebooks Editor
- **Owners**: @kevinyang372 <kevin@warp.dev>, @bnavetta <ben@warp.dev>
- **Matches**: Code diff viewer, language server features, notebook markdown editing and rendering, code block display, apply file diff tool calls in AI blocks

### @ Context and Slash Commands
- **Owners**: @moirahuang <moira@warp.dev>
- **Matches**: Attaching via @ context to agent prompt, slash command general UI and infra, and fallback when specific slash command is not owned by someone else

### Global Search / File Tree
- **Owners**: @moirahuang <moira@warp.dev>
- **Matches**: Global string search across files, the file tree sidebar, file navigation, file search in the command palette

### Onboarding
- **Owners**: @jefflloyd <jeff@warp.dev>
- **Matches**: Onboarding flow, onboarding callouts, first-time user experience, onboarding tooltips and walkthroughs

### Code Review
- **Owners**: @MaggieShan <maggie@warp.dev>
- **Matches**: Code review panel UI, adding and submitting code review comments

### Git Statistics
- **Owners**: @kevinyang372 <kevin@warp.dev>, @MaggieShan <maggie@warp.dev>
- **Matches**: Git status statistics surfaced in Warp features, including the git diff chip, branch status, upstream tracking counts, ahead/behind counts, and other git-derived repository statistics

### Git Operations
- **Owners**: @MaggieShan <maggie@warp.dev>
- **Matches**: Git operations performed from Warp, including the git dialog in the code review view and its actions (e.g. staging, committing, pushing, pulling, branch operations). Excludes git-derived statistics (see "Git Statistics") and code review comment UI (see "Code Review")

### Modality and Cloud Mode UI
- **Owners**: @zachbai <zachbai@warp.dev>, @MaggieShan <maggie@warp.dev>
- **Matches**: Switching between terminal, agent view, and cloud mode. Welcome and zero-state blocks in those views

### Passive Suggestions
- **Owners**: @Advait-M <advait@warp.dev>
- **Matches**: Passive code diff suggestions, passive unit tests, prompt suggestion UX

### Web Search
- **Owners**: @Advait-M <advait@warp.dev>
- **Matches**: Agent web search capability, web search and web fetch tool calls in AI blocks

### Long running command subagent
- **Owners**: @MaggieShan <maggie@warp.dev>, @vkodithala <varoon@warp.dev>, @zachbai <zachbai@warp.dev>
- **Matches**: Agent interaction with long running commands and pagers, agent stuck in long running command/pagers, CLI subagent, cancellations of shell commands run by agent

### Rules and WARP.md
- **Owners**: @MaggieShan <maggie@warp.dev>
- **Matches**: User-defined rules, WARP.md project rules, persistent agent instructions

### Warp Drive Objects and Infra
- **Owners**: @seemeroland <roland@warp.dev>
- **Matches**: Workflows, notebooks, prompts, environment variable collections, generic string objects, Warp Drive sync, cloud object management, ACLs, sharing dialog and permissions

### Codebase Context
- **Owners**: @kevinyang372 <kevin@warp.dev>
- **Matches**: Codebase indexing, semantic code search, repository context for agents, codebase search tool calls in AI blocks

### Universal Agent Support
- **Owners**: @liliwilson <lili@warp.dev>, @bnavetta <ben@warp.dev>
- **Matches**: Features related to working with third-party CLI coding agents (Claude Code, Codex, OpenCode, Gemini), including harness availability detection, harness setup flows, auth secrets for harnesses, harness-specific UI (e.g. codex modal, harness selector), and local harness launch infrastructure

### CLI Agent UI
- **Owners**: @alokedesai <aloke@warp.dev>
- **Matches**: UI footer shown in other agent CLIs like claude code, codex, gemini CLI

### /pr-comments
- **Owners**: @kevinyang372 <kevin@warp.dev>
- **Matches**: /pr-comments slash command

### Mac/Linux Platform Issues
- **Owners**: @alokedesai <aloke@warp.dev>
- **Matches**: macOS/Linux-specific bugs or integration issues

### Windows Platform Issues
- **Owners**: @acarl005 <andy@warp.dev>
- **Matches**: Windows-specific bugs or integration issues

### Command Palette
- **Owners**: @acarl005 <andy@warp.dev>
- **Matches**: Command palette search and ordering, data mixer logic, command palette UI (unless for an area specifically owned by someone else)

### Shell Compatibility
- **Owners**: @zachbai <zachbai@warp.dev>, @vorporeal <david@warp.dev>, @acarl005 <andy@warp.dev>
- **Matches**: Bash/zsh/fish/PowerShell support, shell integration, POSIX compliance issues

### Completions and Bootstrap
- **Owners**: @zachbai <zachbai@warp.dev>, @szgupta <suraj@warp.dev>, @alokedesai <aloke@warp.dev>
- **Matches**: Shell tab completions, first-run bootstrap, shell startup and prompt (PS1) rendering

### Performance Issues
- **Owners**: @alokedesai <aloke@warp.dev>
- **Matches**: Slow rendering, high CPU/memory usage, lag, responsiveness problems not relating to an existing area of ownership

### UI Framework
- **Owners**: @vorporeal <david@warp.dev>
- **Matches**: Core UI rendering framework, layout engine, text rendering, shader logic. Infrastructure-level only and not for feedback about any specific UI

### Settings and Keybindings
- **Owners**: @acarl005 <andy@warp.dev>
- **Matches**: Preferences panel, keyboard shortcuts, keybinding customization, settings UI infra

### Blocklist UX
- **Owners**: @zachbai <zachbai@warp.dev>, @MaggieShan <maggie@warp.dev>
- **Matches**: Appearance of shell command and AI blocks, buttons/interactions relating to blocks, requested command tool calls in AI blocks

### Find, Link Detection, Text Selection
- **Owners**: @seemeroland <roland@warp.dev>
- **Matches**: Find/search within terminal output or AI blocks, clickable links in output, text selection behavior

### Asynchronous Find (experimental)
- **Owners**: @vkodithala <varoon@warp.dev>
- **Matches**: Experimental background-thread implementation of find within terminal output or AI blocks. Will merge with "Find, Link Detection, Text Selection" once it becomes the default implementation.

### Warpifying
- **Owners**: @zachbai <zachbai@warp.dev>, @MaggieShan <maggie@warp.dev>, @kevinyang372 <kevin@warp.dev>, @moirahuang <moira@warp.dev>
- **Matches**: Converting raw terminal output from SSH sessions into Warp's block-based UI

### Warpify Footer
- **Owners**: @MaggieShan <maggie@warp.dev>
- **Matches**: Warpify footer UI, controls, and interactions

### Input Editor
- **Owners**: @vkodithala <varoon@warp.dev>
- **Matches**: The terminal command input area, text input, cursor behavior

### IME (Input Method Editor)
- **Owners**: @acarl005 <andy@warp.dev>, @vkodithala <varoon@warp.dev>
- **Matches**: Input method editor composition, including inputting CJK characters and other characters that require composition. Covers the notion of "marked text" in the editor and grid implementations (input editor, find bar, notebooks rich text editor, alt-screen apps like vim, and inside a block during a long running command).

### Vim Keybindings
- **Owners**: @acarl005 <andy@warp.dev>
- **Matches**: Warp's implementation of Vim keybindings or any settings pertaining to that.

### Vertical Tabs
- **Owners**: @johnturcoo <john.turco@warp.dev>
- **Matches**: Vertical tab bar, vertical tabs UI, tab sidebar, vtab

### Tab Groups
- **Owners**: @johnturcoo <john.turco@warp.dev>
- **Matches**: Grouping and organizing tabs, tab group creation and management, tab group UI

### Window, Tab, and Pane Management
- **Owners**: @vkodithala <varoon@warp.dev>
- **Matches**: Split panes, tab management, window resizing, multi-pane layouts

### Grep tool call
- **Owners**: @vkodithala <varoon@warp.dev>, @moirahuang <moira@warp.dev>, @szgupta <suraj@warp.dev>
- **Matches**: UI for grep tool used by agent

### Tab configs
- **Owners**: @moirahuang <moira@warp.dev>
- **Matches**: Create and edit existing tab configs to configure how you want a tab to be set up
