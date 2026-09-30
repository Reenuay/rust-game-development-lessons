use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static SPEED: AtomicU32 = AtomicU32::new(0);
static MARGIN: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_speed(speed: f32) {
    SPEED.store(speed.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_margin(margin: f32) {
    MARGIN.store(margin.to_bits(), Ordering::Relaxed);
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

        let speed = f32::from_bits(SPEED.load(Ordering::Relaxed));
        let margin = f32::from_bits(MARGIN.load(Ordering::Relaxed));

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

        // Видимая граница обзора — меньше экрана на margin со всех сторон.
        let view_x = margin;
        let view_y = margin;
        let view_width = screen_width() - margin * 2.0;
        let view_height = screen_height() - margin * 2.0;
        draw_rectangle_lines(view_x, view_y, view_width, view_height, 8.0, LIGHTGRAY);

        // Тело турели.
        draw_circle(center_x, center_y, 40.0, BLUE);
        // Глазик — смотрит на мышь.
        draw_circle(eye_x, eye_y, 10.0, WHITE);

        // Двигаем каждую пулю и оставляем только те, что ещё внутри границы.
        let mut kept_bullets: Vec<Bullet> = Vec::new();
        for mut bullet in bullets {
            bullet.x += bullet.direction_x * speed;
            bullet.y += bullet.direction_y * speed;

            if inside_rectangle(bullet.x, bullet.y, view_x, view_y, view_width, view_height) {
                draw_circle(bullet.x, bullet.y, 8.0, YELLOW);
                kept_bullets.push(bullet);
            }
        }
        bullets = kept_bullets;

        next_frame().await;
    }
}
