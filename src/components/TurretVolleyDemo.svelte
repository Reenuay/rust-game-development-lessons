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
// (как в уроке «Константы»), а не переменная внутри loop.
const SPEED: f32 = `,
      { field: 'speed', value: formatFloat(speed) },
      `;

// Одна пуля: положение и зафиксированное направление полёта.
struct Bullet {
    x: f32,
    y: f32,
    direction_x: f32,
    direction_y: f32,
}

#[macroquad::main("Турель стреляет очередью")]
async fn main() {
    // Направление глазика по умолчанию — ещё до первого движения мыши.
    let mut direction_x = 1.0;
    let mut direction_y = 0.0;

    // Список пуль — сначала пустой.
    let mut bullets: Vec<Bullet> = Vec::new();

    loop {
        clear_background(BLACK);

        // Турель стоит в центре экрана.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты мыши.
        let (mouse_x, mouse_y) = mouse_position();

        // Направление от турели к мыши, как в уроке «Направление».
        let dx = mouse_x - center_x;
        let dy = mouse_y - center_y;
        let distance = (dx * dx + dy * dy).sqrt();

        // Мышь не точно в центре — можно обновить направление.
        if distance > 0.0 {
            direction_x = dx / distance;
            direction_y = dy / distance;
        }

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

        // Тело турели.
        draw_circle(center_x, center_y, 40.0, BLUE);
        // Глазик — смотрит на мышь.
        draw_circle(eye_x, eye_y, 10.0, WHITE);

        // Каждая пуля летит своим направлением, независимо от остальных.
        for bullet in &mut bullets {
            bullet.x += bullet.direction_x * SPEED;
            bullet.y += bullet.direction_y * SPEED;
            draw_circle(bullet.x, bullet.y, 8.0, YELLOW);
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

  function applySpeed() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_speed) return;
    exports.set_speed(speed);
    ready = true;
  }

  // <input min max> only guards the spinner arrows, not typed values —
  // clamp for real, same range as the single-bullet turret's speed field.
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

<div class="turret-volley-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Подвигай мышь — глазик поворачивается вслед за ней. Кликай
    сколько угодно раз — каждый клик добавляет новую пулю, и все они
    летят одновременно, каждая своим направлением:
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
  .turret-volley-demo {
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
