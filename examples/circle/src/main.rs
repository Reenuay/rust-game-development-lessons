use macroquad::prelude::*;

#[macroquad::main("Круг")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 100.0, YELLOW);

        next_frame().await;
    }
}
