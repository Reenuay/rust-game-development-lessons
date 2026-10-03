use macroquad::prelude::*;
use std::f32::consts::PI;
use std::sync::atomic::{AtomicU32, Ordering};

static RADIUS: AtomicU32 = AtomicU32::new(0);
static SECTOR_START: AtomicU32 = AtomicU32::new(0);
static SECTOR_END: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_radius(radius: f32) {
    RADIUS.store(radius.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_sector_start(value: f32) {
    SECTOR_START.store(value.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_sector_end(value: f32) {
    SECTOR_END.store(value.to_bits(), Ordering::Relaxed);
}

// На сколько отрезков приближаем дугу (как в уроке «Дуга»).
const SIDES: u8 = 60;
const THICKNESS: f32 = 3.0;

#[macroquad::main("Зона обнаружения")]
async fn main() {
    set_radius(250.0);
    set_sector_start(PI / 2.0);
    set_sector_end(PI);

    loop {
        clear_background(BLACK);

        let radius = f32::from_bits(RADIUS.load(Ordering::Relaxed));
        let sector_start = f32::from_bits(SECTOR_START.load(Ordering::Relaxed));
        let sector_end = f32::from_bits(SECTOR_END.load(Ordering::Relaxed));

        // Центр экрана — здесь же стоит турель.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();
        let dx = mouse_x - center_x;
        let dy = mouse_y - center_y;

        // Расстояние до курсора (как в уроке «Расстояние»).
        let distance = (dx * dx + dy * dy).sqrt();
        // Угол в сторону курсора, в радианах (как в уроке «Угол вектора»).
        let mouse_angle = dy.atan2(dx);

        // Сколько радиан по часовой стрелке от sector_start до
        // sector_end — весь сектор целиком (как в уроке «Сектор
        // обнаружения»).
        let sector_span = (sector_end - sector_start).rem_euclid(2.0 * PI);
        // Сколько радиан по часовой стрелке от sector_start до курсора.
        let relative = (mouse_angle - sector_start).rem_euclid(2.0 * PI);

        // Турель замечает курсор, только если он и ближе radius, и
        // внутри сектора — оба условия сразу, как в «Круглых кнопках».
        let inside = distance < radius && relative <= sector_span;

        // Дуга — единственное место, где нужны градусы: draw_arc из
        // урока «Дуга» принимает только их, хотя весь остальной код
        // здесь в радианах.
        draw_arc(
            center_x,
            center_y,
            SIDES,
            radius,
            sector_start.to_degrees(),
            THICKNESS,
            sector_span.to_degrees(),
            GRAY,
        );

        // Два прямых края сектора — от центра до концов дуги, длиной
        // ровно radius, замыкают контур с боков.
        let start_x = center_x + sector_start.cos() * radius;
        let start_y = center_y + sector_start.sin() * radius;
        draw_line(center_x, center_y, start_x, start_y, THICKNESS, GRAY);

        let end_x = center_x + sector_end.cos() * radius;
        let end_y = center_y + sector_end.sin() * radius;
        draw_line(center_x, center_y, end_x, end_y, THICKNESS, GRAY);

        // Турель красная, если курсор внутри зоны, иначе синяя.
        if inside {
            draw_circle(center_x, center_y, 40.0, RED);
        } else {
            draw_circle(center_x, center_y, 40.0, BLUE);
        }

        // Игрок — кружок под курсором.
        draw_circle(mouse_x, mouse_y, 25.0, WHITE);

        next_frame().await;
    }
}
