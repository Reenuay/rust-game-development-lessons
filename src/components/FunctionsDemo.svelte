<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let speed = $state(10);
  let highlightField = $state(null);
  let built = $derived(codeFor(speed));
  let code = $derived(built.code);
  // Same seeding trick as the other WASM-exported-value demos: start the
  // code panel with plain text so it's never blank while Shiki loads.
  let codeHtml = $state(plainCodeHtml(codeFor(10).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function codeFor(speed) {
    return markedCode([
      `use macroquad::prelude::*;

// Скорость одна и та же на каждом кадре, поэтому это константа
// (как в «Константах»), а не переменная внутри loop.
const SPEED: f32 = `,
      { field: 'speed', value: formatFloat(speed) },
      `;

// Своя функция: по двум точкам считает направление от первой ко второй.
fn direction(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> (f32, f32) {
    // Вектор от начальной точки к конечной.
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    // Длина этого вектора — расстояние между точками.
    let distance = (dx * dx + dy * dy).sqrt();

    // Точки не совпадают — можно посчитать настоящее направление.
    if distance > 0.0 {
        // Делим на длину — получаем направление длиной ровно 1.
        (dx / distance, dy / distance)
    } else {
        // Точки совпали — направления нет, берём по умолчанию вправо.
        (1.0, 0.0)
    }
}

#[macroquad::main("Свои функции")]
async fn main() {
    // bullet_x, bullet_y — положение пули в игровом мире. Пока не было
    // ни одного клика, она стоит далеко за пределами экрана, и её
    // просто не видно.
    let mut bullet_x = -1000.0;
    let mut bullet_y = -1000.0;
    // bullet_direction_x, bullet_direction_y — направление, в котором
    // движется пуля. Сначала оно нулевое: пуля никуда не летит.
    let mut bullet_direction_x = 0.0;
    let mut bullet_direction_y = 0.0;

    loop {
        clear_background(BLACK);

        // Турель стоит в центре экрана.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Направление от центра к курсору: туда смотрит турель и туда же
        // полетит пуля, если сейчас выстрелить.
        let (direction_x, direction_y) = direction(center_x, center_y, mouse_x, mouse_y);

        // Глазик — точка плюс вектор (как в «Точке плюс векторе»):
        // центр турели плюс направление, растянутое на 25 пикселей.
        let eye_x = center_x + direction_x * 25.0;
        let eye_y = center_y + direction_y * 25.0;

        // Клик мышью — выстрел. Обновляем пулю только в этот момент:
        // ставим её в глазик и запоминаем текущее направление как
        // направление её полёта. Дальше оно уже не меняется.
        if is_mouse_button_pressed(MouseButton::Left) {
            bullet_x = eye_x;
            bullet_y = eye_y;
            bullet_direction_x = direction_x;
            bullet_direction_y = direction_y;
        }

        // Пуля летит своим зафиксированным направлением, не подстраиваясь
        // под курсор — в отличие от «Погони за мышью».
        bullet_x += bullet_direction_x * SPEED;
        bullet_y += bullet_direction_y * SPEED;

        // Тело турели.
        draw_circle(center_x, center_y, 40.0, BLUE);
        // Глазик — смотрит на курсор.
        draw_circle(eye_x, eye_y, 10.0, WHITE);
        // Пуля — рисуется всегда, просто до первого клика она далеко
        // за экраном и её не видно.
        draw_circle(bullet_x, bullet_y, 8.0, YELLOW);

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

  function applySpeed() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_speed) return;
    exports.set_speed(speed);
    ready = true;
  }

  // <input min max> only guards the spinner arrows, not typed values —
  // clamp for real, same range as the turret demo.
  function clampSpeed(value) {
    return Math.min(20, Math.max(2, value));
  }

  $effect(() => {
    speed = clampSpeed(speed);
    applySpeed();
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
      applySpeed();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="functions-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Та же турель, что в уроке «Турель стреляет», но направление
    считает своя функция <code>direction</code>. Подвигай мышь, кликни
    для выстрела — скорость пули задаётся полем ниже:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="speed"
      bind:value={speed}
      min="2"
      max="20"
      step="1"
      onfocus={() => (highlightField = 'speed')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .functions-demo {
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
