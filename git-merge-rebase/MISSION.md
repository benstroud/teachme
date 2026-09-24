# Mission: Git Merge, Fast-Forward, Pull & Rebase

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He uses Git daily for solo work,
but when history diverges — his local branch and the remote branch both move —
the integration commands (pull, merge, rebase, and the dreaded "rebase onto") are
where things get murky. He wants a precise mental model, not just memorized
commands: how Git's commit graph works, what a fast-forward is and why pull can
silently fast-forward or not, what kind of commit a merge creates, what a rebase
actually rewrites, and exactly what `--onto` does when you need to move a range of
commits onto a different base.

The goal is the ability to look at any divergent history, choose correctly between
merge and rebase (and between pull, pull --rebase, and fetch + merge), integrate
work safely, resolve conflicts, and — when history was rewritten — know how to
recover. This is a frequent senior-level interview and day-to-day topic.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Read a commit graph and identify branches, HEAD, ancestors/descendants, and the
  merge base of two branches.
- Explain what a fast-forward merge is, when it can happen, and why it creates no
  commit; use `--ff-only` and `--no-ff` with intent.
- Distinguish a true (three-way) merge and the merge commit it creates, and predict
  when one happens instead of a fast-forward.
- Pull the right way: fetch vs pull, remote-tracking refs (e.g. `origin/main`),
  upstream configuration, and the default merge that pull performs.
- Resolve merge conflicts from the `<<<<<<< ==== >>>>>>>` markers through
  `git add` and `git commit`, and abort or continue cleanly.
- Explain what a rebase does (replays commits onto a new base), why the commits get
  new SHAs, and how it differs philosophically from a merge.
- Use `git rebase` and the 3-argument `git rebase --onto newbase upstream [branch]`
  to move a range of commits onto a different base, including dropping a middle
  commit.
- Integrate upstream work with `git pull --rebase` and know when that is the right
  integration mode for a branch.
- State the golden rule — do not rebase commits others have based work on — and
  recover from a rewritten upstream using `--onto`, reflogs, and `upstream@{1}`.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Learns by doing — lessons lean on running git in a scratch repo, tracing graphs,
  and observing outputs (fast-forward, merge commit, conflicts) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in the official git-scm book and man pages; claims beyond
  them are explicit about uncertainty. It is history-integration focused, not a
  full Git tutorial.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- The full breadth of Git (staging internals, hooks, submodules, rerere, remote
  protocols) beyond what explain these five operations.
- GitHub/GitLab-specific automation (PR flows, merge queues), beyond noting that
  teams may enforce merge vs rebase policy.
- A GUI-tool guide; the course is command-line and mental-model focused.