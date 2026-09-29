<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let xFraction = $state(0.5);
  let yFraction = $state(0.5);
  let highlightField = $state(null);
  let built = $derived(codeFor(xFraction, yFraction));
  let code = $derived(built.code);
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(0.5, 0.5).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  // Rust needs a decimal point in a float literal — a bare "1" where f32
  // is expected doesn't compile. Whole numbers like 1 or 0 need ".0"
  // appended; anything already with a fractional part (0.6, 0.25, ...)
  // is already a valid literal as-is.
  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function codeFor(xFraction, yFraction) {
    return markedCode([
      `use macroquad::prelude::*;

#[macroquad::main("Доля от 0 до 1")]
async fn main() {
    loop {
        clear_background(BLACK);

        let x = screen_width() * `,
      { field: 'x', value: formatFloat(xFraction) },
      `;
        let y = screen_height() * `,
      { field: 'y', value: formatFloat(yFraction) },
      `;

        draw_circle(x, y, 80.0, YELLOW);

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
    if (!exports?.set_x_fraction || !exports?.set_y_fraction) return;
    exports.set_x_fraction(xFraction);
    exports.set_y_fraction(yFraction);
    ready = true;
  }

  // Same reasoning as the percent demo's clamp: the <input min max>
  // attributes don't actually stop someone typing 5 or -1 here.
  function clamp(value) {
    return Math.min(1, Math.max(0, value));
  }

  $effect(() => {
    xFraction = clamp(xFraction);
    yFraction = clamp(yFraction);
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

<div class="fraction-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Меняй значения <code>x</code> и <code>y</code> ниже — от 0 до 1 —
    и смотри, как кружок занимает то же самое место, что и в прошлом
    уроке с процентами, только без промежуточного деления в коде:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="x"
      bind:value={xFraction}
      min="0"
      max="1"
      step="0.1"
      onfocus={() => (highlightField = 'x')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="y"
      bind:value={yFraction}
      min="0"
      max="1"
      step="0.1"
      onfocus={() => (highlightField = 'y')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .fraction-demo {
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
