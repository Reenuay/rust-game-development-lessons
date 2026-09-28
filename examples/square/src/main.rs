use macroquad::prelude::*;

#[macroquad::main("Квадрат")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_rectangle(
            screen_width() / 2.0 - 50.0,
            screen_height() / 2.0 - 50.0,
            100.0,
            100.0,
            YELLOW,
        );

        next_frame().await;
    }
}
