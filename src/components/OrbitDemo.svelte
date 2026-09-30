<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import RangeSlider from './RangeSlider.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  // Slider and number field both bind to this same value, so dragging
  // one and typing in the other stay in sync automatically.
  let speed = $state(0.02);
  let highlightField = $state(null);
  let built = $derived(codeFor(speed));
  let code = $derived(built.code);
  let codeHtml = $state(plainCodeHtml(codeFor(0.02).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatSpeed(n) {
    return n.toFixed(3);
  }

  function codeFor(speed) {
    return markedCode([
      `use macroquad::prelude::*;

// Точка — как в уроке «Точка и прямоугольник».
struct Point {
    x: f32,
    y: f32,
}

// Точка на окружности радиусом radius вокруг center, под углом angle.
fn orbit(center: Point, radius: f32, angle: f32) -> Point {
    Point {
        x: center.x + radius * angle.cos(),
        y: center.y + radius * angle.sin(),
    }
}

#[macroquad::main("Вращение вокруг точки")]
async fn main() {
    // Угол растёт каждый кадр — mut и вне loop, чтобы помнить прошлый кадр.
    let mut angle: f32 = 0.0;

    loop {
        clear_background(BLACK);

        let center = Point { x: screen_width() / 2.0, y: screen_height() / 2.0 };

        // Большой кружок в центре — «планета».
        draw_circle(center.x, center.y, 40.0, DARKBLUE);

        // Маленький кружок на орбите вокруг центра.
        let point = orbit(center, 150.0, angle);
        draw_circle(point.x, point.y, 16.0, YELLOW);

        // Угол растёт на фиксированную угловую скорость каждый кадр.
        let speed: f32 = `,
      { field: 'speed', value: formatSpeed(speed) },
      `;
        angle += speed;

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

  function applySpeed() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_speed) return;
    exports.set_speed(speed);
    ready = true;
  }

  // 0 would freeze the orbiting circle in place; too high spins it fast
  // enough to stop reading as smooth rotation.
  function clampSpeed(value) {
    return Math.round(Math.min(0.1, Math.max(0.002, value)) * 1000) / 1000;
  }

  $effect(() => {
    speed = clampSpeed(speed);
    applySpeed();
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
      applySpeed();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="orbit-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Маленький кружок вращается вокруг большого с постоянной угловой
    скоростью. Поменяй <code>speed</code> ниже:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <RangeSlider
      label="speed"
      bind:value={speed}
      min="0.002"
      max="0.1"
      step="0.002"
      onfocus={() => (highlightField = 'speed')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="speed"
      bind:value={speed}
      min="0.002"
      max="0.1"
      step="0.002"
      onfocus={() => (highlightField = 'speed')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .orbit-demo {
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
