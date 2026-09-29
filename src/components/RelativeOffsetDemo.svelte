<script>
  import { createHighlighterCore } from 'shiki/core';
  import { createJavaScriptRegexEngine } from 'shiki/engine/javascript';
  import WasmCanvas from './WasmCanvas.svelte';

  let { name, width = 720, height = 540 } = $props();

  let radius = $state(400);
  let percent = $state(0.5);
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(400, 0.5)));
  let copyCode = $derived(codeFor(radius, percent).replace(/\n/g, '\x7f'));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  // Rust needs a decimal point in a float literal — a bare "1" where f32
  // is expected doesn't compile. Whole numbers like 1 or 0 need ".0"
  // appended; anything already with a fractional part is already a
  // valid literal as-is.
  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

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

  function codeFor(radius, percent) {
    return `use macroquad::prelude::*;

#[macroquad::main("Относительные координаты")]
async fn main() {
    loop {
        clear_background(BLACK);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        draw_circle(center_x, center_y, ${formatFloat(radius)}, DARKBLUE);
        draw_circle(center_x + ${formatFloat(radius)} * ${formatFloat(percent)}, center_y, 40.0, YELLOW);

        next_frame().await;
    }
}
`;
  }

  // Same shaping as CoordinatesDemo's live snippet — matches Expressive
  // Code's DOM/CSS exactly instead of a hand-rolled lookalike.
  const ecShapeTransformer = {
    pre(node) {
      node.properties = { 'data-language': 'rust' };
    },
    code(node) {
      node.properties = {};
    },
    line(node) {
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
    postprocess(html) {
      return html.replace(/\n(?=<div class="ec-line")/g, '');
    },
  };

  function renderCode() {
    if (!highlighter) return;
    codeHtml = highlighter.codeToHtml(codeFor(radius, percent), {
      lang: 'rust',
      themes: { '0': 'night-owl', '1': 'night-owl-light' },
      defaultColor: false,
      cssVariablePrefix: '--',
      transformers: [ecShapeTransformer],
    });
  }

  function applyPosition() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_radius || !exports?.set_percent) return;
    exports.set_radius(radius);
    exports.set_percent(percent);
    ready = true;
  }

  // Same 900 ceiling as the absolute-coordinates demo, so the big
  // circle always fits on screen.
  function clampRadius(value) {
    return Math.min(900, Math.max(50, value));
  }

  function clampPercent(value) {
    return Math.min(1, Math.max(0, value));
  }

  $effect(() => {
    radius = clampRadius(radius);
    percent = clampPercent(percent);
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

<div class="relative-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Теперь смещение жёлтого кружка — это радиус <code>R</code>,
    умноженный на долю <code>P</code> от 0 до 1. Меняй <code>R</code> и
    <code>P</code> — кружок всегда остаётся где-то между центром и
    краем большого круга, каким бы ни был радиус:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <label>
      <span class="label-text">R</span>
      <input type="number" bind:value={radius} min="50" max="900" step="10" />
    </label>
    <label>
      <span class="label-text">P</span>
      <input type="number" bind:value={percent} min="0" max="1" step="0.01" />
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
  .relative-demo {
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
    /* Plain hairline borders on the inputs were too subtle to notice
       against the page — a friend of the site owner's couldn't spot
       them at a glance. The theme's own accent blue reads as an
       obvious "this is interactive" cue without introducing a new
       color. */
    border: 1.5px solid var(--sl-color-accent);
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
