# Mission: nvm & Node Version Management Practices

## Why

Ben is a self-taught TypeScript / Node.js developer aiming for senior mastery,
driven by career growth and interviews. He works across several projects that each
expect a different Node.js version, and he keeps hitting the same class of bug:
a global Node upgrade breaks an old project, CI runs a different version than his
laptop, or a teammate's "works on my machine" turns out to be a different runtime.

He wants to master **version management**: use **nvm** fluently to install and
switch Node versions per project, pin versions reproducibly with `.nvmrc` and
`engines`, understand the **LTS vs Current** release cadence, keep global packages
straight, and make onboarding and CI deterministic.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain why a single global Node install fails across projects, and what
  "version drift" costs (reproducibility, CI parity, native modules).
- Describe what nvm actually is — a POSIX shell **function**, not a global binary —
  and how it switches `node` per shell via `PATH`.
- Install/update nvm and source it from the right shell rc file, then install and
  verify a Node version.
- Use the core commands confidently: `nvm install`, `use`, `ls`, `current`,
  `alias`, `default`, `which`, `exec`.
- Pin a project with **`.nvmrc`** and use `nvm use` / `--install-if-missing`, and
  explain auto-switching on directory change.
- Read the **Node release schedule**: Current, Active LTS, Maintenance, EOL, and
  codenames — and justify choosing LTS for production.
- Manage aliases, the default version, and global packages, including migrating
  globals with `--reinstall-packages-from`.
- Make a team reproducible: `engines`, `.nvmrc`, Corepack `packageManager`, and a
  CI matrix that matches development.
- Compare nvm with alternatives (fnm, Volta, nodenv, nvm-windows, asdf) and know
  the edge cases (non-interactive shells, Docker, Apple Silicon/Rosetta).
- Run a clean workflow: onboarding, switching, upgrading, and troubleshooting the
  common "command not found" / "wrong version" failures.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped (what `PATH` is, what
  a shell rc file is, what semver ranges mean) before nvm specifics.
- Node / TypeScript and Unix-shell mindset; examples read naturally in a terminal.
- Learns by doing — lessons use concrete shell commands, config files, and CI
  snippets.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in cited sources (the nvm repo, Node release schedule, Node
  and npm docs); claims beyond them are explicit about uncertainty.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- A full Unix shell tutorial; only `PATH` and rc-file mechanics needed for nvm.
- Deep npm/yarn/pnpm package-manager internals; Corepack is covered as the pinning
  mechanism.
- Building Node from source or advanced compile flags beyond a brief note.