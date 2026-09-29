<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

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
  // Which field (if any) is hovered/focused right now — highlighted in
  // the code panel below via valueMarkTransformer's data-field markers.
  let highlightField = $state(null);
  let built = $derived(codeFor(x, y));
  let code = $derived(built.code);
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(x, y).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function codeFor(x, y) {
    return markedCode([
      `use macroquad::prelude::*;

#[macroquad::main("Координаты")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_circle(`,
      { field: 'x', value: `${x}.0` },
      `, `,
      { field: 'y', value: `${y}.0` },
      `, 40.0, YELLOW);

        next_frame().await;
    }
}
`,
    ]);
  }

  function renderCode() {
    if (!highlighter) return;
    codeHtml = highlightRust(highlighter, code, built.marks);
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

    loadRustHighlighter().then((h) => {
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
    <NumberField
      label="x"
      bind:value={x}
      step="10"
      onmouseenter={() => (highlightField = 'x')}
      onmouseleave={() => (highlightField = null)}
      onfocus={() => (highlightField = 'x')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="y"
      bind:value={y}
      step="10"
      onmouseenter={() => (highlightField = 'y')}
      onmouseleave={() => (highlightField = null)}
      onfocus={() => (highlightField = 'y')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
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
</style>
