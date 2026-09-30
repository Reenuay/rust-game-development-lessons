use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static SPEED: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_speed(speed: f32) {
    SPEED.store(speed.to_bits(), Ordering::Relaxed);
}

// Точка — как в уроке «Точка и прямоугольник».
struct Point {
    x: f32,
    y: f32,
}

// Точка на окружности радиусом radius вокруг center, под углом angle.
fn orbit(center: Point, radius: f32, angle: f32) -> Point {
    Point {
        x: center.x + radius * angle.cos(),
        y: center.y + radius * angle.sin(),
    }
}

#[macroquad::main("Вращение вокруг точки")]
async fn main() {
    set_speed(0.02);

    // Угол растёт каждый кадр — mut и вне loop, чтобы помнить прошлый кадр
    // (как позиция в уроке «Погоня за мышью»).
    let mut angle: f32 = 0.0;

    loop {
        clear_background(BLACK);

        let speed = f32::from_bits(SPEED.load(Ordering::Relaxed));

        let center = Point { x: screen_width() / 2.0, y: screen_height() / 2.0 };

        // Большой кружок в центре — «планета».
        draw_circle(center.x, center.y, 40.0, DARKBLUE);

        // Маленький кружок на орбите вокруг центра.
        let point = orbit(center, 150.0, angle);
        draw_circle(point.x, point.y, 16.0, YELLOW);

        // Угол растёт на фиксированную угловую скорость каждый кадр.
        angle += speed;

        next_frame().await;
    }
}
