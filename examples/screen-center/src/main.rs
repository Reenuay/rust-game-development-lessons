use macroquad::prelude::*;

#[macroquad::main("Центр экрана")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Середина экрана: половина ширины, половина высоты.
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 100.0, YELLOW);

        next_frame().await;
    }
}
