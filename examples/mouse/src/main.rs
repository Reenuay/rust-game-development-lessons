use macroquad::prelude::*;

#[macroquad::main("Клик мышью")]
async fn main() {
    // Центр экрана — считаем один раз, до loop.
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        // Левая кнопка мыши зажата — рисуем круг.
        if is_mouse_button_down(MouseButton::Left) {
            draw_circle(center_x, center_y, 100.0, YELLOW);
        }

        next_frame().await;
    }
}
