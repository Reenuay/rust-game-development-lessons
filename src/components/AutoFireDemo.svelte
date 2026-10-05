<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let interval = $state(10);
  let highlightField = $state(null);
  let built = $derived(codeFor(interval));
  let code = $derived(built.code);
  // Same seeding trick as the other WASM-exported-value demos: start the
  // code panel with plain text so it's never blank while Shiki loads.
  let codeHtml = $state(plainCodeHtml(codeFor(10).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  function codeFor(interval) {
    return markedCode([
      `use macroquad::prelude::*;
use std::collections::HashMap;

// Через сколько кадров после выстрела можно стрелять снова.
// Чем меньше число, тем чаще выстрелы.
const SHOOT_INTERVAL: u32 = `,
      { field: 'interval', value: interval },
      `;

// Скорость пули — одна и та же на каждом кадре, поэтому это константа
// (как в уроке «Константы»).
const SPEED: f32 = 10.0;
// Радиус пули.
const BULLET_RADIUS: f32 = 8.0;

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
// кнопки».
fn inside_rectangle(x: f32, y: f32, rect_x: f32, rect_y: f32, rect_width: f32, rect_height: f32) -> bool {
    x >= rect_x && x <= rect_x + rect_width && y >= rect_y && y <= rect_y + rect_height
}

#[macroquad::main("Автоматический огонь")]
async fn main() {
    // Пули лежат в HashMap, у каждой свой ID — как в уроке «Стрельба по мишеням».
    let mut bullets: HashMap<u32, Bullet> = HashMap::new();
    // ID, который получит следующая пуля.
    let mut next_id: u32 = 0;
    // Сколько кадров осталось ждать до следующего выстрела. 0 — можно стрелять.
    let mut cooldown: u32 = 0;

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

        // Счётчик идёт к нулю и ниже нуля не опускается.
        if cooldown > 0 {
            cooldown -= 1;
        }

        // Кнопка зажата и ждать больше не нужно — стреляем.
        if is_mouse_button_down(MouseButton::Left) && cooldown == 0 {
            // Новая пуля из глазика под своим ID.
            bullets.insert(
                next_id,
                Bullet { x: eye_x, y: eye_y, direction_x, direction_y },
            );
            // Следующая пуля получит следующий ID.
            next_id += 1;
            // Запускаем отсчёт заново.
            cooldown = SHOOT_INTERVAL;
        }

        // Двигаем каждую пулю её собственным направлением.
        for bullet in bullets.values_mut() {
            bullet.x += bullet.direction_x * SPEED;
            bullet.y += bullet.direction_y * SPEED;
        }

        // Удалять во время чтения нельзя, поэтому ID пуль, которые
        // вылетели за край, сначала собираем в список.
        let mut bullets_to_remove: Vec<u32> = Vec::new();
        for (&bullet_id, bullet) in &bullets {
            // Сам экран и есть прямоугольник: от (0, 0) до (screen_width(),
            // screen_height()). Пуля снаружи — на удаление.
            if !inside_rectangle(bullet.x, bullet.y, 0.0, 0.0, screen_width(), screen_height()) {
                bullets_to_remove.push(bullet_id);
            }
        }
        // Чтение закончилось — теперь можно убирать.
        for id in bullets_to_remove {
            bullets.remove(&id);
        }

        // Тело турели.
        draw_circle(center_x, center_y, 40.0, BLUE);
        // Глазик — смотрит на мышь.
        draw_circle(eye_x, eye_y, 10.0, WHITE);

        // Пули.
        for bullet in bullets.values() {
            draw_circle(bullet.x, bullet.y, BULLET_RADIUS, YELLOW);
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

  function applyInterval() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_interval) return;
    exports.set_interval(interval);
    ready = true;
  }

  // <input min max> only guards the spinner arrows, not typed values —
  // clamp for real, and keep it a whole number of frames.
  function clampInterval(value) {
    return Math.min(60, Math.max(1, Math.round(value)));
  }

  $effect(() => {
    interval = clampInterval(interval);
    applyInterval();
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
      applyInterval();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="auto-fire-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Зажми левую кнопку мыши на турели и води мышью — пули летят, пока
    кнопка зажата. Меняй число кадров между выстрелами:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="interval"
      bind:value={interval}
      min="1"
      max="60"
      step="1"
      onfocus={() => (highlightField = 'interval')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .auto-fire-demo {
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
