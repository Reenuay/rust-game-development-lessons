<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  // Fixed logical resolution of every demo's canvas — see wasm-template.html.
  const WORLD_WIDTH = 2560;
  const WORLD_HEIGHT = 1920;

  let x = $state(400);
  let y = $state(300);
  let rectWidth = $state(600);
  let rectHeight = $state(400);
  let highlightField = $state(null);
  let built = $derived(codeFor(x, y, rectWidth, rectHeight));
  let code = $derived(built.code);
  // Same seeding trick as the other WASM-exported-value demos: start the
  // code panel with plain text so it's never blank while Shiki loads.
  let codeHtml = $state(plainCodeHtml(codeFor(400, 300, 600, 400).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function codeFor(x, y, w, h) {
    return markedCode([
      `use macroquad::prelude::*;

#[macroquad::main("Прямоугольник")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Прямоугольник левым верхним углом в (x, y), размером w на h.
        draw_rectangle(`,
      { field: 'x', value: `${x}.0` },
      `, `,
      { field: 'y', value: `${y}.0` },
      `, `,
      { field: 'width', value: `${w}.0` },
      `, `,
      { field: 'height', value: `${h}.0` },
      `, YELLOW);

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

  function applyRect() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_x || !exports?.set_y || !exports?.set_width || !exports?.set_height) return;
    exports.set_x(x);
    exports.set_y(y);
    exports.set_width(rectWidth);
    exports.set_height(rectHeight);
    ready = true;
  }

  $effect(() => {
    // Re-run whenever any of the four fields changes.
    void x;
    void y;
    void rectWidth;
    void rectHeight;
    applyRect();
    renderCode();
  });

  $effect(() => {
    let cancelled = false;

    loadRustHighlighter().then((h) => {
      if (cancelled) return;
      highlighter = h;
      renderCode();
    });

    const readyPoll = setInterval(() => {
      if (ready) {
        clearInterval(readyPoll);
        return;
      }
      applyRect();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="rectangle-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Меняй <code>x</code>, <code>y</code>, <code>width</code> и
    <code>height</code> — прямоугольник подстраивается под все четыре
    сразу, а код ниже показывает, как это записать в Rust:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="x"
      bind:value={x}
      step="10"
      onfocus={() => (highlightField = 'x')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="y"
      bind:value={y}
      step="10"
      onfocus={() => (highlightField = 'y')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="width"
      bind:value={rectWidth}
      min="1"
      step="10"
      onfocus={() => (highlightField = 'width')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="height"
      bind:value={rectHeight}
      min="1"
      step="10"
      onfocus={() => (highlightField = 'height')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .rectangle-demo {
    margin-block: 1rem;
  }

  .demo-instructions {
    margin: 0.75rem 0;
  }

  .controls {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 1.5rem;
    margin: 0;
  }
</style>
