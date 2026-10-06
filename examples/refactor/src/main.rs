use macroquad::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;

// Здоровье турели: сколько в начале и сколько отнимает один враг.
const MAX_HEALTH: f32 = 100.0;
const ENEMY_DAMAGE: f32 = 20.0;

// Радиус врага и его скорость в пикселях в секунду.
const ENEMY_RADIUS: f32 = 30.0;
const ENEMY_SPEED: f32 = 100.0;

// Пауза между появлением врагов в секундах: в начале игры и самая короткая.
// Каждое очко сокращает паузу на SPEEDUP секунд.
const START_SPAWN_INTERVAL: f32 = 2.0;
const MIN_SPAWN_INTERVAL: f32 = 0.4;
const SPEEDUP: f32 = 0.005;

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

// Точка на экране. Copy и Clone разрешают копировать точку на ходу, как
// обычное число: передал в функцию — и она всё ещё твоя.
#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

// Прямоугольник: левый верхний угол, ширина и высота.
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Point {
    // Расстояние от этой точки до другой.
    fn distance_to(self, other: Point) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    // Направление от этой точки к другой: вектор длиной 1.
    fn direction_to(self, other: Point) -> Point {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 0.0 {
            Point { x: dx / distance, y: dy / distance }
        } else {
            Point { x: 1.0, y: 0.0 }
        }
    }

    // Новая точка: эта, сдвинутая по направлению на distance пикселей.
    fn moved(self, direction: Point, distance: f32) -> Point {
        Point {
            x: self.x + direction.x * distance,
            y: self.y + direction.y * distance,
        }
    }

    // Точка внутри прямоугольника?
    fn inside(self, rect: Rect) -> bool {
        self.x >= rect.x
            && self.x <= rect.x + rect.width
            && self.y >= rect.y
            && self.y <= rect.y + rect.height
    }
}

// Один враг: где он сейчас.
struct Enemy {
    position: Point,
}

// Одна пуля: где она сейчас и куда летит.
struct Bullet {
    position: Point,
    direction: Point,
}

// Враг в случайном месте на краю экрана. Угол случайный, а расстояние от
// турели — половина высоты экрана без радиуса врага, чтобы враг не вылез
// за край.
fn random_enemy(center: Point) -> Enemy {
    // Случайный угол от 0 до полного круга.
    let angle = rand::gen_range(0.0, 2.0 * PI);
    let spawn_distance = screen_height() / 2.0 - ENEMY_RADIUS;
    // Направление по углу — как в уроке «Вращение».
    let direction = Point { x: angle.cos(), y: angle.sin() };
    Enemy { position: center.moved(direction, spawn_distance) }
}

// Двигает каждую пулю её собственным направлением.
fn move_bullets(bullets: &mut HashMap<u32, Bullet>, dt: f32) {
    for bullet in bullets.values_mut() {
        bullet.position = bullet.position.moved(bullet.direction, BULLET_SPEED * dt);
    }
}

// Двигает каждого врага в сторону цели.
fn move_enemies(enemies: &mut HashMap<u32, Enemy>, target: Point, dt: f32) {
    for enemy in enemies.values_mut() {
        // Направление от врага к цели, считаем заново на каждом кадре.
        let direction = enemy.position.direction_to(target);
        enemy.position = enemy.position.moved(direction, ENEMY_SPEED * dt);
    }
}

// Рисует всех врагов.
fn draw_enemies(enemies: &HashMap<u32, Enemy>) {
    for enemy in enemies.values() {
        draw_circle(enemy.position.x, enemy.position.y, ENEMY_RADIUS, RED);
    }
}

// Рисует все пули.
fn draw_bullets(bullets: &HashMap<u32, Bullet>) {
    for bullet in bullets.values() {
        draw_circle(bullet.position.x, bullet.position.y, BULLET_RADIUS, YELLOW);
    }
}

// Рисует турель: тело и глазик.
fn draw_turret(center: Point, eye: Point) {
    draw_circle(center.x, center.y, TURRET_RADIUS, BLUE);
    draw_circle(eye.x, eye.y, 10.0, WHITE);
}

// Рисует полоску здоровья: серую подложку и зелёную часть поверх.
fn draw_health_bar(health: f32) {
    // Доля здоровья от 0 до 1.
    let fraction = health / MAX_HEALTH;
    draw_rectangle(BAR_X, BAR_Y, BAR_WIDTH, BAR_HEIGHT, GRAY);
    draw_rectangle(BAR_X, BAR_Y, BAR_WIDTH * fraction, BAR_HEIGHT, GREEN);
}

