use macroquad::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

static SPEED: AtomicU32 = AtomicU32::new(0);
static INTERVAL: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_speed(speed: f32) {
    SPEED.store(speed.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_interval(interval: f32) {
    INTERVAL.store(interval.to_bits(), Ordering::Relaxed);
}

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

#[macroquad::main("Секунды вместо кадров")]
async fn main() {
    // Пули лежат в HashMap, у каждой свой ID — как в уроке «Стрельба по мишеням».
    let mut bullets: HashMap<u32, Bullet> = HashMap::new();
    // ID, который получит следующая пуля.
    let mut next_id: u32 = 0;
    // Сколько секунд осталось ждать до следующего выстрела. 0 или меньше — можно стрелять.
    let mut cooldown: f32 = 0.0;

    loop {
        clear_background(BLACK);

        let speed = f32::from_bits(SPEED.load(Ordering::Relaxed));
        let shoot_interval = f32::from_bits(INTERVAL.load(Ordering::Relaxed));

        // Сколько секунд прошло с прошлого кадра.
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

        // Счётчик уменьшается на столько секунд, сколько прошло с прошлого
        // кадра. Если он ушёл ниже нуля — ничего страшного, это тоже
        // значит «можно стрелять».
        cooldown -= dt;

        // Кнопка зажата и ждать больше не нужно — стреляем.
        if is_mouse_button_down(MouseButton::Left) && cooldown <= 0.0 {
            // Новая пуля из глазика под своим ID.
            bullets.insert(
                next_id,
                Bullet { x: eye_x, y: eye_y, direction_x, direction_y },
            );
            // Следующая пуля получит следующий ID.
            next_id += 1;
            // Запускаем отсчёт заново.
            cooldown = shoot_interval;
        }

        // Двигаем каждую пулю её собственным направлением: за кадр она
        // проходит скорость, умноженную на время этого кадра.
        for bullet in bullets.values_mut() {
            bullet.x += bullet.direction_x * speed * dt;
            bullet.y += bullet.direction_y * speed * dt;
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
