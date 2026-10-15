# Notes — Astro.js Web Framework workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep slate paper, light ink, coral accent (Astro's warm
  brand hue). Print query flips back to light so pages print cleanly.
- Learner: self-taught TS/JS dev, senior-track, comes from React SPAs. Fill
  foundational gaps (what hydration is, why client JS costs so much, what server-
  first rendering means) before framework internals.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to an Astro doc. Never
  assert behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.

## Facts verified during authoring (2026-09)

- Astro is content-focused and server-first: pages render to HTML on the server
  (build or request time) and ship **zero client JavaScript by default**. Astro
  claims an Astro site can load ~40% faster with ~90% less JavaScript than the same
  site built with the most popular React framework. (docs: Why Astro)
- The `.astro` UI language is a superset of HTML: any valid HTML is valid Astro
  syntax; it borrows JSX-style expressions and scoped CSS by default. React,
  Preact, Svelte, Vue, Solid, and web components can all be used for UI.
  (docs: Why Astro)
- Islands architecture: most of the page is static HTML; only components with a
  client directive hydrate in the browser. Client directives include `client:load`
  (immediately), `client:idle`, `client:visible` (when scrolled into view), and
  `client:only`. Islands hydrate in isolation and can load in parallel.
  (docs: Islands architecture)
- A component's frontmatter is fenced by `---` at the top of the `.astro` file; it
  runs on the server/build and is not exposed to the browser. The template below it
  produces HTML, with `{ }` expressions and `Astro.props`. (docs: Components)
- Routing is file-based under `src/pages/`; dynamic routes use `[param].astro` and
  `getStaticPaths` for static builds. (docs: Routing)
- Content collections define a frontmatter schema with zod; `getCollection` /
  `getEntry` query them; `render()` / `<Content />` render Markdown/MDX content.
  (docs: Content collections)
- On-demand rendering: a page can opt out of static generation with
  `export const prerender = false`; the project uses an adapter (Node, Netlify,
  Vercel, Cloudflare) to run on demand. `Astro.request` / `Astro.response` and
  server endpoints (API routes) return a `Response`. (docs: On-demand rendering)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.

## Facts to verify before teaching

- Default `output` and config option names differ across Astro major versions;
  teach the pattern and cite the current guide.
- The 40% / 90% performance figure is Astro's own benchmark claim; present it as a
  manufacturer's claim, not an independent study.