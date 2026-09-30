use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static SPEED: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_speed(speed: f32) {
    SPEED.store(speed.to_bits(), Ordering::Relaxed);
}

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

        let speed = f32::from_bits(SPEED.load(Ordering::Relaxed));

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
            bullet.x += bullet.direction_x * speed;
            bullet.y += bullet.direction_y * speed;
            draw_circle(bullet.x, bullet.y, 8.0, YELLOW);
        }

        next_frame().await;
    }
}
