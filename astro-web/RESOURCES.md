# Astro.js Web Framework — Resources

Curated, high-trust sources. Knowledge in lessons is drawn from these; verify any
assertion against them before teaching. All are the official Astro documentation.

## Core concepts

- [Why Astro?](https://docs.astro.build/en/concepts/why-astro/) — content-focused,
  server-first, zero-JS-by-default; the performance claim; `.astro` as a superset
  of HTML; "opt in to complexity".
- [Islands architecture](https://docs.astro.build/en/concepts/islands/) — static
  HTML by default, client directives control hydration, parallel loading, server
  islands (`server:defer`).

## Authoring

- [Components](https://docs.astro.build/en/basics/astro-components/) — the
  `.astro` format, the frontmatter fence, props, slots, HTML components.
- [Layouts](https://docs.astro.build/en/basics/layouts/) — reusable page shells,
  slots, composition.
- [Routing](https://docs.astro.build/en/guides/routing/) — file-based routes,
  dynamic `[param]`, `getStaticPaths`, middleware, endpoints.
- [Styles & CSS](https://docs.astro.build/en/guides/styling/) — scoped styles,
  global styles, imports, preprocessors.
- [Images](https://docs.astro.build/en/guides/images/) — `<Image>` / `<Picture>`,
  the `public/` folder, and the image service.
- [Front-end frameworks](https://docs.astro.build/en/guides/framework-components/)
  — using React / Preact / Vue / Svelte components and the renderers.

## Content & data

- [Content collections](https://docs.astro.build/en/guides/content-collections/) —
  collections, zod schemas, `getCollection` / `getEntry`, `render()` / `<Content />`.
- [Data fetching](https://docs.astro.build/en/guides/data-fetching/) — fetching at
  build vs request time.
- [Endpoints](https://docs.astro.build/en/guides/endpoints/) — API routes returning
  a `Response`.

## Rendering & deploy

- [On-demand rendering](https://docs.astro.build/en/guides/on-demand-rendering/) —
  `output`, `export const prerender = false`, adapters, `Astro.request` /
  `Astro.response`.
- [View transitions](https://docs.astro.build/en/guides/view-transitions/) and
  [Prefetch](https://docs.astro.build/en/guides/prefetch/) — client navigation.
- [Deployment](https://docs.astro.build/en/guides/deploy/) — hosting a static or
  server-rendered Astro site.

## Notes on reliability

- Astro evolves quickly; the docs' version, config option names, and default
  `output` behavior change across major versions (e.g. v5 vs v7). Teach the
  pattern and cite the current guide for specifics.
- If a deep doc URL 404s in a fetch tool, open it in a browser — the canonical
  paths above resolve.