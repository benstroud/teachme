# Mission: Astro.js Web Framework

## Why

Ben is a self-taught TypeScript / JavaScript developer aiming for senior-level
front-end and full-stack mastery, driven by career growth and interviews. He has
built React single-page apps and knows their cost: lots of client JavaScript,
hydration, and a slow first paint for content-heavy pages. He keeps hearing that
**Astro** is the framework for content-driven websites, and he wants to genuinely
understand it — not just paste a starter template.

The goal is to be able to design and build a real Astro site: author `.astro`
components with server-only frontmatter, use file-based routing and layouts,
choose deliberately when a component becomes an interactive "island", style and
optimize assets, manage typed content collections, decide between static and
on-demand rendering with the right adapter, fetch data and expose API endpoints,
and ship a fast, accessible, SEO-friendly site.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain Astro's design: server-first, content-focused, **zero JavaScript by
  default**, and the **islands architecture** — and contrast it with a SPA
  framework's request-time hydration model.
- Author `.astro` components: the frontmatter script fence (`---`) that runs at
  build/server time and never ships to the browser, template expressions, props,
  and composition.
- Build pages with file-based routing: static routes, dynamic `[param].astro`
  routes, `getStaticPaths`, and how pages map to output files.
- Use layouts and slots (`<slot />`, named slots, fallback content) to compose
  reusable page shells.
- Make a component interactive deliberately with client directives
  (`client:load`, `client:idle`, `client:visible`, `client:only`) and UI framework
  components, and understand partial/selective hydration.
- Style with scoped `<style>` and global CSS, and handle static assets, the
  `public/` folder, and optimized images.
- Define and query **content collections**: a schema (zod), `getCollection` /
  `getEntry`, and rendering with `render()` / `<Content />`.
- Choose between static output and **on-demand rendering** (`export const
  prerender = false`), and pick an adapter (Node, Netlify, Vercel, Cloudflare).
- Fetch data at build or request time and write server **endpoints / API routes**
  that return a `Response`.
- Apply performance and shipping practices: prefetch, View Transitions, SEO /
  sitemap, and deploying to a static host.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples read naturally to a JS/TS developer.
- Learns by doing — lessons lean on reasoning about concrete component/page code
  and small snippets over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in cited sources (the official Astro documentation); claims
  beyond them are explicit about uncertainty.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- A full CSS/Tailwind or design-systems tour; styling is covered only as far as
  Astro's scoped styles, global styles, and asset pipeline.
- Deep content-CMS integrations; content collections and Markdown/MDX are covered
  at the pattern level.
- A complete deployment-provider matrix; the deploy lesson covers the model and a
  couple of representative hosts.