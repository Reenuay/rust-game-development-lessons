use macroquad::prelude::*;

#[macroquad::main("Круг")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Круг в точке (150, 150): x, y — центр, дальше радиус и цвет.
        draw_circle(150.0, 150.0, 100.0, YELLOW);

        next_frame().await;
    }
}
