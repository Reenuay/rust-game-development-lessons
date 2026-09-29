<script>
  import { createHighlighterCore } from 'shiki/core';
  import { createJavaScriptRegexEngine } from 'shiki/engine/javascript';
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';

  let { name, width = 720, height = 540 } = $props();

  let xPercent = $state(50);
  let yPercent = $state(50);
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(50, 50)));
  let copyCode = $derived(codeFor(xPercent, yPercent).replace(/\n/g, '\x7f'));

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

  function codeFor(xPercent, yPercent) {
    return `use macroquad::prelude::*;

#[macroquad::main("Проценты")]
async fn main() {
    loop {
        clear_background(BLACK);

        let x = screen_width() * ${xPercent}.0 / 100.0;
        let y = screen_height() * ${yPercent}.0 / 100.0;

        draw_circle(x, y, 80.0, YELLOW);

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
    codeHtml = highlighter.codeToHtml(codeFor(xPercent, yPercent), {
      lang: 'rust',
      themes: { '0': 'night-owl', '1': 'night-owl-light' },
      defaultColor: false,
      cssVariablePrefix: '--',
      transformers: [ecShapeTransformer],
    });
  }

  function applyPosition() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_x_percent || !exports?.set_y_percent) return;
    exports.set_x_percent(xPercent);
    exports.set_y_percent(yPercent);
    ready = true;
  }

  // The <input min max> attributes only style the spinner buttons and
  // mark the field :invalid — they don't stop someone from typing 200
  // directly, which would silently contradict the lesson text's own
  // "число от 0 до 100". Clamp for real here instead.
  function clamp(value) {
    return Math.min(100, Math.max(0, value));
  }

  $effect(() => {
    xPercent = clamp(xPercent);
    yPercent = clamp(yPercent);
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

<div class="percent-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Меняй значения <code>x%</code> и <code>y%</code> ниже — кружок
    будет прыгать в новую точку, а код под ним покажет, как это
    записать в Rust, с уже подставленными числами:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField label="x%" bind:value={xPercent} min="0" max="100" step="1" />
    <NumberField label="y%" bind:value={yPercent} min="0" max="100" step="1" />
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
  .percent-demo {
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

  .expressive-code {
    margin-top: 0.75rem;
  }
</style>
