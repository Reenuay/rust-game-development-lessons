<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let headLength = $state(24);
  let headWidth = $state(12);
  let highlightField = $state(null);
  let built = $derived(codeFor(headLength, headWidth));
  let code = $derived(built.code);
  // Same seeding trick as the other WASM-exported-value demos: start the
  // code panel with plain text so it's never blank while Shiki loads.
  let codeHtml = $state(plainCodeHtml(codeFor(24, 12).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function codeFor(headLength, headWidth) {
    return markedCode([
      `use macroquad::prelude::*;

// Точка — как в уроке «Точка и прямоугольник».
struct Point {
    x: f32,
    y: f32,
}

// Рисует стрелку из from в to — линию с треугольным наконечником.
fn draw_arrow(from: Point, to: Point, head_length: f32, head_width: f32, color: Color) {
    // Направление и длина — как в «Направлении».
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let length = (dx * dx + dy * dy).sqrt();

    // Нулевой вектор не на что направить — как в «Делении на ноль».
    if length == 0.0 {
        return;
    }

    let direction_x = dx / length;
    let direction_y = dy / length;

    // Шаг назад от кончика вдоль направления.
    let back_x = to.x - direction_x * head_length;
    let back_y = to.y - direction_y * head_length;

    // Перпендикуляр к направлению — поворот на 90°, как в «Повороте на 90°».
    let perp_x = -direction_y;
    let perp_y = direction_x;

    // Два «крыла» наконечника — по разные стороны от back. Каждая
    // координата — своя переменная, чтобы было видно, как она
    // считается.
    let wing1_x = back_x + perp_x * head_width;
    let wing1_y = back_y + perp_y * head_width;
    let wing2_x = back_x - perp_x * head_width;
    let wing2_y = back_y - perp_y * head_width;

    // Тело стрелки — до back, а не до самого кончика, чтобы не вылезать
    // из-под наконечника.
    draw_line(from.x, from.y, back_x, back_y, 4.0, color);

    // draw_triangle() в macroquad хочет точки в виде Vec2, а не просто
    // x и y по отдельности — vec2() здесь просто упаковывает уже
    // готовые координаты.
    draw_triangle(
        vec2(to.x, to.y),
        vec2(wing1_x, wing1_y),
        vec2(wing2_x, wing2_y),
        color,
    );
}

#[macroquad::main("Стрелки")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Стрелка летит из центра экрана в курсор мыши.
        let center = Point { x: screen_width() / 2.0, y: screen_height() / 2.0 };
        let (mouse_x, mouse_y) = mouse_position();
        let cursor = Point { x: mouse_x, y: mouse_y };

        draw_arrow(center, cursor, `,
      { field: 'headLength', value: formatFloat(headLength) },
      `, `,
      { field: 'headWidth', value: formatFloat(headWidth) },
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

  function applyValues() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_head_length || !exports?.set_head_width) return;
    exports.set_head_length(headLength);
    exports.set_head_width(headWidth);
    ready = true;
  }

  // A head shorter than its own width stops reading as an arrow
  // (collapses into a blob) — and a head longer than the demo's own
  // radius would poke out the other side of the shaft.
  function clampHeadLength(value) {
    return Math.round(Math.min(60, Math.max(8, value)));
  }

  function clampHeadWidth(value) {
    return Math.round(Math.min(30, Math.max(4, value)));
  }

  $effect(() => {
    headLength = clampHeadLength(headLength);
    headWidth = clampHeadWidth(headWidth);
    applyValues();
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
      applyValues();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="arrow-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Стрелка летит из центра экрана прямо в курсор мыши (клик по демо
    переносит на него фокус). Поменяй <code>head_length</code> и
    <code>head_width</code> ниже:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="head_length"
      bind:value={headLength}
      min="8"
      max="60"
      step="1"
      onfocus={() => (highlightField = 'headLength')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="head_width"
      bind:value={headWidth}
      min="4"
      max="30"
      step="1"
      onfocus={() => (highlightField = 'headWidth')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .arrow-demo {
    margin-block: 1rem;
  }

  .demo-instructions {
    margin: 0.75rem 0;
  }

  .controls {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 1.5rem;
    margin: 0;
  }
</style>
