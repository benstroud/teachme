# Git Merge, Fast-Forward, Pull & Rebase Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is a solo learner and has
opted out of communities, so that section is kept minimal and is not pushed.

## Knowledge

### Primary sources (cited throughout)

- [Pro Git Book — Basic Branching and Merging](https://git-scm.com/book/en/v2/Git-Branching-Basic-Branching-and-Merging)
  The classic tutorial: create a branch, the hotfix flow, fast-forward vs true
  merge, and resolving a conflict from the `<<<<<<< ==== >>>>>>>` markers.
  Use for: lessons 0001, 0004, 0005, 0006.

- [Pro Git Book — Rebasing](https://git-scm.com/book/en/v2/Git-Branching-Rebasing)
  The authoritative rebase explanation: the basic rebase, the diverged-branch
  scenario, when not to rebase, `git pull --rebase`, and the merge-vs-rebase
  philosophy.
  Use for: lessons 0007, 0008, 0009, 0010.

- [git-merge man page](https://git-scm.com/docs/git-merge)
  Fast-forward vs true merge, strategies, `--no-ff`, `--ff-only`, `--abort`,
  `--continue`, and how conflicts are presented and resolved.
  Use for: lessons 0003, 0004, 0005, 0006.

- [git-rebase man page](https://git-scm.com/docs/git-rebase)
  The `--onto` mode (transplanting a topic branch), the 3-argument form, and
  recovering from an upstream rebase.
  Use for: lessons 0007, 0008, 0010.

- [git-pull man page](https://git-scm.com/docs/git-pull)
  Pull = fetch + integrate; `--rebase`, `--ff-only`, `--no-rebase`, upstream
  config, and remote-tracking refs.
  Use for: lessons 0002, 0003, 0009.

### Supporting references

- [git-fetch man page](https://git-scm.com/docs/git-fetch) and [git-merge-base](https://git-scm.com/docs/git-merge-base) — mechanics behind remote refs and the merge base.
- [git-reflog](https://git-scm.com/docs/git-reflog) — the safety net for history rewrites.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. deciding a team's
  merge-vs-rebase policy), revisit — but do not push.
- Git's official [Community](https://git-scm.com/community) and Stack Overflow are
  natural places for real-world validation, offered only as pointers.

## Gaps

- Several git-scm pages are large and JS-heavy; lessons cite the canonical URLs and
  keep version-specific flag behavior conservative. Confirm flags against `git --version` on the learner's machine.
- Some behaviors (e.g. default pull strategy) evolve across Git versions; lessons
  present the well-established defaults and flag where to confirm on the learner's
  installation.