<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let speed = $state(4);
  let rotateSpeed = $state(0.05);
  let highlightField = $state(null);
  let built = $derived(codeFor(speed, rotateSpeed));
  let code = $derived(built.code);
  // Same seeding trick as the other WASM-exported-value demos: start the
  // code panel with plain text so it's never blank while Shiki loads.
  let codeHtml = $state(plainCodeHtml(codeFor(4, 0.05).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function formatRotateSpeed(n) {
    return n.toFixed(3);
  }

  function codeFor(speed, rotateSpeed) {
    return markedCode([
      `use macroquad::prelude::*;

// Игрок — положение и угол поворота вместе, как в уроке «Свои структуры».
struct Player {
    x: f32,
    y: f32,
    angle: f32,
}

impl Player {
    // Направление, куда сейчас смотрит игрок, — единичный вектор по
    // текущему углу (как в уроке «Синус и косинус»).
    fn direction(&self) -> (f32, f32) {
        (self.angle.cos(), self.angle.sin())
    }
}

// Скорости одни и те же на каждом кадре, поэтому это константы
// (как в уроке «Константы»), а не переменные внутри loop.
const SPEED: f32 = `,
      { field: 'speed', value: formatFloat(speed) },
      `;
const ROTATE_SPEED: f32 = `,
      { field: 'rotateSpeed', value: formatRotateSpeed(rotateSpeed) },
      `;

#[macroquad::main("Поворот и движение")]
async fn main() {
    // Игрок стартует в центре экрана и смотрит вправо — угол 0.
    let mut player = Player {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        angle: 0.0,
    };

    loop {
        clear_background(BLACK);

        // Стрелка влево — поворот против часовой стрелки.
        if is_key_down(KeyCode::Left) {
            player.angle -= ROTATE_SPEED;
        }
        // Стрелка вправо — поворот по часовой стрелке.
        if is_key_down(KeyCode::Right) {
            player.angle += ROTATE_SPEED;
        }

        // Направление, куда сейчас смотрит игрок, — пересчитывается
        // заново на каждом кадре, сразу после поворота.
        let (direction_x, direction_y) = player.direction();

        // Стрелка вверх — шаг вперёд вдоль направления взгляда.
        if is_key_down(KeyCode::Up) {
            player.x += direction_x * SPEED;
            player.y += direction_y * SPEED;
        }
        // Стрелка вниз — шаг назад вдоль того же направления.
        if is_key_down(KeyCode::Down) {
            player.x -= direction_x * SPEED;
            player.y -= direction_y * SPEED;
        }

        // Тело игрока.
        draw_circle(player.x, player.y, 30.0, YELLOW);

        // Глазик — как у турели: точка плюс вектор, растянутый на 25
        // пикселей в направлении взгляда.
        let eye_x = player.x + direction_x * 25.0;
        let eye_y = player.y + direction_y * 25.0;
        draw_circle(eye_x, eye_y, 8.0, WHITE);

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
    if (!exports?.set_speed || !exports?.set_rotate_speed) return;
    exports.set_speed(speed);
    exports.set_rotate_speed(rotateSpeed);
    ready = true;
  }

  // <input min max> only guards the spinner arrows, not typed values —
  // clamp for real so the player can't stall (0 speed) or spin/fly too
  // fast to read as controllable.
  function clampSpeed(value) {
    return Math.min(10, Math.max(1, value));
  }

  function clampRotateSpeed(value) {
    return Math.round(Math.min(0.15, Math.max(0.02, value)) * 1000) / 1000;
  }

  $effect(() => {
    speed = clampSpeed(speed);
    rotateSpeed = clampRotateSpeed(rotateSpeed);
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

<div class="steering-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Стрелки влево/вправо поворачивают игрока, стрелки вверх/вниз двигают
    его вперёд и назад вдоль того направления, куда он сейчас повёрнут.
    Глазик показывает, куда он смотрит (клик по демо переносит на него
    фокус клавиатуры). Поменяй скорости полями ниже:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="speed"
      bind:value={speed}
      min="1"
      max="10"
      step="1"
      onfocus={() => (highlightField = 'speed')}
      onblur={() => (highlightField = null)}
    />
    <NumberField
      label="rotate_speed"
      bind:value={rotateSpeed}
      min="0.02"
      max="0.15"
      step="0.01"
      onfocus={() => (highlightField = 'rotateSpeed')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .steering-demo {
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
