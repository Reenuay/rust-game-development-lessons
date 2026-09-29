use macroquad::prelude::*;

#[macroquad::main("Координаты мыши")]
async fn main() {
    loop {
        clear_background(BLACK);

        let (x, y) = mouse_position();
        draw_circle(x, y, 40.0, YELLOW);

        next_frame().await;
    }
}
