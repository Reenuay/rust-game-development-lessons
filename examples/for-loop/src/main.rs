use macroquad::prelude::*;

// Сколько кружков рисуем.
const COUNT: usize = 5;
// Ширина, на которой они равномерно распределены.
const WIDTH: f32 = 600.0;

#[macroquad::main("Цикл for")]
async fn main() {
    // Шаг между соседними кружками: ширину делим на количество
    // промежутков между ними (кружков пять, промежутков — четыре).
    let step = WIDTH / (COUNT - 1) as f32;

    loop {
        clear_background(BLACK);

        // Центр экрана.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // i по очереди принимает 0, 1, 2, 3, 4 — по разу на каждый кружок.
        for i in 0..COUNT {
            // Начинаем от левого края WIDTH и сдвигаемся на i шагов вправо.
            let x = center_x - WIDTH / 2.0 + step * (i as f32);
            draw_circle(x, center_y, 20.0, YELLOW);
        }

        next_frame().await;
    }
}
