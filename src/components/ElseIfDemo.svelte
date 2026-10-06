<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let barWidth = $state(400);
  let highlightField = $state(null);
  let built = $derived(codeFor(barWidth));
  let code = $derived(built.code);
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(400).code));

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

  function codeFor(barWidth) {
    return markedCode([
      `use macroquad::prelude::*;

// Здоровье в начале и его максимум.
const MAX_HEALTH: f32 = 100.0;
// Сколько здоровья отнимает один клик.
const DAMAGE: f32 = 10.0;
// Выше какой доли здоровья полоска зелёная.
const GOOD_HEALTH: f32 = 0.6;
// Выше какой доли — жёлтая, а ниже — красная.
const LOW_HEALTH: f32 = 0.3;

// Левый верхний угол полоски, её высота и длина.
const BAR_X: f32 = 20.0;
const BAR_Y: f32 = 20.0;
const BAR_HEIGHT: f32 = 40.0;
const BAR_WIDTH: f32 = `,
      { field: 'bar_width', value: formatFloat(barWidth) },
      `;

#[macroquad::main("else if")]
async fn main() {
    // Текущее здоровье — сначала полное.
    let mut health = MAX_HEALTH;

    loop {
        clear_background(BLACK);

        // Кнопка мыши только что нажата.
        if is_mouse_button_pressed(MouseButton::Left) {
            if health <= 0.0 {
                // Здоровья уже нет — возвращаем полное.
                health = MAX_HEALTH;
            } else {
                // Отнимаем урон. max(0.0) не даёт здоровью уйти ниже нуля.
                health = (health - DAMAGE).max(0.0);
            }
        }

        // Доля здоровья от 0 до 1.
        let fraction = health / MAX_HEALTH;

        // Цвет выбираем по очереди: много здоровья — зелёный, иначе
        // средне — жёлтый, иначе мало — красный.
        let color = if fraction > GOOD_HEALTH {
            GREEN
        } else if fraction > LOW_HEALTH {
            YELLOW
        } else {
            RED
        };

        // Серая подложка: вся полоска целиком.
        draw_rectangle(BAR_X, BAR_Y, BAR_WIDTH, BAR_HEIGHT, GRAY);
        // Цветная часть поверх неё: ширина — доля от всей полоски.
        draw_rectangle(BAR_X, BAR_Y, BAR_WIDTH * fraction, BAR_HEIGHT, color);

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

  function applyBarWidth() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_bar_width) return;
    exports.set_bar_width(barWidth);
    ready = true;
  }

  // The <input min max> attributes only style the spinner buttons and
  // mark the field :invalid — they don't stop someone from typing 5000
  // or -50 directly. Clamp for real here instead, same as the
  // percent/fraction demos do for their own bounded fields.
  function clampBarWidth(value) {
    return Math.min(700, Math.max(100, value));
  }

  $effect(() => {
    barWidth = clampBarWidth(barWidth);
    applyBarWidth();
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
      applyBarWidth();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="else-if-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Кликай по примеру: каждый клик отнимает здоровье. Когда оно
    кончится, следующий клик вернёт его полностью. Меняй длину полоски
    ниже:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="bar_width"
      bind:value={barWidth}
      min="100"
      max="700"
      step="10"
      onfocus={() => (highlightField = 'bar_width')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .else-if-demo {
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
