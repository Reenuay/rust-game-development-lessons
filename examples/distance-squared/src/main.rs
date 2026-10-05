use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static RADIUS: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_radius(radius: f32) {
    RADIUS.store(radius.to_bits(), Ordering::Relaxed);
}

#[macroquad::main("Квадрат расстояния")]
async fn main() {
    set_radius(300.0);

    loop {
        clear_background(BLACK);

        let radius = f32::from_bits(RADIUS.load(Ordering::Relaxed));
        // Радиус в квадрате (в уроке это константа RADIUS_SQUARED).
        let radius_squared = radius * radius;

        // Центр экрана — здесь же стоит турель.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Квадрат расстояния от турели до курсора — то, что в уроке
        // «Расстояние» стояло под корнем, только без самого корня.
        let dx = mouse_x - center_x;
        let dy = mouse_y - center_y;
        let distance_squared = dx * dx + dy * dy;

        // Контур — радиус обнаружения турели.
        draw_circle_lines(center_x, center_y, radius, 3.0, GRAY);

        // Турель красная, если игрок внутри радиуса, иначе синяя. Сравниваем
        // квадрат расстояния с квадратом радиуса — результат тот же.
        if distance_squared < radius_squared {
            draw_circle(center_x, center_y, 40.0, RED);
        } else {
            draw_circle(center_x, center_y, 40.0, BLUE);
        }

        // Игрок — кружок под курсором.
        draw_circle(mouse_x, mouse_y, 25.0, WHITE);

        next_frame().await;
    }
}
