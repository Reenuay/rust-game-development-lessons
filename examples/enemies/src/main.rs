use macroquad::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;

// Здоровье турели: сколько в начале и сколько отнимает один враг.
const MAX_HEALTH: f32 = 100.0;
const ENEMY_DAMAGE: f32 = 20.0;

// Сколько врагов расставляем при старте, их радиус и скорость
// в пикселях в секунду.
const ENEMY_COUNT: u32 = 8;
const ENEMY_RADIUS: f32 = 30.0;
const ENEMY_SPEED: f32 = 100.0;

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

#[macroquad::main("Враги идут к турели")]
async fn main() {
    // Зерно — текущее время, поэтому при каждом запуске враги стоят
    // по-новому.
    rand::srand(macroquad::miniquad::date::now() as u64);

    // Враги лежат в HashMap, у каждого свой ID — как мишени в уроке
    // «Стрельба по мишеням».
    let mut enemies: HashMap<u32, Enemy> = HashMap::new();

    // Враги стоят вокруг турели под случайными углами и на случайном
    // расстоянии: не дальше половины высоты экрана (минус радиус врага,
    // чтобы враг не вылез за край) и не ближе половины от этого. Тогда
    // они дойдут до турели не все сразу.
    let start_x = screen_width() / 2.0;
    let start_y = screen_height() / 2.0;
    let max_distance = screen_height() / 2.0 - ENEMY_RADIUS;
    let min_distance = max_distance / 2.0;
    for id in 0..ENEMY_COUNT {
        // Случайный угол от 0 до полного круга.
        let angle = rand::gen_range(0.0, 2.0 * PI);
        // Случайное расстояние от турели.
        let spawn_distance = rand::gen_range(min_distance, max_distance);
        // Точка вокруг турели — как в уроке «Вращение».
        let x = start_x + angle.cos() * spawn_distance;
        let y = start_y + angle.sin() * spawn_distance;
        // Враг идёт к турели по прямой, поэтому направление считаем один
        // раз, прямо сейчас.
        let (direction_x, direction_y) = direction(x, y, start_x, start_y);
        enemies.insert(id, Enemy { x, y, direction_x, direction_y });
    }

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
