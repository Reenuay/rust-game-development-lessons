use macroquad::prelude::*;
use std::f32::consts::PI;
use std::sync::atomic::{AtomicU32, Ordering};

static SECTOR_START: AtomicU32 = AtomicU32::new(0);
static SECTOR_END: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_sector_start(value: f32) {
    SECTOR_START.store(value.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_sector_end(value: f32) {
    SECTOR_END.store(value.to_bits(), Ordering::Relaxed);
}

// Длина граничных лучей — с запасом, чтобы выходить за край экрана
// любого размера, как положение пули до выстрела в «Турель стреляет».
const RAY_LENGTH: f32 = 2000.0;
// Биссектриса — только для наглядности, какая из двух сторон
// проверяется, поэтому рисуется короткой, а не во весь экран.
const BISECTOR_LENGTH: f32 = 120.0;

#[macroquad::main("Сектор обнаружения")]
async fn main() {
    set_sector_start(PI / 2.0);
    set_sector_end(PI);

    loop {
        clear_background(BLACK);

        let sector_start = f32::from_bits(SECTOR_START.load(Ordering::Relaxed));
        let sector_end = f32::from_bits(SECTOR_END.load(Ordering::Relaxed));

        // Центр экрана — здесь же стоит турель.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();
        let dx = mouse_x - center_x;
        let dy = mouse_y - center_y;

        // Угол на курсор — как в уроке «Угол вектора», в радианах.
        let mouse_angle = dy.atan2(dx);

        // Сколько радиан по часовой стрелке от sector_start до
        // sector_end — весь сектор целиком (rem_euclid сам разбирается
        // с переходом через точку обрыва оборота, как в уроке «Угол от
        // 0 до 360°»).
        let sector_span = (sector_end - sector_start).rem_euclid(2.0 * PI);
        // Сколько радиан по часовой стрелке от sector_start до курсора.
        let relative = (mouse_angle - sector_start).rem_euclid(2.0 * PI);

        let inside = relative <= sector_span;

        // Граничные лучи сектора — уходят далеко за край экрана.
        let start_x = center_x + sector_start.cos() * RAY_LENGTH;
        let start_y = center_y + sector_start.sin() * RAY_LENGTH;
        draw_line(center_x, center_y, start_x, start_y, 3.0, GRAY);

        let end_x = center_x + sector_end.cos() * RAY_LENGTH;
        let end_y = center_y + sector_end.sin() * RAY_LENGTH;
        draw_line(center_x, center_y, end_x, end_y, 3.0, GRAY);

        // Биссектриса — ровно посередине сектора, только чтобы видно
        // было, какая сторона проверяется. На саму проверку не влияет.
        let middle_angle = sector_start + sector_span / 2.0;
        let middle_x = center_x + middle_angle.cos() * BISECTOR_LENGTH;
        let middle_y = center_y + middle_angle.sin() * BISECTOR_LENGTH;
        draw_line(center_x, center_y, middle_x, middle_y, 3.0, YELLOW);

        // Турель красная, если курсор внутри сектора, иначе синяя.
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
