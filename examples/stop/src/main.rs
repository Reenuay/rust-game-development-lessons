use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static SPEED: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_speed(speed: f32) {
    SPEED.store(speed.to_bits(), Ordering::Relaxed);
}

#[macroquad::main("Точная остановка")]
async fn main() {
    // Кружок стартует в центре экрана.
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        let speed = f32::from_bits(SPEED.load(Ordering::Relaxed));

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Вектор от кружка к курсору.
        let dx = mouse_x - x;
        let dy = mouse_y - y;
        let distance = (dx * dx + dy * dy).sqrt();

        // Курсор не точно на кружке — есть куда шагать.
        if distance > 0.0 {
            // Направление на курсор, длиной ровно 1.
            let direction_x = dx / distance;
            let direction_y = dy / distance;

            // Шаг не больше, чем осталось — не проскакиваем мимо цели.
            let step = speed.min(distance);
            x += direction_x * step;
            y += direction_y * step;
        }

        draw_circle(x, y, 30.0, YELLOW);

        next_frame().await;
    }
}
