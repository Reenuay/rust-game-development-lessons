<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import RangeSlider from './RangeSlider.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let sectorStart = $state(90);
  let sectorEnd = $state(180);
  let highlightField = $state(null);
  let built = $derived(codeFor(sectorStart, sectorEnd));
  let code = $derived(built.code);
  let codeHtml = $state(plainCodeHtml(codeFor(90, 180).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function codeFor(sectorStart, sectorEnd) {
    return markedCode([
      `use macroquad::prelude::*;

#[macroquad::main("Сектор обнаружения")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Первый луч сектора — начало.
        let sector_start = `,
      { field: 'start', value: formatFloat(sectorStart) },
      `;
        // Второй луч сектора — конец. От первого до второго — по
        // часовой стрелке.
        let sector_end = `,
      { field: 'end', value: formatFloat(sectorEnd) },
      `;

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        let (mouse_x, mouse_y) = mouse_position();
        let dx = mouse_x - center_x;
        let dy = mouse_y - center_y;

        // Угол на курсор — как в уроке «Угол вектора», сразу в градусах.
        let mouse_angle = dy.atan2(dx).to_degrees();

        // На сколько градусов по часовой стрелке от sector_start до
        // sector_end — весь сектор целиком (rem_euclid сам разбирается
        // с переходом через 0°/360°, как в уроке «Угол от 0 до 360°»).
        let sector_span = (sector_end - sector_start).rem_euclid(360.0);
        // На сколько градусов по часовой стрелке от sector_start до курсора.
        let relative = (mouse_angle - sector_start).rem_euclid(360.0);

        let inside = relative <= sector_span;

        // Граничные лучи сектора — уходят далеко за край экрана.
        let ray_length = 2000.0;
        let start_x = center_x + sector_start.to_radians().cos() * ray_length;
        let start_y = center_y + sector_start.to_radians().sin() * ray_length;
        draw_line(center_x, center_y, start_x, start_y, 3.0, GRAY);

        let end_x = center_x + sector_end.to_radians().cos() * ray_length;
        let end_y = center_y + sector_end.to_radians().sin() * ray_length;
        draw_line(center_x, center_y, end_x, end_y, 3.0, GRAY);

        // Биссектриса — ровно посередине сектора, только чтобы видно
        // было, какая сторона проверяется. На саму проверку не влияет.
        let middle_angle = sector_start + sector_span / 2.0;
        let middle_x = center_x + middle_angle.to_radians().cos() * 120.0;
        let middle_y = center_y + middle_angle.to_radians().sin() * 120.0;
        draw_line(center_x, center_y, middle_x, middle_y, 3.0, YELLOW);

        // Турель красная, если курсор внутри сектора, иначе синяя.
        if inside {
            draw_circle(center_x, center_y, 40.0, RED);
        } else {
            draw_circle(center_x, center_y, 40.0, BLUE);
        }

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

  function applyValues() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_sector_start || !exports?.set_sector_end) return;
    exports.set_sector_start(sectorStart);
    exports.set_sector_end(sectorEnd);
    ready = true;
  }

  function clampDegrees(value) {
    return Math.round(Math.min(360, Math.max(0, value)));
  }

  $effect(() => {
    sectorStart = clampDegrees(sectorStart);
    sectorEnd = clampDegrees(sectorEnd);
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

<div class="detection-sector-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Серые лучи — границы сектора: от <code>start</code> до
    <code>end</code> по часовой стрелке. Жёлтый отрезок покороче —
    просто ориентир, какая из двух сторон проверяется, на саму проверку
    не влияет. Води мышью — турель красная, пока курсор внутри сектора,
    иначе синяя. Подвигай оба поля:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <RangeSlider
      label="start°"
      bind:value={sectorStart}
      min="0"
      max="360"
      step="1"
      onfocus={() => (highlightField = 'start')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="start°"
      bind:value={sectorStart}
      min="0"
      max="360"
      step="1"
      onfocus={() => (highlightField = 'start')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <div class="controls" style={`max-width: ${width}px;`}>
    <RangeSlider
      label="end°"
      bind:value={sectorEnd}
      min="0"
      max="360"
      step="1"
      onfocus={() => (highlightField = 'end')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="end°"
      bind:value={sectorEnd}
      min="0"
      max="360"
      step="1"
      onfocus={() => (highlightField = 'end')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .detection-sector-demo {
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
