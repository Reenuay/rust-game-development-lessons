# CLAUDE.md

Guidance for Claude Code when working in this repository — a Russian-language
site teaching general programming through Rust + macroquad, deployed to
GitHub Pages under base `/rust-game-development-lessons/`.

## Writing lessons

- Simplest possible language. Lessons target beginners; avoid unexplained
  jargon and unnecessary technical detail that isn't the point of the
  current lesson (e.g. don't mention a type like `(f32, f32)` if the lesson
  isn't about types).
- Keep it concise — explain the one idea the lesson is about, not everything
  adjacent to it.
- **Never reference a past lesson without linking it.** No "in a previous
  lesson we..." — always a real markdown link to the specific lesson, e.g.
  `[«Клик мышью»](../mouse-click/)`. Every existing lesson already does
  this; keep it that way.
- **Never show the same code twice on one page.** If a lesson's demo is an
  interactive Svelte component with its own live code panel (see below),
  that panel *is* the code example — don't also add a static "## Код"
  fenced block with `title="src/main.rs"` above/below it repeating the same
  program. The static block predates live panels on some older lessons
  (`percentages.mdx`, `fractions.mdx` both had this until it was cleaned
  up) — if you spot another one, remove it the same way.
- **Comment every meaningful line of every lesson's code**, not just the
  lines introducing something new — a short inline `//` comment saying what
  that line does, right there. Do this even when the exact same thing is
  already explained in the prose or the "Что здесь происходит" bullets
  below: the point is that the comment is visible right at the line,
  without needing to cross-reference a paragraph elsewhere on the page.
  This is a deliberate exception to "don't comment obvious code": these are
  teaching examples, not production code. Boilerplate that's identical in
  every lesson (`clear_background(BLACK)`, `next_frame().await`,
  `use macroquad::prelude::*`, the `#[macroquad::main(...)]` line) doesn't
  need one. Apply this both to a lesson's static "## Код" block and to a
  live demo's `codeFor()` in its Svelte component — and, where a demo's
  underlying wasm crate (`examples/*/src/main.rs`) has the same logic
  (even wrapped in different WASM-export plumbing), mirror the same
  comments there too for consistency, though its export machinery itself
  stays uncommented since it's never shown to the reader.
- Structure for a lesson with an interactive live demo: intro prose →
  (optional) a short *conceptual* snippet if useful → `## Попробуй сам` →
  the demo component → `## Что здесь происходит` (bullet points). Look at
  `coordinates.mdx` or `offset.mdx` for the current reference shape.
- Structure for a lesson with a plain (non-interactive) `WasmCanvas` demo
  and no live code panel: intro prose → `## Код` (full program, replace
  `src/main.rs`, `cargo run`) → the demo → `## Что здесь происходит`. This
  is correct here because the static block is the *only* representation of
  the code — see `mouse-click.mdx`, `movement.mdx`, `else.mdx`.

## The Expressive Code asset trap

Live code panels (`CodePanel.svelte`) reuse Starlight's real Expressive
Code CSS classes (`.expressive-code`, `.frame`, `.ec-line`, ...) instead of
a hand-rolled copy, so they're pixel-identical to the site's real static
code blocks (this was tried the other way once — a self-contained,
independently-styled panel — and reverted because a hand-copied
approximation never looked quite right: wrong corners, wrong copy-button
position, wrong spacing).

The cost: **Astro's Expressive Code integration only injects its
stylesheet and copy-button script into a page that has at least one real,
statically-rendered ```code``` fence for it to process.** This is how the
`rehype-expressive-code` plugin works — it scans the page's markdown AST at
build time and does nothing at all if it finds zero fenced code blocks;
there is no config flag to force it on unconditionally (checked the
plugin's source — `rehype-expressive-code/dist/index.js`,
`astro-expressive-code/dist/index.js`. no such option exists). A page that
renders a `<Code />` component instead of a markdown fence doesn't help
either — that component only emits per-block syntax-color styles, not the
frame CSS or the copy-button JS module.

**Practical rule: every `.mdx` file that embeds a live demo using
`CodePanel.svelte` must keep at least one real, static ```code``` fence
somewhere on the page.** In practice this is usually already satisfied by
the lesson's own content, but check when editing a page down toward zero
static fences (e.g. removing a redundant static "Код" block per the rule
above — if it was the page's only fence, replace it with a small,
non-redundant conceptual snippet instead of deleting it outright, the way
`offset.mdx` keeps a two-line formula example). This has broken silently
before — twice (`relative-coordinates.mdx`, then `offset.mdx`) — with the
symptom being an unstyled, unhighlighted code panel and a broken-looking
copy button, nothing more obvious than that. Verify by checking the built
page for `<link ... ec.*.css>` / `<script ... ec.*.js>` if in doubt.

## Verification

- Before considering any change done: build (`npm run build`), preview
  (`astro preview`), and check the actual result with Playwright in a real
  browser — screenshots and/or computed-style assertions, not just "it
  should work." This applies to visuals, interactivity (hover/focus/click),
  and console errors. Don't claim something works without having looked at
  it.
- Clean up scratch test scripts, screenshots, and build artifacts
  (`dist`, `.astro`, `public/wasm-examples`, `examples/target`) before
  committing — none of that belongs in the repo.
- Harmless pre-existing console noise to ignore: `register_plugin is not
  defined` (macroquad plugin warning) and 404s for wasm assets not built
  locally in a given verification pass.

## Reuse over duplication

Shared building blocks already exist for common needs — use them instead of
copy-pasting between demo components:
- `src/lib/rustHighlight.js` — Shiki setup, `markedCode()` /
  `valueMarkTransformer()` for field-highlighted live code, shared
  highlighter loading.
- `src/components/NumberField.svelte`, `CodePanel.svelte`, `WasmCanvas.svelte`
  — shared input, code panel, and canvas embed components.
- `src/styles/pulse.css` — the shared `[data-pulse]` hover/focus glow
  animation (an attribute, not a class — see the comment in that file for
  why: a `class` attribute on a Shiki token span defeats Expressive Code's
  own token-coloring CSS, permanently, even after the class is removed
  again).

Before adding a new demo's worth of state/logic, check whether an existing
pattern (e.g. `AbsoluteOffsetDemo.svelte` for a WASM-exported-value demo)
already solves the same shape of problem.

## Git workflow

- Work directly on `main`. No feature branches, no PRs — this has been the
  workflow throughout the project.
- After pushing, don't schedule a deploy check or poll GitHub Actions —
  the user watches the deploy themselves.
- Rebuild wasm examples locally (`cargo build --release --target
  wasm32-unknown-unknown --manifest-path examples/Cargo.toml`) and copy
  into `public/wasm-examples/<name>/` the same way `.github/workflows/deploy.yml`
  does, whenever verifying a demo that depends on a wasm crate — the CI
  workflow picks up new crates automatically via a `for dir in examples/*/`
  loop, so a new example just needs adding to `examples/Cargo.toml`'s
  `members` list.
