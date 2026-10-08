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

    // Сумма двух точек: x складываем с x, y с y.
    fn plus(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    // Точка, умноженная на обычное число: и x, и y умножаются на него.
    fn times(self, factor: f32) -> Point {
        Point {
            x: self.x * factor,
            y: self.y * factor,
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

// Один враг: где он сейчас и куда идёт.
struct Enemy {
    position: Point,
    direction: Point,
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
    // Направление от турели по углу — как в уроке «Вращение».
    let outward = Point { x: angle.cos(), y: angle.sin() };
    let position = center.plus(outward.times(spawn_distance));
    // Враг идёт к турели по прямой, поэтому направление считаем один раз.
    Enemy { position, direction: position.direction_to(center) }
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

// Центр экрана — здесь стоит турель.
fn screen_center() -> Point {
    Point { x: screen_width() / 2.0, y: screen_height() / 2.0 }
}

// Вся игра в одной структуре: всё, что нужно помнить от кадра к кадру.
struct Game {
    // Враги лежат в HashMap, у каждого свой ID.
    enemies: HashMap<u32, Enemy>,
    // ID, который получит следующий враг.
    next_enemy_id: u32,
    // Сколько секунд осталось ждать до следующего врага.
    spawn_timer: f32,
    // Пули — тоже HashMap с ID.
    bullets: HashMap<u32, Bullet>,
    // ID, который получит следующая пуля.
    next_bullet_id: u32,
    // Сколько секунд осталось ждать до следующего выстрела.
    cooldown: f32,
    // Текущее здоровье турели и очки игрока.
    health: f32,
    score: u32,
    // Куда смотрит турель: направление длиной 1.
    aim: Point,
}

impl Game {
    // Новая игра в начальном состоянии: врагов и пуль нет, здоровье полное,
    // очков нет. self здесь нет — игры ещё не существует, мы её создаём.
    fn new() -> Game {
        Game {
            enemies: HashMap::new(),
            next_enemy_id: 0,
            spawn_timer: 0.0,
            bullets: HashMap::new(),
            next_bullet_id: 0,
            cooldown: 0.0,
            health: MAX_HEALTH,
            score: 0,
            aim: Point { x: 1.0, y: 0.0 },
        }
    }

    // Глазик — центр турели плюс направление, растянутое на 25 пикселей.
    // &self значит, что метод только читает игру.
    fn eye(&self) -> Point {
        screen_center().plus(self.aim.times(25.0))
    }

    // Один шаг игры. &mut self значит, что метод может менять игру.
    // Мышь и кнопку читает main и передаёт сюда.
    fn update(&mut self, dt: f32, mouse: Point, shooting: bool) {
        // Куда смотрит турель — направление от центра к мыши.
        self.aim = screen_center().direction_to(mouse);

        // Шаги игры один за другим.
        self.spawn_enemies(dt);
        self.shoot(dt, shooting);
        self.move_bullets(dt);
        self.move_enemies(dt);
        self.resolve_hits();
    }

    // Появление врагов по таймеру.
    fn spawn_enemies(&mut self, dt: f32) {
        // Пауза между врагами зависит от очков и не короче MIN_SPAWN_INTERVAL.
        let spawn_interval =
            (START_SPAWN_INTERVAL - self.score as f32 * SPEEDUP).max(MIN_SPAWN_INTERVAL);

        // Таймер появления идёт к нулю.
        self.spawn_timer -= dt;

        // Время вышло — на краю экрана появляется новый враг.
        if self.spawn_timer <= 0.0 {
            // Новый враг под своим ID.
            self.enemies.insert(self.next_enemy_id, random_enemy(screen_center()));
            // Следующий враг получит следующий ID.
            self.next_enemy_id += 1;
            // Запускаем отсчёт заново.
            self.spawn_timer = spawn_interval;
        }
    }

    // Выстрел: кнопка зажата и перезарядка закончилась.
    fn shoot(&mut self, dt: f32, shooting: bool) {
        // Счётчик перезарядки уменьшается на столько секунд, сколько прошло
        // с прошлого кадра.
        self.cooldown -= dt;

        // Кнопка зажата и ждать больше не нужно — стреляем.
        if shooting && self.cooldown <= 0.0 {
            // Новая пуля из глазика под своим ID.
            let eye = self.eye();
            self.bullets.insert(
                self.next_bullet_id,
                Bullet { position: eye, direction: self.aim },
            );
            // Следующая пуля получит следующий ID.
            self.next_bullet_id += 1;
            // Запускаем отсчёт заново.
            self.cooldown = SHOOT_INTERVAL;
        }
    }

    // Двигает каждую пулю её собственным направлением.
    fn move_bullets(&mut self, dt: f32) {
        for bullet in self.bullets.values_mut() {
            // Шаг за кадр — направление, умноженное на скорость и на dt.
            let step = bullet.direction.times(BULLET_SPEED * dt);
            bullet.position = bullet.position.plus(step);
        }
    }

    // Двигает каждого врага его собственным направлением.
    fn move_enemies(&mut self, dt: f32) {
        for enemy in self.enemies.values_mut() {
            // Шаг за кадр — направление, умноженное на скорость и на dt.
            let step = enemy.direction.times(ENEMY_SPEED * dt);
            enemy.position = enemy.position.plus(step);
        }
    }

    // Попадания пуль, враги у турели и вылет пуль за экран.
    fn resolve_hits(&mut self) {
        let center = screen_center();

        // Удалять во время чтения нельзя, поэтому ID всего лишнего сначала
        // собираем в списки: пули на удаление, враги, в которых попали, и
        // враги, которые дошли до турели.
        let mut bullets_to_remove: Vec<u32> = Vec::new();
        let mut enemies_shot: Vec<u32> = Vec::new();
        let mut enemies_arrived: Vec<u32> = Vec::new();

        for (&bullet_id, bullet) in &self.bullets {
            // Весь экран как прямоугольник — для проверки, что пуля ещё в нём.
            let screen = Rect { x: 0.0, y: 0.0, width: screen_width(), height: screen_height() };
            // Пуля вылетела за край экрана — на удаление.
            if !bullet.position.inside(screen) {
                bullets_to_remove.push(bullet_id);
            }
            // Пуля задела врага — удаляем и пулю, и врага. Круги
            // пересекаются, если расстояние меньше суммы радиусов.
            for (&enemy_id, enemy) in &self.enemies {
                if bullet.position.distance_to(enemy.position) < BULLET_RADIUS + ENEMY_RADIUS {
                    bullets_to_remove.push(bullet_id);
                    enemies_shot.push(enemy_id);
                }
            }
        }

        for (&enemy_id, enemy) in &self.enemies {
            // Враг дошёл до турели.
            if enemy.position.distance_to(center) < ENEMY_RADIUS + TURRET_RADIUS {
                enemies_arrived.push(enemy_id);
            }
        }

        // Чтение закончилось — теперь можно убирать.
        for id in bullets_to_remove {
            self.bullets.remove(&id);
        }
        for id in enemies_shot {
            // Если в одного врага попали две пули за один кадр, очки
            // даём только раз.
            if let Some(_) = self.enemies.remove(&id) {
                self.score += POINTS_PER_ENEMY;
            }
        }
        for id in enemies_arrived {
            // Дошедший враг пропадает и отнимает здоровье, но не ниже нуля.
            if let Some(_) = self.enemies.remove(&id) {
                self.health = (self.health - ENEMY_DAMAGE).max(0.0);
            }
        }
    }

    // Рисуем всё по порядку: враги, пули, турель, полоска, счёт.
    fn draw(&self) {
        draw_enemies(&self.enemies);
        draw_bullets(&self.bullets);
        draw_turret(screen_center(), self.eye());
        draw_health_bar(self.health);
        draw_score(self.score);
    }
}

#[macroquad::main("Состояние игры")]
async fn main() {
    // Зерно — текущее время, поэтому при каждом запуске враги появляются
    // по-новому.
    rand::srand(macroquad::miniquad::date::now() as u64);

    // Вся игра — одна переменная.
    let mut game = Game::new();

    loop {
        clear_background(BLACK);

        // Положение мыши.
        let (mouse_x, mouse_y) = mouse_position();
        let mouse = Point { x: mouse_x, y: mouse_y };

        // dt (delta time) — сколько секунд прошло между прошлым кадром и этим.
        // Игра делает шаг и рисует себя.
        game.update(get_frame_time(), mouse, is_mouse_button_down(MouseButton::Left));
        game.draw();

        next_frame().await;
    }
}
