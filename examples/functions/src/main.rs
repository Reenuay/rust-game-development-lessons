use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static SPEED: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_speed(speed: f32) {
    SPEED.store(speed.to_bits(), Ordering::Relaxed);
}

// Своя функция: по двум точкам считает направление от первой ко второй.
fn direction(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> (f32, f32) {
    // Вектор от начальной точки к конечной.
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    // Длина этого вектора — расстояние между точками.
    let distance = (dx * dx + dy * dy).sqrt();

    // Точки не совпадают — можно посчитать настоящее направление.
    if distance > 0.0 {
        // Делим на длину — получаем направление длиной ровно 1.
        (dx / distance, dy / distance)
    } else {
        // Точки совпали — направления нет, берём по умолчанию вправо.
        (1.0, 0.0)
    }
}

#[macroquad::main("Свои функции")]
async fn main() {
    // bullet_x, bullet_y — положение пули в игровом мире. Пока не было
    // ни одного клика, она стоит далеко за пределами экрана, и её
    // просто не видно.
    let mut bullet_x = -1000.0;
    let mut bullet_y = -1000.0;
    // bullet_direction_x, bullet_direction_y — направление, в котором
    // движется пуля. Сначала оно нулевое: пуля никуда не летит.
    let mut bullet_direction_x = 0.0;
    let mut bullet_direction_y = 0.0;

    loop {
        clear_background(BLACK);

        let speed = f32::from_bits(SPEED.load(Ordering::Relaxed));

        // Турель стоит в центре экрана.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Направление от центра к курсору: туда смотрит турель и туда же
        // полетит пуля, если сейчас выстрелить.
        let (direction_x, direction_y) = direction(center_x, center_y, mouse_x, mouse_y);

        // Глазик — точка плюс вектор (как в уроке «Точка плюс вектор»):
        // центр турели плюс направление, растянутое на 25 пикселей.
        let eye_x = center_x + direction_x * 25.0;
        let eye_y = center_y + direction_y * 25.0;

        // Кнопка мыши только что нажата — новый выстрел.
        if is_mouse_button_pressed(MouseButton::Left) {
            // Переносим пулю в положение глазика.
            bullet_x = eye_x;
            bullet_y = eye_y;
            // Устанавливаем её направление в то, что вычислили выше.
            bullet_direction_x = direction_x;
            bullet_direction_y = direction_y;
        }

        // Пуля летит своим зафиксированным направлением, не подстраиваясь
        // под курсор — в отличие от урока «Погоня за мышью».
        bullet_x += bullet_direction_x * speed;
        bullet_y += bullet_direction_y * speed;

        // Тело турели.
        draw_circle(center_x, center_y, 40.0, BLUE);
        // Глазик — смотрит на курсор.
        draw_circle(eye_x, eye_y, 10.0, WHITE);
        // Пуля — рисуется всегда, просто до первого клика она далеко
        // за экраном и её не видно.
        draw_circle(bullet_x, bullet_y, 8.0, YELLOW);

        next_frame().await;
    }
}
