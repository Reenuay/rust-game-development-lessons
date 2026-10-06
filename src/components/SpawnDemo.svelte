<script>
  import WasmCanvas from './WasmCanvas.svelte';
  import NumberField from './NumberField.svelte';
  import CodePanel from './CodePanel.svelte';
  import { loadRustHighlighter, highlightRust, plainCodeHtml, markedCode } from '../lib/rustHighlight.js';

  let { name, width = 720, height = 540 } = $props();

  let speedup = $state(0.005);
  let highlightField = $state(null);
  let built = $derived(codeFor(speedup));
  let code = $derived(built.code);
  // Shiki's highlighter loads asynchronously, so codeHtml can't start
  // highlighted. Starting it at '' left the code panel visibly blank
  // (just the "src/main.rs" title bar, nothing under it) until the
  // highlighter finished loading — seed it with plain, unhighlighted
  // text instead so there's always real code on screen.
  let codeHtml = $state(plainCodeHtml(codeFor(0.005).code));

  let iframeEl = $state(null);
  let highlighter;
  let ready = false;

  // Rust needs a decimal point in a float literal — a bare "1" where f32
  // is expected doesn't compile. Whole numbers like 1 or 0 need ".0"
  // appended; anything already with a fractional part is already a
  // valid literal as-is.
  function formatFloat(n) {
    return Number.isInteger(n) ? `${n}.0` : `${n}`;
  }

  function codeFor(speedup) {
    return markedCode([
      `use macroquad::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;

// Здоровье турели: сколько в начале и сколько отнимает один враг.
const MAX_HEALTH: f32 = 100.0;
const ENEMY_DAMAGE: f32 = 20.0;

// Радиус врага и его скорость в пикселях в секунду.
const ENEMY_RADIUS: f32 = 30.0;
const ENEMY_SPEED: f32 = 100.0;

// Пауза между появлением врагов в секундах: в начале игры и самая короткая.
const START_SPAWN_INTERVAL: f32 = 2.0;
const MIN_SPAWN_INTERVAL: f32 = 0.4;
// На сколько секунд каждое очко сокращает эту паузу.
const SPEEDUP: f32 = `,
      { field: 'speedup', value: formatFloat(speedup) },
      `;

// Пули: радиус, скорость в пикселях в секунду и пауза между выстрелами
// в секундах.
const BULLET_RADIUS: f32 = 8.0;
const BULLET_SPEED: f32 = 600.0;
const SHOOT_INTERVAL: f32 = 0.2;

// Радиус турели.
const TURRET_RADIUS: f32 = 40.0;

// Сколько очков даёт один убитый враг.
const POINTS_PER_ENEMY: u32 = 10;

// Полоска здоровья: левый верхний угол, длина и высота.
const BAR_X: f32 = 20.0;
const BAR_Y: f32 = 20.0;
const BAR_WIDTH: f32 = 300.0;
const BAR_HEIGHT: f32 = 30.0;

// Счёт под полоской: линия, на которой стоят буквы, размер букв и то,
// насколько число сдвинуто вправо от подписи.
const SCORE_Y: f32 = 130.0;
const SCORE_SIZE: f32 = 80.0;
const SCORE_NUMBER_OFFSET: f32 = 260.0;

// Один враг: положение и зафиксированное направление к турели.
struct Enemy {
    x: f32,
    y: f32,
    direction_x: f32,
    direction_y: f32,
}

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

// Своя функция: расстояние между двумя точками, как в уроке «Расстояние».
fn distance(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> f32 {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    (dx * dx + dy * dy).sqrt()
}

// Своя функция: точка (x, y) внутри прямоугольника (rect_x, rect_y,
// rect_width, rect_height)? Та же проверка, что и в уроке «Прямоугольные
// кнопки».
fn inside_rectangle(x: f32, y: f32, rect_x: f32, rect_y: f32, rect_width: f32, rect_height: f32) -> bool {
    x >= rect_x && x <= rect_x + rect_width && y >= rect_y && y <= rect_y + rect_height
}

// Своя функция: враг в случайном месте на краю экрана. Угол случайный, а
// расстояние от турели — половина высоты экрана без радиуса врага, чтобы
// враг не вылез за край.
fn random_enemy(center_x: f32, center_y: f32) -> Enemy {
    // Случайный угол от 0 до полного круга.
    let angle = rand::gen_range(0.0, 2.0 * PI);
    let spawn_distance = screen_height() / 2.0 - ENEMY_RADIUS;
    // Точка вокруг турели — как в уроке «Вращение».
    let x = center_x + angle.cos() * spawn_distance;
    let y = center_y + angle.sin() * spawn_distance;
    // Враг идёт к турели по прямой, поэтому направление считаем один раз.
    let (direction_x, direction_y) = direction(x, y, center_x, center_y);
    Enemy { x, y, direction_x, direction_y }
}

#[macroquad::main("Враги появляются сами")]
async fn main() {
    // Зерно — текущее время, поэтому при каждом запуске враги появляются
    // по-новому.
    rand::srand(macroquad::miniquad::date::now() as u64);

    // Враги лежат в HashMap, у каждого свой ID — как мишени в уроке
    // «Стрельба по мишеням». Сначала врагов нет.
    let mut enemies: HashMap<u32, Enemy> = HashMap::new();
    // ID, который получит следующий враг.
    let mut next_enemy_id: u32 = 0;
    // Сколько секунд осталось ждать до следующего врага.
    let mut spawn_timer: f32 = 0.0;

    // Пули — тоже HashMap с ID, как в уроке «Стрельба по мишеням».
    let mut bullets: HashMap<u32, Bullet> = HashMap::new();
    // ID, который получит следующая пуля.
    let mut next_bullet_id: u32 = 0;
    // Сколько секунд осталось ждать до следующего выстрела.
    let mut cooldown: f32 = 0.0;

    // Текущее здоровье турели и очки игрока.
    let mut health = MAX_HEALTH;
    let mut score: u32 = 0;

    loop {
        clear_background(BLACK);

        // dt (delta time) — сколько секунд прошло между прошлым кадром и этим.
        let dt = get_frame_time();

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

        // Пауза между врагами зависит от очков: каждое очко сокращает её на
        // SPEEDUP секунд, но не короче MIN_SPAWN_INTERVAL. score — целое
        // число, поэтому перед умножением превращаем его в f32.
        let spawn_interval = (START_SPAWN_INTERVAL - score as f32 * SPEEDUP).max(MIN_SPAWN_INTERVAL);

        // Таймер появления идёт к нулю, как счётчик перезарядки.
        spawn_timer -= dt;

        // Время вышло — на краю экрана появляется новый враг.
        if spawn_timer <= 0.0 {
            // Новый враг под своим ID.
            enemies.insert(next_enemy_id, random_enemy(center_x, center_y));
            // Следующий враг получит следующий ID.
            next_enemy_id += 1;
            // Запускаем отсчёт заново.
            spawn_timer = spawn_interval;
        }

        // Счётчик перезарядки уменьшается на столько секунд, сколько прошло
        // с прошлого кадра.
        cooldown -= dt;

        // Кнопка зажата и ждать больше не нужно — стреляем.
        if is_mouse_button_down(MouseButton::Left) && cooldown <= 0.0 {
            // Новая пуля из глазика под своим ID.
            bullets.insert(
                next_bullet_id,
                Bullet { x: eye_x, y: eye_y, direction_x, direction_y },
            );
            // Следующая пуля получит следующий ID.
            next_bullet_id += 1;
            // Запускаем отсчёт заново.
            cooldown = SHOOT_INTERVAL;
        }

        // Двигаем каждую пулю её собственным направлением.
        for bullet in bullets.values_mut() {
            bullet.x += bullet.direction_x * BULLET_SPEED * dt;
            bullet.y += bullet.direction_y * BULLET_SPEED * dt;
        }

        // Двигаем каждого врага его собственным направлением.
        for enemy in enemies.values_mut() {
            // Шаг за кадр — скорость в пикселях в секунду, умноженная на dt.
            enemy.x += enemy.direction_x * ENEMY_SPEED * dt;
            enemy.y += enemy.direction_y * ENEMY_SPEED * dt;
        }

        // Удалять во время чтения нельзя, поэтому ID всего лишнего сначала
        // собираем в списки: пули на удаление, враги, в которых попали, и
        // враги, которые дошли до турели.
        let mut bullets_to_remove: Vec<u32> = Vec::new();
        let mut enemies_shot: Vec<u32> = Vec::new();
        let mut enemies_arrived: Vec<u32> = Vec::new();

        for (&bullet_id, bullet) in &bullets {
            // Пуля вылетела за край экрана — на удаление.
            if !inside_rectangle(bullet.x, bullet.y, 0.0, 0.0, screen_width(), screen_height()) {
                bullets_to_remove.push(bullet_id);
            }
            // Пуля задела врага — удаляем и пулю, и врага. Круги
            // пересекаются, если расстояние меньше суммы радиусов.
            for (&enemy_id, enemy) in &enemies {
                if distance(bullet.x, bullet.y, enemy.x, enemy.y) < BULLET_RADIUS + ENEMY_RADIUS {
                    bullets_to_remove.push(bullet_id);
                    enemies_shot.push(enemy_id);
                }
            }
        }

        for (&enemy_id, enemy) in &enemies {
            // Враг дошёл до турели.
            if distance(enemy.x, enemy.y, center_x, center_y) < ENEMY_RADIUS + TURRET_RADIUS {
                enemies_arrived.push(enemy_id);
            }
        }

        // Чтение закончилось — теперь можно убирать.
        for id in bullets_to_remove {
            bullets.remove(&id);
        }
        for id in enemies_shot {
            // remove отдаёт Some, если такой враг ещё был. Если в одного
            // врага попали две пули за один кадр, очки даём только раз.
            if let Some(_) = enemies.remove(&id) {
                score += POINTS_PER_ENEMY;
            }
        }
        for id in enemies_arrived {
            // Дошедший враг пропадает и отнимает здоровье. max(0.0) не даёт
            // здоровью уйти ниже нуля.
            if let Some(_) = enemies.remove(&id) {
                health = (health - ENEMY_DAMAGE).max(0.0);
            }
        }

        // Враги.
        for enemy in enemies.values() {
            draw_circle(enemy.x, enemy.y, ENEMY_RADIUS, RED);
        }

        // Пули.
        for bullet in bullets.values() {
            draw_circle(bullet.x, bullet.y, BULLET_RADIUS, YELLOW);
        }

        // Тело турели.
        draw_circle(center_x, center_y, TURRET_RADIUS, BLUE);
        // Глазик — смотрит на мышь.
        draw_circle(eye_x, eye_y, 10.0, WHITE);

        // Полоска здоровья. Доля здоровья от 0 до 1.
        let fraction = health / MAX_HEALTH;
        // Серая подложка: вся полоска целиком.
        draw_rectangle(BAR_X, BAR_Y, BAR_WIDTH, BAR_HEIGHT, GRAY);
        // Зелёная часть поверх неё: ширина — доля от всей полоски.
        draw_rectangle(BAR_X, BAR_Y, BAR_WIDTH * fraction, BAR_HEIGHT, GREEN);

        // Счёт под полоской: подпись и число отдельными надписями.
        draw_text("Score:", BAR_X, SCORE_Y, SCORE_SIZE, WHITE);
        draw_text(&score.to_string(), BAR_X + SCORE_NUMBER_OFFSET, SCORE_Y, SCORE_SIZE, WHITE);

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

  function applySpeedup() {
    const exports = iframeEl?.contentWindow?.wasm_exports;
    if (!exports?.set_speedup) return;
    exports.set_speedup(speedup);
    ready = true;
  }

  // The <input min max> attributes only style the spinner buttons and
  // mark the field :invalid — they don't stop someone from typing 5000
  // or -50 directly. Clamp for real here instead, same as the
  // percent/fraction demos do for their own bounded fields.
  function clampSpeedup(value) {
    return Math.min(0.05, Math.max(0, Math.round(value * 1000) / 1000));
  }

  $effect(() => {
    speedup = clampSpeedup(speedup);
    applySpeedup();
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
      applySpeedup();
    }, 100);

    return () => {
      cancelled = true;
      clearInterval(readyPoll);
    };
  });
</script>

<div class="spawn-demo">
  <WasmCanvas {name} {width} {height} bind:iframeEl />

  <p class="demo-instructions">
    Зажми левую кнопку мыши и води мышью: турель стреляет туда, куда
    смотрит. Чем больше очков, тем чаще появляются враги. Меняй, на
    сколько секунд каждое очко сокращает паузу между ними. При нуле
    пауза не сокращается совсем:
  </p>

  <div class="controls" style={`max-width: ${width}px;`}>
    <NumberField
      label="speedup"
      bind:value={speedup}
      min="0"
      max="0.05"
      step="0.001"
      onfocus={() => (highlightField = 'speedup')}
      onblur={() => (highlightField = null)}
    />
  </div>

  <CodePanel html={codeHtml} {code} {highlightField} />
</div>

<style>
  .spawn-demo {
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
