<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let offsetX = $state(0);
  let offsetY = $state(0);
  let highlightField = $state(null);
  let built = $derived(codeFor(offsetX, offsetY));
  let code = $derived(built.code);
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(0, 0).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  // Rust needs a decimal point in a float literal — a bare "1" where f32
  // is expected doesn't compile. Whole numbers like 1, 0 or -50 need
  // ".0" appended; anything already with a fractional part is already a
  // valid literal as-is.
  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function codeFor(offsetX, offsetY) {
    return markedCode([
      `use macroquad::prelude::*;

#[macroquad::main("Смещение")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Белая точка — сама позиция курсора, просто ориентир.
        draw_circle(mouse_x, mouse_y, 8.0, WHITE);

        // Жёлтый кружок — курсор плюс смещение.
        draw_circle(mouse_x + `,
      { field: 'offsetX', value: formatFloat(offsetX) },
      `, mouse_y + `,
      { field: 'offsetY', value: formatFloat(offsetY) },
      `, 40.0, YELLOW);

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

  function applyOffset() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_offset_x || !exports?.set_offset_y) return;
    exports.set_offset_x(offsetX);
    exports.set_offset_y(offsetY);
    ready = true;
  }

  $effect(() => {
    // Same "early return before reading state" trap as the absolute-
    // coordinates demo: applyOffset() bails out before touching
    // offsetX/offsetY until the WASM exports are ready, so reading
    // them directly here keeps Svelte tracking both from the very
    // first run, before either has actually changed.
    void offsetX;
    void offsetY;
    applyOffset();
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
      applyOffset();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="offset-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Двигай мышь над демкой — маленькая белая точка отмечает саму
    позицию курсора. Жёлтый кружок стоит от неё на смещении, заданном
    полями ниже. Поставь оба поля в <code>0</code> — кружок точно
    совпадёт с точкой курсора и скроет её под собой:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="offset x"
      bind:value={offsetX}
      min="-300"
      max="300"
      step="10"
      onfocus={() => (highlightField = 'offsetX')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="offset y"
      bind:value={offsetY}
      min="-300"
      max="300"
      step="10"
      onfocus={() => (highlightField = 'offsetY')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .offset-demo {
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
