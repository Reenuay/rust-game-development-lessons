<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let seed = $state(42);
  let highlightField = $state(null);
  let built = $derived(codeFor(seed));
  let code = $derived(built.code);
  // Same seeding trick as the other WASM-exported-value demos: start the
  // code panel with plain text so it's never blank while Shiki loads.
  let codeHtml = $state(plainCodeHtml(codeFor(42).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function codeFor(seed) {
    return markedCode([
      `use macroquad::prelude::*;

// Зерно (seed): с него начинается набор случайных чисел.
const SEED: u64 = `,
      { field: 'seed', value: seed },
      `;
// Сколько кружков нарисовать.
const COUNT: usize = 20;

#[macroquad::main("Новая случайность")]
async fn main() {
    // Задаём зерно один раз, в самом начале, до первого gen_range.
    rand::srand(SEED);

    // Список случайных точек — сначала пустой.
    let mut points: Vec<(f32, f32)> = Vec::new();
    // Заполняем его один раз, при старте программы.
    for _ in 0..COUNT {
        // Случайная точка: x от 0 до ширины экрана, y от 0 до высоты.
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        // Кладём точку в список.
        points.push((x, y));
    }

    loop {
        clear_background(BLACK);

        // Рисуем кружок в каждой точке списка.
        for &(x, y) in &points {
            draw_circle(x, y, 30.0, YELLOW);
        }

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

  function applySeed() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_seed) return;
    exports.set_seed(seed);
    ready = true;
  }

  // The same number Rust's `date::now() as u64` gives: whole seconds.
  function seedFromTime() {
    seed = Math.floor(Date.now() / 1000);
  }

  // <input min max> only guards the spinner arrows, not typed values —
  // clamp for real. The wasm export takes a 32-bit whole number.
  function clampSeed(value) {
    return Math.min(4294967295, Math.max(0, Math.round(value) || 0));
  }

  $effect(() => {
    seed = clampSeed(seed);
    applySeed();
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
      applySeed();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="srand-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Меняй seed и смотри, как меняется расклад. Верни 42 — получишь тот
    же расклад, что был сначала. Кнопка берёт в качестве зерна текущее
    время:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <div class="seed-field">
      <NumberField
        label="seed"
        bind:value={seed}
        min="0"
        max="4294967295"
        step="1"
        onfocus={() => (highlightField = 'seed')}
        onblur={() => (highlightField = null)}
      />
    </div>
    <button type="button" class="time-button" onclick={seedFromTime}>Из времени</button>
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .srand-demo {
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

  /* A time-based seed has ten digits — the shared field is a bit
     narrow for that. */
  .seed-field :global(input) {
    width: 9.5rem;
  }

  /* Same look as the "Переставить" button in PythagorasDemo. */
  .time-button {
    margin: 0;
    height: 1.75rem;
    padding: 0 0.75rem;
    border: 1.5px solid var(--sl-color-accent);
    border-radius: 0.25rem;
    background: var(--sl-color-bg);
    color: var(--sl-color-text);
    font: inherit;
    font-family: var(--__sl-font-mono, ui-monospace, monospace);
    font-size: var(--sl-text-sm);
    line-height: 1.75rem;
    cursor: pointer;
  }

  .time-button:hover {
    background: var(--sl-color-gray-6);
  }
</style>
