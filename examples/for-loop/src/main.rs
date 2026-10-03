use macroquad::prelude::*;

// Сколько кружков рисуем.
const COUNT: usize = 5;
// Откуда начинаем и на сколько пикселей сдвигаем каждый следующий кружок.
const START_X: f32 = 60.0;
const STEP: f32 = 80.0;

#[macroquad::main("Цикл for")]
async fn main() {
    loop {
        clear_background(BLACK);

        // i по очереди принимает 0, 1, 2, 3, 4 — по разу на каждый кружок.
        for i in 0..COUNT {
            // Начинаем от START_X и сдвигаемся на i шагов вправо.
            let x = START_X + STEP * (i as f32);
            draw_circle(x, screen_height() / 2.0, 20.0, YELLOW);
        }

        next_frame().await;
    }
}
