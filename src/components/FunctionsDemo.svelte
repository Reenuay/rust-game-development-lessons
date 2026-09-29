<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let speed = $state(8);
  let highlightField = $state(null);
  let built = $derived(codeFor(speed));
  let code = $derived(built.code);
  // Same seeding trick as the other WASM-exported-value demos: start the
  // code panel with plain text so it's never blank while Shiki loads.
  let codeHtml = $state(plainCodeHtml(codeFor(8).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function codeFor(speed) {
    return markedCode([
      `use macroquad::prelude::*;

// Скорость одна и та же на каждом кадре, поэтому это константа
// (как в «Константах»), а не переменная внутри loop.
const SPEED: f32 = `,
      { field: 'speed', value: formatFloat(speed) },
      `;

// Своя функция: по двум точкам считает направление от первой ко второй.
fn direction(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> (f32, f32) {
    // Вектор от начальной точки к конечной.
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    // Длина этого вектора — расстояние между точками.
    let distance = (dx * dx + dy * dy).sqrt();

    // Точки не совпадают — можно посчитать настоящее направление.
    if distance > 0.0 {
        // Делим на длину — получаем направление длиной ровно 1.
        (dx / distance, dy / distance)
    } else {
        // Точки совпали — направления нет, берём по умолчанию вправо.
        (1.0, 0.0)
    }
}

#[macroquad::main("Свои функции")]
async fn main() {
    // Кружок стартует в центре экрана. mut и вне loop — позиция
    // должна помнить прошлый кадр, а не сбрасываться каждый раз.
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Направление от кружка к курсору — вся математика внутри функции.
        let (direction_x, direction_y) = direction(x, y, mouse_x, mouse_y);

        // Точка + вектор: шаг фиксированной длины в эту сторону.
        x += direction_x * SPEED;
        y += direction_y * SPEED;

        draw_circle(x, y, 30.0, YELLOW);

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

  // <input min max> only guards the spinner arrows, not typed values —
  // clamp for real so speed can't go to 0 (circle would freeze) or fly
  // off far too fast to look like a game.
  function clampSpeed(value) {
    return Math.min(20, Math.max(2, value));
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

<div class="functions-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Та же погоня за курсором, что и раньше, но направление считает
    своя функция <code>direction</code>. Кружок бежит за мышью —
    поменяй скорость полем ниже:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="speed"
      bind:value={speed}
      min="2"
      max="20"
      step="1"
      onfocus={() => (highlightField = 'speed')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .functions-demo {
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
