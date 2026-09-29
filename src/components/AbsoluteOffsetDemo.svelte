<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let radius = $state(400);
  let offset = $state(200);
  let highlightField = $state(null);
  let built = $derived(codeFor(radius, offset));
  let code = $derived(built.code);
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(400, 200).code));

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

  function codeFor(radius, offset) {
    return markedCode([
      `use macroquad::prelude::*;

#[macroquad::main("Абсолютные координаты")]
async fn main() {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        draw_circle(center_x, center_y, `,
      { field: 'radius', value: formatFloat(radius) },
      `, DARKBLUE);
        draw_circle(center_x + `,
      { field: 'offset', value: formatFloat(offset) },
      `, center_y, 40.0, YELLOW);

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
    if (!exports?.set_radius || !exports?.set_offset) return;
    exports.set_radius(radius);
    exports.set_offset(offset);
    ready = true;
  }

  // The big circle has to stay on screen at every radius the input
  // allows — 900 keeps it inside the demo's 1920-tall world even at
  // dead center.
  function clampRadius(value) {
    return Math.min(900, Math.max(50, value));
  }

  // No clamp on the offset itself — a fixed pixel offset that ignores
  // how big the circle is (even ending up outside it) is exactly the
  // point being made here. Still need to *read* it directly here
  // though (not just inside applyPosition/renderCode): on the very
  // first run those two bail out early (no WASM exports / no
  // highlighter yet) before ever touching `offset`, so Svelte never
  // registers it as a dependency of this effect — changing the offset
  // field would then silently do nothing until radius changed too and
  // forced a re-run that finally read it.
  $effect(() => {
    radius = clampRadius(radius);
    void offset;
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

<div class="absolute-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Смещение жёлтого кружка от центра большого круга задано просто
    числом пикселей — это и есть абсолютные координаты. Сколько бы ты
    ни менял радиус <code>R</code> большого круга, жёлтый кружок
    остаётся на том же самом месте — а если радиус станет меньше
    смещения, кружок вообще окажется снаружи:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="R"
      bind:value={radius}
      min="50"
      max="900"
      step="10"
      onmouseenter={() => (highlightField = 'radius')}
      onmouseleave={() => (highlightField = null)}
      onfocus={() => (highlightField = 'radius')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="offset"
      bind:value={offset}
      step="10"
      onmouseenter={() => (highlightField = 'offset')}
      onmouseleave={() => (highlightField = null)}
      onfocus={() => (highlightField = 'offset')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .absolute-demo {
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
