use macroquad::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

static SIDES: AtomicU32 = AtomicU32::new(0);
static RADIUS: AtomicU32 = AtomicU32::new(0);
static SHOW_CIRCLE: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub extern "C" fn set_sides(sides: u32) {
    SIDES.store(sides, Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_radius(radius: f32) {
    RADIUS.store(radius.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_show_circle(show: bool) {
    SHOW_CIRCLE.store(show, Ordering::Relaxed);
}

#[macroquad::main("Многоугольники")]
async fn main() {
    set_sides(6);
    set_radius(150.0);

    loop {
        clear_background(BLACK);

        let sides = SIDES.load(Ordering::Relaxed);
        let radius = f32::from_bits(RADIUS.load(Ordering::Relaxed));
        let show_circle = SHOW_CIRCLE.load(Ordering::Relaxed);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Окружность того же радиуса — только для сравнения, её можно
        // включить и выключить отдельно.
        if show_circle {
            draw_circle_lines(center_x, center_y, radius, 2.0, LIGHTGRAY);
        }

        // Угол между соседними вершинами — полный оборот, поделённый на
        // количество сторон (как в уроке «Радианы»).
        let step = (360.0 / sides as f32).to_radians();

        for i in 0..sides {
            // Угол этой вершины и угол следующей. У последней стороны
            // следующий угол равен полному обороту — той же самой точке,
            // что и нулевой угол, поэтому многоугольник сам замыкается.
            let angle = step * i as f32;
            let next_angle = step * (i as f32 + 1.0);

            // Точка на окружности — центр плюс направление (cos, sin),
            // растянутое на радиус (как в уроке «Синус и косинус»).
            let x1 = center_x + angle.cos() * radius;
            let y1 = center_y + angle.sin() * radius;
            let x2 = center_x + next_angle.cos() * radius;
            let y2 = center_y + next_angle.sin() * radius;

            draw_line(x1, y1, x2, y2, 4.0, YELLOW);
        }

        next_frame().await;
    }
}
