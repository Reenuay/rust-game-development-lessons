use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static SEED: AtomicU32 = AtomicU32::new(42);

#[no_mangle]
pub extern "C" fn set_seed(seed: u32) {
    SEED.store(seed, Ordering::Relaxed);
}

// Сколько кружков нарисовать.
const COUNT: usize = 20;

// Список случайных точек экрана. Вынесен в функцию только ради демо:
// когда меняют поле seed, программа «запускается заново» и собирает
// список ещё раз.
fn make_points() -> Vec<(f32, f32)> {
    // Список случайных точек — сначала пустой.
    let mut points: Vec<(f32, f32)> = Vec::new();
    // Заполняем его один раз, при старте программы.
    for _ in 0..COUNT {
        // Случайная точка: x от 0 до ширины экрана, y от 0 до высоты.
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        // Кладём точку в список.
        points.push((x, y));
    }
    points
}

#[macroquad::main("Новая случайность")]
async fn main() {
    // Зерно (seed): с него начинается набор случайных чисел.
    let mut seed = SEED.load(Ordering::Relaxed);

    // Задаём зерно один раз, в самом начале, до первого gen_range.
    rand::srand(seed as u64);
    let mut points = make_points();

    loop {
        clear_background(BLACK);

        // Поле seed поменяли — начинаем заново с новым зерном.
        let new_seed = SEED.load(Ordering::Relaxed);
        if new_seed != seed {
            seed = new_seed;
            rand::srand(seed as u64);
            points = make_points();
        }

        // Рисуем кружок в каждой точке списка.
        for &(x, y) in &points {
            draw_circle(x, y, 30.0, YELLOW);
        }

        next_frame().await;
    }
}
