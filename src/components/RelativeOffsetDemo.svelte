<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let radius = $state(400);
  let percent = $state(0.5);
  let code = $derived(codeFor(radius, percent));
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(400, 0.5)));

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

  function renderCode() {
    if (!highlighter) return;
    codeHtml = highlightRust(highlighter, code);
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

<div class="relative-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Теперь смещение жёлтого кружка — это радиус <code>R</code>,
    умноженный на долю <code>P</code> от 0 до 1. Меняй <code>R</code> и
    <code>P</code> — кружок всегда остаётся где-то между центром и
    краем большого круга, каким бы ни был радиус:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField label="R" bind:value={radius} min="50" max="900" step="10" />
    <NumberField label="P" bind:value={percent} min="0" max="1" step="0.01" />
  </div>

  <CodePanel html={codeHtml} {code} />
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
</style>
