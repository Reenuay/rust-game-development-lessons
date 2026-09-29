<script>
  import { createHighlighterCore } from 'shiki/core';
  import { createJavaScriptRegexEngine } from 'shiki/engine/javascript';
  import WasmCanvas from './WasmCanvas.svelte';

  let { name, width = 720, height = 540 } = $props();

  // The demo's actual logical resolution inside the iframe — fixed in
  // wasm-template.html regardless of how big the demo is displayed here
  // (that's `width`/`height` above, used only for WasmCanvas's own CSS
  // box). Coordinates sent to the WASM module need to be in that fixed
  // space, not this display size, or they'd land off-center.
  const WORLD_WIDTH = 2560;
  const WORLD_HEIGHT = 1920;

  let x = $state(Math.round(WORLD_WIDTH / 2));
  let y = $state(Math.round(WORLD_HEIGHT / 2));
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(x, y)));
  // Expressive Code's copy button reads the text to copy from a
  // data-code attribute, encoding newlines as \x7f instead of literal
  // "\n" (HTML attribute values collapse literal newlines to spaces).
  let copyCode = $derived(codeFor(x, y).replace(/\n/g, '\x7f'));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function escapeHtml(s) {
    return s.replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);
  }

  // Same ec-line/code DOM shape as ecShapeTransformer below, but without
  // syntax colors — cheap to compute synchronously, unlike the real
  // highlighter.
  function plainCodeHtml(code) {
    const lines = code.replace(/\n$/, '').split('\n');
    const body = lines
      .map((line) => `<div class="ec-line"><div class="code">${line.length ? escapeHtml(line) : '\n'}</div></div>`)
      .join('');
    return `<pre data-language="rust"><code>${body}</code></pre>`;
  }

  function codeFor(x, y) {
    return `use macroquad::prelude::*;

#[macroquad::main("Координаты")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_circle(${x}.0, ${y}.0, 40.0, YELLOW);

        next_frame().await;
    }
}
`;
  }

  // Reshapes Shiki's default output into the same DOM shape Starlight's
  // Expressive Code uses for its static code blocks (.expressive-code >
  // .ec-line > .code), so this live snippet is styled by the exact same
  // sitewide CSS instead of a hand-rolled lookalike.
  const ecShapeTransformer = {
    pre(node) {
      node.properties = { 'data-language': 'rust' };
    },
    code(node) {
      node.properties = {};
    },
    line(node) {
      // A truly empty line has no children, and an empty block box has
      // no line box to size itself by — it collapses to 0 height. Give
      // it the same single "\n" text placeholder Expressive Code's own
      // empty lines have, which is enough to establish one line's worth
      // of height under the pre's white-space: pre.
      const children = node.children.length > 0 ? node.children : [{ type: 'text', value: '\n' }];
      return {
        type: 'element',
        tagName: 'div',
        properties: { class: 'ec-line' },
        children: [
          {
            type: 'element',
            tagName: 'div',
            properties: { class: 'code' },
            children,
          },
        ],
      };
    },
    // Shiki's default renderer joins line elements with a literal "\n"
    // text node (its usual per-line wrapper is an inline <span>, which
    // needs that newline to actually break the line). Our .ec-line divs
    // are block-level and don't need it, but inside <pre> that stray
    // newline is still whitespace:pre-preserved — showing up as a whole
    // extra blank line after every line, exactly doubling the vertical
    // rhythm the static code blocks have. Strip them.
    postprocess(html) {
      return html.replace(/\n(?=<div class="ec-line")/g, '');
    },
  };

  function renderCode() {
    if (!highlighter) return;
    // Two colors per token (--0 for dark, --1 for light) instead of one,
    // matching Expressive Code's own dual-theme output exactly (down to
    // the variable names) so its existing CSS picks the right one off
    // the site's <html data-theme> — no separate light/dark render pass
    // or theme-change listener needed here.
    codeHtml = highlighter.codeToHtml(codeFor(x, y), {
      lang: 'rust',
      themes: { '0': 'night-owl', '1': 'night-owl-light' },
      defaultColor: false,
      cssVariablePrefix: '--',
      transformers: [ecShapeTransformer],
    });
  }

  function applyPosition() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_x || !exports?.set_y) return;
    exports.set_x(x);
    exports.set_y(y);
    ready = true;
  }

  $effect(() => {
    // Re-run whenever x or y changes.
    void x;
    void y;
    applyPosition();
    renderCode();
  });

  $effect(() => {
    let cancelled = false;

    createHighlighterCore({
      themes: [import('@shikijs/themes/night-owl'), import('@shikijs/themes/night-owl-light')],
      langs: [import('@shikijs/langs/rust')],
      engine: createJavaScriptRegexEngine(),
    }).then((h) => {
      if (cancelled) return;
      highlighter = h;
      renderCode();
    });

    // The WASM module loads asynchronously inside the iframe; poll until
    // its exports are ready, then apply the current position once.
    const readyPoll = setInterval(() => {
      if (ready) {
        clearInterval(readyPoll);
        return;
      }
      applyPosition();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="coordinates-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Меняй значения <code>x</code> и <code>y</code> — кружок будет
    двигаться, а код ниже покажет, как это записать в Rust:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <label>
      <span class="label-text">x</span>
      <input type="number" bind:value={x} step="10" />
    </label>
    <label>
      <span class="label-text">y</span>
      <input type="number" bind:value={y} step="10" />
    </label>
  </div>

  <div class="expressive-code">
    <figure class="frame has-title not-content">
      <figcaption class="header"><span class="title">src/main.rs</span></figcaption>
      {@html codeHtml}
      <div class="copy">
        <div aria-live="polite"></div>
        <button title="Копировать" data-copied="Скопировано!" data-code={copyCode}><div></div></button>
      </div>
    </figure>
  </div>
</div>

<style>
  .coordinates-demo {
    margin-block: 1rem;
  }

  .demo-instructions {
    margin: 0.75rem 0;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    margin: 0;
  }

  .controls label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    /* Starlight's prose CSS adds margin-top between adjacent content
       elements (for normal paragraph/list spacing); it doesn't know
       these <label>s are a flex row, not stacked text, so it was
       pushing the second one down. */
    margin: 0;
    line-height: 1;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-sm);
    color: var(--sl-color-text);
  }

  .controls input {
    width: 5.5rem;
    height: 1.75rem;
    padding: 0 0.5rem;
    border: 1px solid var(--sl-color-hairline);
    border-radius: 0.25rem;
    background: var(--sl-color-bg);
    color: var(--sl-color-text);
    font: inherit;
    line-height: 1.75rem;
  }

  .expressive-code {
    margin-top: 0.75rem;
  }
</style>
