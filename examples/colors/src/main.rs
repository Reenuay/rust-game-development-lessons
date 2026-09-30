use macroquad::prelude::*;

// Фиксированный список цветов, из которого выбираем случайный.
const COLORS: [Color; 4] = [YELLOW, RED, GREEN, BLUE];

#[macroquad::main("Разноцветные кружки")]
async fn main() {
    // Список кружков: координаты клика и выбранный для него цвет.
    let mut circles: Vec<(f32, f32, Color)> = Vec::new();

    loop {
        clear_background(BLACK);

        // Кнопка мыши только что нажата — новый кружок со случайным цветом.
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            // Случайный индекс от 0 до длины списка COLORS.
            let index = rand::gen_range(0, COLORS.len());
            circles.push((x, y, COLORS[index]));
        }

        // Рисуем каждый кружок его собственным цветом.
        for &(x, y, color) in &circles {
            draw_circle(x, y, 20.0, color);
        }

        next_frame().await;
    }
}
