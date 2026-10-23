# Notes — nvm & Node Version Management workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep slate paper, light ink, lime accent. Print query flips
  back to light so pages print cleanly.
- Learner: self-taught TS/Node dev, senior-track, comfortable in a Unix shell. Fill
  foundational gaps (PATH, shell rc files, semver ranges) before nvm specifics.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to an official source. Never
  assert behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.

## Facts verified during authoring (2026-09)

- nvm is a "POSIX-compliant bash script to manage multiple active node.js
  versions". (nvm repo) It is sourced into the shell (e.g. from `~/.bashrc` /
  `~/.zshrc`), and core commands include `nvm install`, `nvm use`, `nvm ls`,
  `nvm current`, `nvm alias`, `nvm default`, `nvm exec`, `nvm which`. (nvm README)
- nvm reads a project's `.nvmrc` for the version to use; `nvm use` accepts
  `--install-if-missing`. (nvm README)
- Node.js releases move through phases: Alpha, Current, then Active LTS, then
  Maintenance, and finally End-of-Life. Recent lines: 22.x "Jod" (Maintenance LTS,
  EOL 2027-04-30), 24.x "Krypton" (Active LTS, EOL 2028-04-30), 26.x Current (EOL
  2029-04-30). Active LTS lasts about 12 months, then Maintenance about 18 months
  (for modern lines), with dates subject to change. (nodejs/Release)
- Corepack is a zero-runtime-dependency Node.js script that lets you use Yarn, npm,
  and pnpm without installing them; the `packageManager` field in package.json pins
  which package manager (and version) a project uses. Corepack shipped with Node
  from 14.19.0 up to (but not including) 25.0.0. (Corepack docs)
- npm's package.json supports `engines` (e.g. `engines.node`) and the newer
  `devEngines` (runtime/packageManager with `onFail` of warn/error/ignore).
  (npm package.json docs)
- On Apple Silicon, older Node versions (pre-16) may need x86_64/Rosetta; the nvm
  README documents compiling with `--shared-zlib` in that scenario. (nvm README)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.

## Facts to verify before teaching

- Exact release-line dates and codenames change monthly; cite the live schedule.
- nvm version numbers change; cite the repo instead of a fixed tag.
- Enforcement of `engines` depends on `engine-strict`/`devEngines`; flag it rather
  than saying npm always blocks mismatches.