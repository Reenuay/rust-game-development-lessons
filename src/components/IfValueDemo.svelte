<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let radius = $state(300);
  let highlightField = $state(null);
  let built = $derived(codeFor(radius));
  let code = $derived(built.code);
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(300).code));

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

  function codeFor(radius) {
    return markedCode([
      `use macroquad::prelude::*;

// Радиус один и тот же на каждом кадре, поэтому это константа
// (как в уроке «Константы»), а не переменная внутри loop.
const RADIUS: f32 = `,
      { field: 'radius', value: formatFloat(radius) },
      `;

#[macroquad::main("if как значение")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Центр экрана — здесь же стоит турель.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Расстояние от турели до курсора (как в уроке «Расстояние»).
        let dx = mouse_x - center_x;
        let dy = mouse_y - center_y;
        let distance = (dx * dx + dy * dy).sqrt();

        // if с else выдаёт значение: что выбрала сработавшая ветка, то и
        // попадёт в color. Игрок внутри радиуса — красный, иначе синий.
        // После RED и BLUE точку с запятой не ставим.
        let color = if distance < RADIUS { RED } else { BLUE };

        // Контур — радиус обнаружения, того же цвета, что и турель.
        draw_circle_lines(center_x, center_y, RADIUS, 3.0, color);

        // Турель.
        draw_circle(center_x, center_y, 40.0, color);

        // Игрок — кружок под курсором.
        draw_circle(mouse_x, mouse_y, 25.0, WHITE);

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

  function applyRadius() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_radius) return;
    exports.set_radius(radius);
    ready = true;
  }

  // The <input min max> attributes only style the spinner buttons and
  // mark the field :invalid — they don't stop someone from typing 5000
  // or -50 directly. Clamp for real here instead, same as the
  // percent/fraction demos do for their own bounded fields.
  function clampRadius(value) {
    return Math.min(900, Math.max(50, value));
  }

  $effect(() => {
    radius = clampRadius(radius);
    applyRadius();
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
      applyRadius();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="if-value-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Белый кружок под курсором — игрок. Заведи его внутрь контура:
    и турель, и контур покраснеют разом, потому что цвет выбран один
    раз. Меняй радиус ниже:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="radius"
      bind:value={radius}
      min="50"
      max="900"
      step="10"
      onfocus={() => (highlightField = 'radius')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .if-value-demo {
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
