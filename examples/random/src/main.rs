use macroquad::prelude::*;

#[macroquad::main("Случайная точка")]
async fn main() {
    // Кружок стартует по центру экрана.
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(BLACK);

        // Кнопка мыши только что нажата — новая случайная точка.
        if is_mouse_button_pressed(MouseButton::Left) {
            // Случайное число от 0 до ширины экрана.
            x = rand::gen_range(0.0, screen_width());
            // Случайное число от 0 до высоты экрана.
            y = rand::gen_range(0.0, screen_height());
        }

        draw_circle(x, y, 40.0, YELLOW);

        next_frame().await;
    }
}
