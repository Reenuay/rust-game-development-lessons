<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import RangeSlider from './RangeSlider.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  // Slider and number field below both bind to this same value, so
  // dragging one and typing in the other stay in sync automatically.
  let t = $state(0.5);
  let highlightField = $state(null);
  let built = $derived(codeFor(t));
  let code = $derived(built.code);
  let codeHtml = $state(plainCodeHtml(codeFor(0.5).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatT(n) {
    return n.toFixed(2);
  }

  function codeFor(t) {
    return markedCode([
      `use macroquad::prelude::*;

// Точка — как в уроке «Точка и прямоугольник».
struct Point {
    x: f32,
    y: f32,
}

// Линейная интерполяция (lerp): точка на пути от from к to, где t — доля
// пройденного пути, от 0.0 (ещё в from) до 1.0 (уже в to).
fn lerp(from: Point, to: Point, t: f32) -> Point {
    Point {
        x: from.x + (to.x - from.x) * t,
        y: from.y + (to.y - from.y) * t,
    }
}

// t одна и та же на каждом кадре, поэтому это константа (как в уроке
// «Константы»), а не переменная внутри loop.
const T: f32 = `,
      { field: 't', value: formatT(t) },
      `;

#[macroquad::main("Линейная интерполяция")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Начало и конец отрезка — по умолчанию на одной высоте.
        let start = Point { x: screen_width() * 0.2, y: screen_height() / 2.0 };
        let end = Point { x: screen_width() * 0.8, y: screen_height() / 2.0 };

        // Сам отрезок.
        draw_line(start.x, start.y, end.x, end.y, 4.0, YELLOW);
        draw_circle(start.x, start.y, 10.0, WHITE);
        draw_circle(end.x, end.y, 10.0, WHITE);

        // Точка на отрезке, определяемая t.
        let point = lerp(start, end, T);
        draw_circle(point.x, point.y, 14.0, SKYBLUE);

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

  function applyT() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_t) return;
    exports.set_t(t);
    ready = true;
  }

  function clampT(value) {
    return Math.round(Math.min(1, Math.max(0, value)) * 100) / 100;
  }

  $effect(() => {
    t = clampT(t);
    applyT();
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
      applyT();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="lerp-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Двигай ползунок или впиши число — синяя точка едет по отрезку:
    при <code>t = 0</code> она в начале, при <code>t = 1</code> — в
    конце, а между ними — пропорционально доле пути:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <RangeSlider
      label="t"
      bind:value={t}
      min="0"
      max="1"
      step="0.01"
      onfocus={() => (highlightField = 't')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="t"
      bind:value={t}
      min="0"
      max="1"
      step="0.01"
      onfocus={() => (highlightField = 't')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .lerp-demo {
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
