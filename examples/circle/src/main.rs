use macroquad::prelude::*;

#[macroquad::main("Кружок")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 50.0, YELLOW);

        next_frame().await;
    }
}
