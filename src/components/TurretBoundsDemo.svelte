<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let speed = $state(10);
  let margin = $state(50);
  let highlightField = $state(null);
  let built = $derived(codeFor(speed, margin));
  let code = $derived(built.code);
  // Same seeding trick as the other WASM-exported-value demos: start the
  // code panel with plain text so it's never blank while Shiki loads.
  let codeHtml = $state(plainCodeHtml(codeFor(10, 50).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function codeFor(speed, margin) {
    return markedCode([
      `use macroquad::prelude::*;

// Скорость одна и та же на каждом кадре, поэтому это константа
// (как в уроке «Константы»), а не переменная внутри loop.
const SPEED: f32 = `,
      { field: 'speed', value: formatFloat(speed) },
      `;
// Отступ границы обзора от краёв экрана.
const MARGIN: f32 = `,
      { field: 'margin', value: formatFloat(margin) },
      `;

// Одна пуля: положение и зафиксированное направление полёта.
struct Bullet {
    x: f32,
    y: f32,
    direction_x: f32,
    direction_y: f32,
}

// Своя функция: по двум точкам считает направление от первой ко второй,
// как в уроке «Свои функции».
fn direction(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> (f32, f32) {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance > 0.0 {
        (dx / distance, dy / distance)
    } else {
        (1.0, 0.0)
    }
}

// Своя функция: точка (x, y) внутри прямоугольника (rect_x, rect_y,
// rect_width, rect_height)? Та же проверка, что и в уроке «Прямоугольные
// кнопки», только вынесенная отдельно.
fn inside_rectangle(x: f32, y: f32, rect_x: f32, rect_y: f32, rect_width: f32, rect_height: f32) -> bool {
    x >= rect_x && x <= rect_x + rect_width && y >= rect_y && y <= rect_y + rect_height
}

#[macroquad::main("Пули пропадают за краем")]
async fn main() {
    // Список пуль — сначала пустой.
    let mut bullets: Vec<Bullet> = Vec::new();

    loop {
        clear_background(BLACK);

        // Турель стоит в центре экрана.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты мыши.
        let (mouse_x, mouse_y) = mouse_position();

        // Куда смотрит турель — направление от центра к мыши.
        let (direction_x, direction_y) = direction(center_x, center_y, mouse_x, mouse_y);

        // Глазик — центр турели плюс направление, растянутое на 25 пикселей.
        let eye_x = center_x + direction_x * 25.0;
        let eye_y = center_y + direction_y * 25.0;

        // Кнопка мыши только что нажата — добавляем новую пулю в список.
        if is_mouse_button_pressed(MouseButton::Left) {
            bullets.push(Bullet {
                x: eye_x,
                y: eye_y,
                direction_x,
                direction_y,
            });
        }

        // Видимая граница обзора — меньше экрана на MARGIN со всех сторон.
        let view_x = MARGIN;
        let view_y = MARGIN;
        let view_width = screen_width() - MARGIN * 2.0;
        let view_height = screen_height() - MARGIN * 2.0;
        draw_rectangle_lines(view_x, view_y, view_width, view_height, 8.0, LIGHTGRAY);

        // Тело турели.
        draw_circle(center_x, center_y, 40.0, BLUE);
        // Глазик — смотрит на мышь.
        draw_circle(eye_x, eye_y, 10.0, WHITE);

        // Двигаем каждую пулю и оставляем только те, что ещё внутри границы.
        let mut kept_bullets: Vec<Bullet> = Vec::new();
        for mut bullet in bullets {
            bullet.x += bullet.direction_x * SPEED;
            bullet.y += bullet.direction_y * SPEED;

            if inside_rectangle(bullet.x, bullet.y, view_x, view_y, view_width, view_height) {
                draw_circle(bullet.x, bullet.y, 8.0, YELLOW);
                kept_bullets.push(bullet);
            }
        }
        bullets = kept_bullets;

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
    if (!exports?.set_speed || !exports?.set_margin) return;
    exports.set_speed(speed);
    exports.set_margin(margin);
    ready = true;
  }

  // <input min max> only guards the spinner arrows, not typed values —
  // clamp for real.
  function clampSpeed(value) {
    return Math.min(20, Math.max(2, value));
  }

  // Margin can go all the way down to 0 — that pulls the boundary flush
  // with the real screen edge, per the lesson's point. Upper bound just
  // keeps some visible play area left in a normal-sized viewport.
  function clampMargin(value) {
    return Math.min(200, Math.max(0, value));
  }

  $effect(() => {
    speed = clampSpeed(speed);
    margin = clampMargin(margin);
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

<div class="turret-bounds-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Кликай сколько угодно раз — пули летят и пропадают, едва пересекая
    серую рамку. Подвинь <code>margin</code> к 0 — рамка дойдёт до
    самого края экрана:
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
    <NumberField
      label="margin"
      bind:value={margin}
      min="0"
      max="200"
      step="10"
      onfocus={() => (highlightField = 'margin')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .turret-bounds-demo {
    margin-block: 1rem;
  }

  .demo-instructions {
    margin: 0.75rem 0;
  }

  .controls {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 1.5rem;
    margin: 0;
  }
</style>
