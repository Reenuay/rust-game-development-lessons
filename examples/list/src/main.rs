use macroquad::prelude::*;

#[macroquad::main("Список кружков")]
async fn main() {
    // Список точек, куда кликнул игрок — сначала он пустой.
    let mut points: Vec<(f32, f32)> = Vec::new();

    loop {
        clear_background(BLACK);

        // Кнопка мыши только что нажата — запоминаем точку клика.
        if is_mouse_button_pressed(MouseButton::Left) {
            points.push(mouse_position());
        }

        // Рисуем кружок в каждой запомненной точке.
        for &(x, y) in &points {
            draw_circle(x, y, 20.0, YELLOW);
        }

        next_frame().await;
    }
}
