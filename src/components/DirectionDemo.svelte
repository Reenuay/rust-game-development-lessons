<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let length = $state(200);
  let highlightField = $state(null);
  let built = $derived(codeFor(length));
  let code = $derived(built.code);
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(200).code));

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

  function codeFor(length) {
    return markedCode([
      `use macroquad::prelude::*;

// Длина одна и та же на каждом кадре, поэтому это константа
// (как в «Константах»), а не переменная внутри loop.
const LENGTH: f32 = `,
      { field: 'length', value: formatFloat(length) },
      `;

#[macroquad::main("Направление")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Неподвижный центр.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Расстояние от центра до курсора (как в «Расстоянии»).
        let dx = mouse_x - center_x;
        let dy = mouse_y - center_y;
        let distance = (dx * dx + dy * dy).sqrt();

        // Нормированное направление — та же пара чисел, но длиной ровно 1.
        let direction_x = dx / distance;
        let direction_y = dy / distance;

        // Растягиваем направление до нужной длины.
        let end_x = center_x + direction_x * LENGTH;
        let end_y = center_y + direction_y * LENGTH;

        // Белая точка — неподвижный центр.
        draw_circle(center_x, center_y, 10.0, WHITE);
        // Жёлтый луч — направление на курсор, заданной длины.
        draw_line(center_x, center_y, end_x, end_y, 4.0, YELLOW);

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

  function applyLength() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_length) return;
    exports.set_length(length);
    ready = true;
  }

  // The <input min max> attributes only style the spinner buttons and
  // mark the field :invalid — they don't stop someone from typing 5000
  // or -50 directly. Clamp for real here instead.
  function clampLength(value) {
    return Math.min(900, Math.max(0, value));
  }

  $effect(() => {
    length = clampLength(length);
    applyLength();
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
      applyLength();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="direction-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Белая точка — неподвижный центр. Жёлтый луч из неё всегда смотрит
    на курсор, а вот его длина не зависит от того, далеко мышь или
    близко — она задаётся полем ниже:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="length"
      bind:value={length}
      min="0"
      max="900"
      step="10"
      onfocus={() => (highlightField = 'length')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .direction-demo {
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
