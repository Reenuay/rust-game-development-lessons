use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static RADIUS: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_radius(radius: f32) {
    RADIUS.store(radius.to_bits(), Ordering::Relaxed);
}

// Своя функция: расстояние между двумя точками, как в уроке «Расстояние».
fn distance(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> f32 {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    (dx * dx + dy * dy).sqrt()
}

#[macroquad::main("if как значение")]
async fn main() {
    set_radius(300.0);

    loop {
        clear_background(BLACK);

        let radius = f32::from_bits(RADIUS.load(Ordering::Relaxed));

        // Центр экрана — здесь же стоит турель.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Расстояние от турели до курсора.
        let player_distance = distance(center_x, center_y, mouse_x, mouse_y);

        // if с else выдаёт значение: что выбрала сработавшая ветка, то и
        // попадёт в color. Игрок внутри радиуса — красный, иначе синий.
        // После RED и BLUE точку с запятой не ставим.
        let color = if player_distance < radius { RED } else { BLUE };

        // Контур — радиус обнаружения, того же цвета, что и турель.
        draw_circle_lines(center_x, center_y, radius, 3.0, color);

        // Турель.
        draw_circle(center_x, center_y, 40.0, color);

        // Игрок — кружок под курсором.
        draw_circle(mouse_x, mouse_y, 25.0, WHITE);

        next_frame().await;
    }
}
