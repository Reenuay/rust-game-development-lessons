use macroquad::prelude::*;

#[macroquad::main("Координаты мыши")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Координаты курсора прямо сейчас.
        let (x, y) = mouse_position();
        // Круг рисуется прямо под курсором.
        draw_circle(x, y, 40.0, YELLOW);

        next_frame().await;
    }
}
