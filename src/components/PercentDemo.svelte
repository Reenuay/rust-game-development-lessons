<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let xPercent = $state(50);
  let yPercent = $state(50);
  let highlightField = $state(null);
  let built = $derived(codeFor(xPercent, yPercent));
  let code = $derived(built.code);
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(50, 50).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function codeFor(xPercent, yPercent) {
    return markedCode([
      `use macroquad::prelude::*;

#[macroquad::main("Проценты")]
async fn main() {
    loop {
        clear_background(BLACK);

        let x = screen_width() * `,
      { field: 'x', value: `${xPercent}.0` },
      ` / 100.0;
        let y = screen_height() * `,
      { field: 'y', value: `${yPercent}.0` },
      ` / 100.0;

        draw_circle(x, y, 80.0, YELLOW);

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

  function applyPosition() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_x_percent || !exports?.set_y_percent) return;
    exports.set_x_percent(xPercent);
    exports.set_y_percent(yPercent);
    ready = true;
  }

  // The <input min max> attributes only style the spinner buttons and
  // mark the field :invalid — they don't stop someone from typing 200
  // directly, which would silently contradict the lesson text's own
  // "число от 0 до 100". Clamp for real here instead.
  function clamp(value) {
    return Math.min(100, Math.max(0, value));
  }

  $effect(() => {
    xPercent = clamp(xPercent);
    yPercent = clamp(yPercent);
    applyPosition();
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
      applyPosition();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="percent-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Меняй значения <code>x%</code> и <code>y%</code> ниже — кружок
    будет прыгать в новую точку, а код под ним покажет, как это
    записать в Rust, с уже подставленными числами:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="x%"
      bind:value={xPercent}
      min="0"
      max="100"
      step="1"
      onmouseenter={() => (highlightField = 'x')}
      onmouseleave={() => (highlightField = null)}
      onfocus={() => (highlightField = 'x')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="y%"
      bind:value={yPercent}
      min="0"
      max="100"
      step="1"
      onmouseenter={() => (highlightField = 'y')}
      onmouseleave={() => (highlightField = null)}
      onfocus={() => (highlightField = 'y')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .percent-demo {
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