// Рисует счёт под полоской: подпись и число отдельными надписями.
fn draw_score(score: u32) {
    draw_text("Score:", BAR_X, SCORE_Y, SCORE_SIZE, WHITE);
    draw_text(&score.to_string(), BAR_X + SCORE_NUMBER_OFFSET, SCORE_Y, SCORE_SIZE, WHITE);
}

#[macroquad::main("Наводим порядок")]
async fn main() {
    // Зерно — текущее время, поэтому при каждом запуске враги появляются
    // по-новому.
    rand::srand(macroquad::miniquad::date::now() as u64);

    // Враги лежат в HashMap, у каждого свой ID. Сначала врагов нет.
    let mut enemies: HashMap<u32, Enemy> = HashMap::new();
    // ID, который получит следующий враг.
    let mut next_enemy_id: u32 = 0;
    // Сколько секунд осталось ждать до следующего врага.
    let mut spawn_timer: f32 = 0.0;

    // Пули — тоже HashMap с ID.
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
        let center = Point { x: screen_width() / 2.0, y: screen_height() / 2.0 };

        // Положение мыши.
        let (mouse_x, mouse_y) = mouse_position();
        let mouse = Point { x: mouse_x, y: mouse_y };

        // Куда смотрит турель — направление от центра к мыши.
        let aim = center.direction_to(mouse);

        // Глазик — центр турели, сдвинутый в сторону мыши на 25 пикселей.
        let eye = center.moved(aim, 25.0);

        // Пауза между врагами зависит от очков и не короче MIN_SPAWN_INTERVAL.
        let spawn_interval = (START_SPAWN_INTERVAL - score as f32 * SPEEDUP).max(MIN_SPAWN_INTERVAL);

        // Таймер появления идёт к нулю.
        spawn_timer -= dt;

        // Время вышло — на краю экрана появляется новый враг.
        if spawn_timer <= 0.0 {
            // Новый враг под своим ID.
            enemies.insert(next_enemy_id, random_enemy(center));
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
            bullets.insert(next_bullet_id, Bullet { position: eye, direction: aim });
            // Следующая пуля получит следующий ID.
            next_bullet_id += 1;
            // Запускаем отсчёт заново.
            cooldown = SHOOT_INTERVAL;
        }

        // Двигаем пуль и врагов.
        move_bullets(&mut bullets, dt);
        move_enemies(&mut enemies, center, dt);

        // Удалять во время чтения нельзя, поэтому ID всего лишнего сначала
        // собираем в списки: пули на удаление, враги, в которых попали, и
        // враги, которые дошли до турели.
        let mut bullets_to_remove: Vec<u32> = Vec::new();
        let mut enemies_shot: Vec<u32> = Vec::new();
        let mut enemies_arrived: Vec<u32> = Vec::new();

        for (&bullet_id, bullet) in &bullets {
            // Весь экран как прямоугольник — для проверки, что пуля ещё в нём.
            let screen = Rect { x: 0.0, y: 0.0, width: screen_width(), height: screen_height() };
            // Пуля вылетела за край экрана — на удаление.
            if !bullet.position.inside(screen) {
                bullets_to_remove.push(bullet_id);
            }
            // Пуля задела врага — удаляем и пулю, и врага. Круги
            // пересекаются, если расстояние меньше суммы радиусов.
            for (&enemy_id, enemy) in &enemies {
                if bullet.position.distance_to(enemy.position) < BULLET_RADIUS + ENEMY_RADIUS {
                    bullets_to_remove.push(bullet_id);
                    enemies_shot.push(enemy_id);
                }
            }
        }

        for (&enemy_id, enemy) in &enemies {
            // Враг дошёл до турели.
            if enemy.position.distance_to(center) < ENEMY_RADIUS + TURRET_RADIUS {
                enemies_arrived.push(enemy_id);
            }
        }

        // Чтение закончилось — теперь можно убирать.
        for id in bullets_to_remove {
            bullets.remove(&id);
        }
        for id in enemies_shot {
            // Если в одного врага попали две пули за один кадр, очки
            // даём только раз.
            if let Some(_) = enemies.remove(&id) {
                score += POINTS_PER_ENEMY;
            }
        }
        for id in enemies_arrived {
            // Дошедший враг пропадает и отнимает здоровье, но не ниже нуля.
            if let Some(_) = enemies.remove(&id) {
                health = (health - ENEMY_DAMAGE).max(0.0);
            }
        }

        // Рисуем всё по порядку: враги, пули, турель, полоска, счёт.
        draw_enemies(&enemies);
        draw_bullets(&bullets);
        draw_turret(center, eye);
        draw_health_bar(health);
        draw_score(score);

        next_frame().await;
    }
}
