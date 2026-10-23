# Starting Point: nvm & Node Version Management

The learner is a self-taught TypeScript / Node.js developer working toward senior
mastery who juggles several projects on different Node.js versions and keeps
hitting version drift: a global upgrade breaks an old project, CI runs a different
Node than his laptop, and teammates cannot reproduce his environment. He wants to
master version management with nvm: understand that nvm is a POSIX shell function,
install and source it, install/switch versions per shell, pin projects with
.nvmrc and engines, read the LTS/Current release schedule, manage aliases and
global packages, make onboarding and CI deterministic (including Corepack), compare
alternatives (fnm, Volta, nvm-windows, asdf), and troubleshoot the common "command
not found" and "wrong version" failures. This lesson set is designed to be
self-contained and independent of all other courses in the repository.