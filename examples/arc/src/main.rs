use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static ROTATION: AtomicU32 = AtomicU32::new(0);
static ARC: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_rotation(rotation: f32) {
    ROTATION.store(rotation.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_arc(arc: f32) {
    ARC.store(arc.to_bits(), Ordering::Relaxed);
}

// Толщина дуги и то, на сколько отрезков приближаем полную окружность
// такого же радиуса, — фиксированные, в демке не меняются.
const THICKNESS: f32 = 16.0;
const SIDES: u8 = 60;

#[macroquad::main("Дуга")]
async fn main() {
    set_rotation(0.0);
    set_arc(90.0);

    loop {
        clear_background(BLACK);

        // Читаем текущие значения, выставленные полями в демке.
        let rotation = f32::from_bits(ROTATION.load(Ordering::Relaxed));
        let arc = f32::from_bits(ARC.load(Ordering::Relaxed));

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        let radius = 200.0;

        // Полная окружность — только для ориентира, чтобы видно было,
        // частью какого круга является дуга.
        draw_circle_lines(center_x, center_y, radius, 2.0, LIGHTGRAY);

        // Сама дуга.
        draw_arc(center_x, center_y, SIDES, radius, rotation, THICKNESS, arc, YELLOW);

        next_frame().await;
    }
}
