use macroquad::prelude::*;

#[macroquad::main("Круг")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Круг в точке (100, 100): x, y — центр, дальше радиус и цвет.
        draw_circle(100.0, 100.0, 100.0, YELLOW);

        next_frame().await;
    }
}
