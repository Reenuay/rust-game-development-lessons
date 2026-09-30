<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import RangeSlider from './RangeSlider.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  // Slider and number field below both bind to this same value, so
  // dragging one and typing in the other stay in sync automatically.
  let t = $state(0.5);
  let highlightField = $state(null);
  let built = $derived(codeFor(t));
  let code = $derived(built.code);
  let codeHtml = $state(plainCodeHtml(codeFor(0.5).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatT(n) {
    return n.toFixed(2);
  }

  function codeFor(t) {
    return markedCode([
      `use macroquad::prelude::*;

// Точка — как в уроке «Точка и прямоугольник». Ещё и Copy — start и end
// нужно и передавать в функции, и хранить между кадрами одновременно.
#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

// Расстояние между двумя точками — как в уроке «Точка и прямоугольник».
fn distance(from: Point, to: Point) -> f32 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    (dx * dx + dy * dy).sqrt()
}

// Линейная интерполяция (lerp): точка на пути от from к to, где t — доля
// пройденного пути, от 0.0 (ещё в from) до 1.0 (уже в to).
fn lerp(from: Point, to: Point, t: f32) -> Point {
    Point {
        x: from.x + (to.x - from.x) * t,
        y: from.y + (to.y - from.y) * t,
    }
}

#[macroquad::main("Линейная интерполяция")]
async fn main() {
    // Начало и конец отрезка — можно двигать мышью, как в уроке
    // «Отрезок мышью».
    let mut start = Point { x: screen_width() * 0.2, y: screen_height() / 2.0 };
    let mut end = Point { x: screen_width() * 0.8, y: screen_height() / 2.0 };

    // Какой из концов сейчас тянем, если тянем вообще.
    let mut dragging_start = false;
    let mut dragging_end = false;

    loop {
        clear_background(BLACK);

        // Координаты мыши как точка — удобно сравнивать с start и end.
        let (mouse_x, mouse_y) = mouse_position();
        let mouse = Point { x: mouse_x, y: mouse_y };

        // Кнопку только что нажали рядом с одним из концов — начинаем его тянуть.
        if is_mouse_button_pressed(MouseButton::Left) {
            if distance(mouse, start) < 20.0 {
                dragging_start = true;
            } else if distance(mouse, end) < 20.0 {
                dragging_end = true;
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            // Кнопка зажата — двигаем тот конец, который тянем.
            if dragging_start {
                start = mouse;
            }
            if dragging_end {
                end = mouse;
            }
        } else {
            // Кнопку отпустили — перетаскивание закончилось.
            dragging_start = false;
            dragging_end = false;
        }

        // Сам отрезок — как в уроке «Отрезок мышью».
        draw_line(start.x, start.y, end.x, end.y, 4.0, YELLOW);
        draw_circle(start.x, start.y, 10.0, WHITE);
        draw_circle(end.x, end.y, 10.0, WHITE);

        // Точка на отрезке, определяемая t.
        let t: f32 = `,
      { field: 't', value: formatT(t) },
      `;
        let point = lerp(start, end, t);
        draw_circle(point.x, point.y, 14.0, SKYBLUE);

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

  function applyT() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_t) return;
    exports.set_t(t);
    ready = true;
  }

  function clampT(value) {
    return Math.round(Math.min(1, Math.max(0, value)) * 100) / 100;
  }

  $effect(() => {
    t = clampT(t);
    applyT();
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
      applyT();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="lerp-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Двигай ползунок или впиши число — синяя точка едет по отрезку:
    при <code>t = 0</code> она в начале, при <code>t = 1</code> — в
    конце, а между ними — пропорционально доле пути. Сам отрезок тоже
    можно тянуть за любой из белых концов:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <RangeSlider
      label="t"
      bind:value={t}
      min="0"
      max="1"
      step="0.01"
      onfocus={() => (highlightField = 't')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="t"
      bind:value={t}
      min="0"
      max="1"
      step="0.01"
      onfocus={() => (highlightField = 't')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .lerp-demo {
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
