<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  // Both positive so the gray line (drawn from the true top-left corner)
  // stays on screen by default — a negative y would send it straight off
  // the top edge before the reader ever gets to see it.
  let vectorX = $state(150);
  let vectorY = $state(100);
  let highlightField = $state(null);
  let built = $derived(codeFor(vectorX, vectorY));
  let code = $derived(built.code);
  // Same seeding trick as the other WASM-exported-value demos: start the
  // code panel with plain text so it's never blank while Shiki loads.
  let codeHtml = $state(plainCodeHtml(codeFor(150, 100).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function codeFor(vectorX, vectorY) {
    return markedCode([
      `use macroquad::prelude::*;

#[macroquad::main("Точка плюс вектор")]
async fn main() {
    loop {
        clear_background(BLACK);

        let vector_x = `,
      { field: 'vectorX', value: formatFloat(vectorX) },
      `;
        let vector_y = `,
      { field: 'vectorY', value: formatFloat(vectorY) },
      `;

        // Тот же вектор, нарисованный от истинного (0, 0).
        draw_line(0.0, 0.0, vector_x, vector_y, 4.0, GRAY);
        draw_circle(vector_x, vector_y, 10.0, GRAY);

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Белая точка — сама позиция курсора, просто ориентир.
        draw_circle(mouse_x, mouse_y, 6.0, WHITE);

        // Тот же вектор, прибавленный к курсору: точка + вектор.
        draw_line(mouse_x, mouse_y, mouse_x + vector_x, mouse_y + vector_y, 4.0, YELLOW);
        draw_circle(mouse_x + vector_x, mouse_y + vector_y, 10.0, YELLOW);

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

  function applyVector() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_vector_x || !exports?.set_vector_y) return;
    exports.set_vector_x(vectorX);
    exports.set_vector_y(vectorY);
    ready = true;
  }

  $effect(() => {
    void vectorX;
    void vectorY;
    applyVector();
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
      applyVector();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="vector-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Серая линия — вектор, нарисованный от левого верхнего угла: это он
    же, но как точка. Жёлтая линия — тот же самый вектор, только
    прибавленный к курсору (белая точка). Подвигай мышь и поменяй
    вектор полями ниже — оба конца двигаются одинаково, потому что это
    одни и те же два числа:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="vector_x"
      bind:value={vectorX}
      min="-300"
      max="300"
      step="10"
      onfocus={() => (highlightField = 'vectorX')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="vector_y"
      bind:value={vectorY}
      min="-300"
      max="300"
      step="10"
      onfocus={() => (highlightField = 'vectorY')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .vector-demo {
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
