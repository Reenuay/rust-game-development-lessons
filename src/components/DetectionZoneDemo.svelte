<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import RangeSlider from './RangeSlider.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  const TAU = Math.PI * 2;

  let radius = $state(250);
  let sectorStart = $state(Math.PI / 2);
  let sectorEnd = $state(Math.PI);
  let highlightField = $state(null);
  let built = $derived(codeFor(radius, sectorStart, sectorEnd));
  let code = $derived(built.code);
  let codeHtml = $state(plainCodeHtml(codeFor(250, Math.PI / 2, Math.PI).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatRadius(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function formatAngle(n) {
    return n.toFixed(2);
  }

  function codeFor(radius, sectorStart, sectorEnd) {
    return markedCode([
      `use macroquad::prelude::*;
use std::f32::consts::PI;

#[macroquad::main("Зона обнаружения")]
async fn main() {
    loop {
        clear_background(BLACK);

        let radius = `,
      { field: 'radius', value: formatRadius(radius) },
      `;
        // Первый луч сектора — начало.
        let sector_start = `,
      { field: 'start', value: formatAngle(sectorStart) },
      `;
        // Второй луч сектора — конец. От первого до второго — по
        // часовой стрелке.
        let sector_end = `,
      { field: 'end', value: formatAngle(sectorEnd) },
      `;

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        let (mouse_x, mouse_y) = mouse_position();
        let dx = mouse_x - center_x;
        let dy = mouse_y - center_y;

        // Расстояние до курсора (как в уроке «Расстояние»).
        let distance = (dx * dx + dy * dy).sqrt();
        // Угол на курсор, в радианах (как в уроке «Угол вектора»).
        let mouse_angle = dy.atan2(dx);

        // Сколько радиан по часовой стрелке от sector_start до
        // sector_end — весь сектор целиком (как в уроке «Сектор
        // обнаружения»).
        let sector_span = (sector_end - sector_start).rem_euclid(2.0 * PI);
        // Сколько радиан по часовой стрелке от sector_start до курсора.
        let relative = (mouse_angle - sector_start).rem_euclid(2.0 * PI);

        // Турель замечает курсор, только если он и ближе radius, и
        // внутри сектора — оба условия сразу, как в «Круглых кнопках».
        let inside = distance < radius && relative <= sector_span;

        // Дуга — единственное место, где нужны градусы: draw_arc из
        // урока «Дуга» принимает только их, хотя весь остальной код
        // здесь в радианах.
        draw_arc(
            center_x,
            center_y,
            60,
            radius,
            sector_start.to_degrees(),
            3.0,
            sector_span.to_degrees(),
            GRAY,
        );

        // Два прямых края сектора — от центра до концов дуги, длиной
        // ровно radius, замыкают контур с боков.
        let start_x = center_x + sector_start.cos() * radius;
        let start_y = center_y + sector_start.sin() * radius;
        draw_line(center_x, center_y, start_x, start_y, 3.0, GRAY);

        let end_x = center_x + sector_end.cos() * radius;
        let end_y = center_y + sector_end.sin() * radius;
        draw_line(center_x, center_y, end_x, end_y, 3.0, GRAY);

        // Турель красная, если курсор внутри зоны, иначе синяя.
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
    if (!exports?.set_radius || !exports?.set_sector_start || !exports?.set_sector_end) return;
    exports.set_radius(radius);
    exports.set_sector_start(sectorStart);
    exports.set_sector_end(sectorEnd);
    ready = true;
  }

  function clampRadius(value) {
    return Math.min(900, Math.max(50, value));
  }

  function clampRadians(value) {
    return Math.min(TAU, Math.max(0, value));
  }

  $effect(() => {
    radius = clampRadius(radius);
    sectorStart = clampRadians(sectorStart);
    sectorEnd = clampRadians(sectorEnd);
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

<div class="detection-zone-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Турель красная, только когда курсор одновременно и ближе
    <code>radius</code>, и внутри сектора между <code>start</code> и
    <code>end</code> (оба в радианах). Контур зоны — дуга и два прямых
    края. Подвигай поля:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <RangeSlider
      label="radius"
      bind:value={radius}
      min="50"
      max="900"
      step="10"
      onfocus={() => (highlightField = 'radius')}
      onblur={() => (highlightField = null)}
    />
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

  <div class="controls" style={`max-width: ${width}px;`}>
    <RangeSlider
      label="start"
      bind:value={sectorStart}
      min="0"
      max={TAU}
      step="0.01"
      onfocus={() => (highlightField = 'start')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="start"
      bind:value={sectorStart}
      min="0"
      max={TAU}
      step="0.01"
      onfocus={() => (highlightField = 'start')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <div class="controls" style={`max-width: ${width}px;`}>
    <RangeSlider
      label="end"
      bind:value={sectorEnd}
      min="0"
      max={TAU}
      step="0.01"
      onfocus={() => (highlightField = 'end')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="end"
      bind:value={sectorEnd}
      min="0"
      max={TAU}
      step="0.01"
      onfocus={() => (highlightField = 'end')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .detection-zone-demo {
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
