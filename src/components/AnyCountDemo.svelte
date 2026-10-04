<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let count = $state(5);
  let highlightField = $state(null);
  let built = $derived(codeFor(count));
  let code = $derived(built.code);
  // Same seeding trick as the other WASM-exported-value demos: start the
  // code panel with plain text so it's never blank while Shiki loads.
  let codeHtml = $state(plainCodeHtml(codeFor(5).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function codeFor(count) {
    return markedCode([
      `use macroquad::prelude::*;

// Сколько кружков рисуем — можно поменять полем ниже.
const COUNT: usize = `,
      { field: 'count', value: `${count}` },
      `;
// Откуда начинаем и на сколько пикселей сдвигаем каждый следующий кружок —
// то же самое, что и в уроке «Цикл for».
const START_X: f32 = 60.0;
const STEP: f32 = 160.0;

#[macroquad::main("Сколько угодно кружков")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Сколько бы ни было кружков, цикл сам подстроится под COUNT.
        for i in 0..COUNT {
            let x = START_X + STEP * (i as f32);
            draw_circle(x, screen_height() / 2.0, 40.0, YELLOW);
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

  function applyCount() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_count) return;
    exports.set_count(count);
    ready = true;
  }

  // <input min max> only guards the spinner arrows, not typed values —
  // clamp for real. Min 1 (not 0 — zero circles would need its own
  // no-op case we're not covering here), max 16 — with the fixed
  // START_X/STEP, 16 circles still fit the demo's actual canvas width.
  function clampCount(value) {
    return Math.min(16, Math.max(1, Math.round(value)));
  }

  $effect(() => {
    count = clampCount(count);
    applyCount();
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
      applyCount();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="any-count-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Тот же цикл, что и в прошлом уроке, но количество кружков теперь
    можно менять полем ниже — код при этом не трогаем совсем,
    <code>for</code> сам подстраивается под новое число:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="count"
      bind:value={count}
      min="1"
      max="16"
      step="1"
      onfocus={() => (highlightField = 'count')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .any-count-demo {
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
