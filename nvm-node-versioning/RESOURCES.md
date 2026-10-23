# nvm & Node Version Management — Resources

Curated, high-trust sources. Knowledge in lessons is drawn from these; verify any
assertion against them before teaching.

## nvm

- [nvm on GitHub](https://github.com/nvm-sh/nvm) — Node Version Manager: a
  POSIX-compliant bash script to manage multiple active Node.js versions; install
  script, shell sourcing, core commands, `.nvmrc`, and platform notes (Apple
  Silicon, WSL).
- [nvm-windows](https://github.com/coreybutler/nvm-windows) — the separate Windows
  implementation (nvm itself is POSIX shells).

## Node.js releases

- [Node.js Release Working Group](https://github.com/nodejs/Release) — the release
  schedule: Current, Active LTS, Maintenance, End-of-Life, codenames, and dates.
- [Node.js downloads](https://nodejs.org/en/download) — where to get installers and
  the current/LTS marketing view.

## Reproducibility

- [Corepack](https://nodejs.org/api/corepack.html) — a zero-runtime-dependency
  bridge that pins a package manager via the `packageManager` field.
- [package.json (`engines`, `devEngines`)](https://docs.npmjs.com/cli/v10/configuring-npm/package-json)
  — declaring the Node/npm versions a project expects.
- [Semantic Versioning](https://semver.org/) — how version ranges like `^20` behave.

## Alternatives worth knowing

- [fnm](https://github.com/Schniz/fnm) — a fast, cross-platform version manager.
- [Volta](https://volta.sh/) — pins toolchains per project and is shell-agnostic.
- [asdf](https://asdf-vm.com/) — a multi-language version manager.
- [nodenv](https://github.com/nodenv/nodenv) — another Node-focused manager.

## Notes on reliability

- nvm is a shell function, so it only exists in shells that source it. A
  non-interactive shell (cron, some CI, IDE tasks) may not have `nvm` on `PATH`.
- On Apple Silicon, older Node versions (pre-16) may need Rosetta/`arch -x86_64`;
  the nvm README documents this.
- `engines` is advisory unless you enforce it (`engine-strict` or `devEngines`);
  do not assume npm blocks a mismatched runtime by default.