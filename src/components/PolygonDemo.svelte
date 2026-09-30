<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let sides = $state(6);
  let radius = $state(150);
  let showCircle = $state(false);
  let highlightField = $state(null);
  let built = $derived(codeFor(sides, radius));
  let code = $derived(built.code);
  // Same seeding trick as the other WASM-exported-value demos: start the
  // code panel with plain text so it's never blank while Shiki loads.
  let codeHtml = $state(plainCodeHtml(codeFor(6, 150).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function codeFor(sides, radius) {
    return markedCode([
      `use macroquad::prelude::*;

#[macroquad::main("Многоугольники")]
async fn main() {
    loop {
        clear_background(BLACK);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Сколько сторон у многоугольника и какой у него радиус.
        let sides = `,
      { field: 'sides', value: String(sides) },
      `;
        let radius = `,
      { field: 'radius', value: formatFloat(radius) },
      `;

        // Угол между соседними вершинами — полный оборот, поделённый
        // на количество сторон (как в уроке «Радианы»).
        let step = (360.0 / sides as f32).to_radians();

        for i in 0..sides {
            // Угол этой вершины и угол следующей. У последней стороны
            // следующий угол равен полному обороту — той же самой
            // точке, что и нулевой угол, поэтому многоугольник сам
            // замыкается.
            let angle = step * i as f32;
            let next_angle = step * (i as f32 + 1.0);

            // Точка на окружности — центр плюс направление (cos, sin),
            // растянутое на радиус (как в уроке «Синус и косинус»).
            let x1 = center_x + angle.cos() * radius;
            let y1 = center_y + angle.sin() * radius;
            let x2 = center_x + next_angle.cos() * radius;
            let y2 = center_y + next_angle.sin() * radius;

            draw_line(x1, y1, x2, y2, 4.0, YELLOW);
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

  function applyValues() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_sides || !exports?.set_radius || !exports?.set_show_circle) return;
    exports.set_sides(sides);
    exports.set_radius(radius);
    exports.set_show_circle(showCircle);
    ready = true;
  }

  // A polygon needs at least 3 sides to enclose any area at all; above
  // roughly 80 neighboring vertices land closer together than a pixel,
  // so there's nothing more to see past that.
  function clampSides(value) {
    return Math.round(Math.min(80, Math.max(3, value)));
  }

  function clampRadius(value) {
    return Math.min(250, Math.max(20, value));
  }

  function toggleCircle() {
    showCircle = !showCircle;
  }

  $effect(() => {
    sides = clampSides(sides);
    radius = clampRadius(radius);
    applyValues();
    renderCode();
  });

  $effect(() => {
    // showCircle isn't part of the code panel's own text, but it still
    // needs applying to the running demo whenever it changes.
    showCircle;
    applyValues();
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

<div class="polygon-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Подвинь <code>sides</code> до 3-4-5 — увидишь треугольник, квадрат,
    пятиугольник. Подвинь сильно больше — фигура станет неотличима от
    круга. Кнопка ниже рисует рядом настоящую окружность того же
    радиуса для сравнения — при небольшом <code>radius</code> разницы
    почти не видно, а при большом она снова становится заметна.
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="sides"
      bind:value={sides}
      min="3"
      max="80"
      step="1"
      onfocus={() => (highlightField = 'sides')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="radius"
      bind:value={radius}
      min="20"
      max="250"
      step="10"
      onfocus={() => (highlightField = 'radius')}
      onblur={() => (highlightField = null)}
    />
    <button type="button" class="circle-toggle" onclick={toggleCircle}>
      {showCircle ? 'Скрыть окружность' : 'Показать окружность'}
    </button>
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .polygon-demo {
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

  .circle-toggle {
    /* Starlight's prose CSS adds margin-top between adjacent content
       elements — NumberField's own <label> resets this on itself (see
       the comment in that file), but a plain <button> next to it
       still gets pushed down without the same reset. */
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

  .circle-toggle:hover {
    background: var(--sl-color-gray-6);
  }
</style>
