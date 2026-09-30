<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import RangeSlider from './RangeSlider.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  // Slider and number field both bind to this same value, so dragging
  // one and typing in the other stay in sync automatically.
  let factor = $state(0.08);
  let highlightField = $state(null);
  let built = $derived(codeFor(factor));
  let code = $derived(built.code);
  let codeHtml = $state(plainCodeHtml(codeFor(0.08).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatFactor(n) {
    return n.toFixed(2);
  }

  function codeFor(factor) {
    return markedCode([
      `use macroquad::prelude::*;

// Точка — как в уроке «Точка и прямоугольник».
struct Point {
    x: f32,
    y: f32,
}

// Линейная интерполяция (lerp) — как в уроке «Линейная интерполяция».
fn lerp(from: Point, to: Point, t: f32) -> Point {
    Point {
        x: from.x + (to.x - from.x) * t,
        y: from.y + (to.y - from.y) * t,
    }
}

#[macroquad::main("Плавная погоня")]
async fn main() {
    // Кружок стартует в центре экрана. mut и вне loop — позиция должна
    // помнить прошлый кадр, а не сбрасываться каждый раз.
    let mut position = Point { x: screen_width() / 2.0, y: screen_height() / 2.0 };

    loop {
        clear_background(BLACK);

        // Курсор — цель, к которой каждый кадр чуть-чуть приближаемся заново.
        let (mouse_x, mouse_y) = mouse_position();
        let mouse = Point { x: mouse_x, y: mouse_y };

        // Каждый кадр сдвигаемся на factor от текущего расстояния до
        // курсора: далеко от цели шаг большой, у цели — маленький.
        let factor: f32 = `,
      { field: 'factor', value: formatFactor(factor) },
      `;
        position = lerp(position, mouse, factor);

        draw_circle(position.x, position.y, 30.0, YELLOW);

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

  function applyFactor() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_factor) return;
    exports.set_factor(factor);
    ready = true;
  }

  // 0 would freeze the circle in place forever; 1 would teleport it
  // straight onto the cursor every frame with no easing at all.
  function clampFactor(value) {
    return Math.round(Math.min(0.5, Math.max(0.01, value)) * 100) / 100;
  }

  $effect(() => {
    factor = clampFactor(factor);
    applyFactor();
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
      applyFactor();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="smooth-chase-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Води мышью по демке — кружок плавно подъезжает следом, замедляясь у
    цели. Дёрни мышь резко в сторону — кружок тут же снова ускорится.
    Поменяй <code>factor</code> ниже:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <RangeSlider
      label="factor"
      bind:value={factor}
      min="0.01"
      max="0.5"
      step="0.01"
      onfocus={() => (highlightField = 'factor')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="factor"
      bind:value={factor}
      min="0.01"
      max="0.5"
      step="0.01"
      onfocus={() => (highlightField = 'factor')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .smooth-chase-demo {
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
