use macroquad::prelude::*;

#[macroquad::main("Ввод с клавиатуры")]
async fn main() {
    // Центр экрана — считаем один раз, до loop.
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        // Стрелка вверх зажата — рисуем круг.
        if is_key_down(KeyCode::Up) {
            draw_circle(center_x, center_y, 100.0, YELLOW);
        }

        next_frame().await;
    }
}
