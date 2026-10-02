<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import RangeSlider from './RangeSlider.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let rotation = $state(0);
  let arc = $state(90);
  let highlightField = $state(null);
  let built = $derived(codeFor(rotation, arc));
  let code = $derived(built.code);
  let codeHtml = $state(plainCodeHtml(codeFor(0, 90).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function codeFor(rotation, arc) {
    return markedCode([
      `use macroquad::prelude::*;

// Толщина дуги и то, на сколько отрезков приближаем полную окружность
// такого же радиуса, — фиксированные, в демке не меняются.
const THICKNESS: f32 = 16.0;
const SIDES: u8 = 60;

#[macroquad::main("Дуга")]
async fn main() {
    loop {
        clear_background(BLACK);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        let radius = 200.0;

        // Угол, с которого дуга начинается, — в градусах.
        let rotation = `,
      { field: 'rotation', value: formatFloat(rotation) },
      `;
        // Сколько градусов дуги рисуем, начиная от rotation.
        let arc = `,
      { field: 'arc', value: formatFloat(arc) },
      `;

        // Полная окружность — только для ориентира, чтобы видно было,
        // частью какого круга является дуга.
        draw_circle_lines(center_x, center_y, radius, 2.0, LIGHTGRAY);

        // Сама дуга.
        draw_arc(center_x, center_y, SIDES, radius, rotation, THICKNESS, arc, YELLOW);

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
    if (!exports?.set_rotation || !exports?.set_arc) return;
    exports.set_rotation(rotation);
    exports.set_arc(arc);
    ready = true;
  }

  function clampRotation(value) {
    return Math.round(Math.min(360, Math.max(0, value)));
  }

  function clampArc(value) {
    return Math.round(Math.min(360, Math.max(0, value)));
  }

  $effect(() => {
    rotation = clampRotation(rotation);
    arc = clampArc(arc);
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

<div class="arc-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Серый контур — ориентир, полная окружность того же радиуса. Жёлтая
    дуга начинается под углом <code>rotation</code> и тянется на
    <code>arc</code> градусов по часовой стрелке. Подвигай оба поля:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <RangeSlider
      label="rotation°"
      bind:value={rotation}
      min="0"
      max="360"
      step="1"
      onfocus={() => (highlightField = 'rotation')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="rotation°"
      bind:value={rotation}
      min="0"
      max="360"
      step="1"
      onfocus={() => (highlightField = 'rotation')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <div class="controls" style={`max-width: ${width}px;`}>
    <RangeSlider
      label="arc°"
      bind:value={arc}
      min="0"
      max="360"
      step="1"
      onfocus={() => (highlightField = 'arc')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="arc°"
      bind:value={arc}
      min="0"
      max="360"
      step="1"
      onfocus={() => (highlightField = 'arc')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .arc-demo {
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

  .controls + .controls {
    margin-top: 0.5rem;
  }
</style>
