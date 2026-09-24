# Notes — Git Merge, Fast-Forward, Pull & Rebase

## Working notes (author/maintainer)

- Dark mode everywhere: deep warm paper, light ink, coral accent. Print query
  flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what a commit
  graph is, a merge base, a remote-tracking ref) before advanced material.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to the Pro Git book or a
  git-scm man page. Never assert Git behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.
- Many git-scm pages are large; cite canonical URLs, keep version-specific flag
  details conservative, and suggest the learner confirm against their `git
  --version`.

## Facts verified during authoring (2026-09)

- Git has two main ways to integrate changes between branches: merge and rebase.
  (Pro Git — Rebasing)
- A merge incorporates changes from the named commits (since the histories
  diverged) into the current branch; it is what `git pull` uses to integrate from
  another repository. (git-merge man page)
- When the current branch can be simply moved forward (the branch being merged is a
  descendant), a fast-forward happens and no new commit is created; otherwise a
  true (three-way) merge creates a merge commit. `--ff-only` fails if a
  fast-forward is impossible; `--no-ff` forces a merge commit even when it could
  fast-forward. (git-merge man page)
- Conflicts are presented with `<<<<<<< HEAD`, `=======`, and `>>>>>>>` markers;
  you resolve each block, then `git add` to mark resolved (staging = resolved), and
  `git commit` to finalize the merge. `git merge --abort` / `--continue` handle the
  in-progress merge. (Pro Git — Basic Merging)
- Rebase "reapplies commits on top of another base tip." A basic rebase takes the
  commits unique to your branch and replays them onto the tip of the target,
  producing new commits (new hashes). (git-rebase man page / Pro Git — Rebasing)
- The `--onto` form moves a range of commits: `git rebase --onto newbase upstream
  [branch]` transplants the commits that are unique to `upstream..branch` onto
  `newbase`. Its classic use is moving a topic branch or replaying just a subset
  (e.g. after a middle commit should be dropped). (git-rebase man page)
- `git pull` first runs `git fetch`, then integrates (merges by default) the
  upstream branch into the current one; `git pull --rebase` rebases instead. The
  `pull.rebase` config can make rebase the default. Fetch + merge can be done
  manually: `git fetch origin` then `git merge origin/next`. (git-pull man page)
- Golden rule: do not rebase commits that have been pushed to a shared branch and
  that others may have based work on; if you must, teammates should run
  `git pull --rebase`. You can rebase local work freely. Recovering from an
  upstream rebase uses `git rebase --onto subsystem subsystem@{1}` (the old tip via
  the reflog). (Pro Git — Rebasing)
- Merge vs rebase is a team philosophy: history as "what actually happened" (merge)
  vs "the story" (rebase). Many teams rebase local work before integrating, then
  merge published work. (Pro Git — Rebasing)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.