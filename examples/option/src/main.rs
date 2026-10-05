use macroquad::prelude::*;
use std::collections::HashMap;

// Радиус мишени.
const TARGET_RADIUS: f32 = 40.0;
// Сколько мишеней расставляем.
const TARGET_COUNT: u32 = 10;

// Своя функция: по двум точкам считает направление от первой ко второй,
// как в уроке «Свои функции».
fn direction(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> (f32, f32) {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance > 0.0 {
        (dx / distance, dy / distance)
    } else {
        (1.0, 0.0)
    }
}

// Своя функция: расстояние между двумя точками, как в уроке «Расстояние».
fn distance(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> f32 {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    (dx * dx + dy * dy).sqrt()
}

// Своя функция: расставляет мишени в случайных местах экрана. Ключ — ID
// мишени, значение — её координаты.
fn make_targets() -> HashMap<u32, (f32, f32)> {
    // Мишеней пока нет.
    let mut targets: HashMap<u32, (f32, f32)> = HashMap::new();

    for id in 0..TARGET_COUNT {
        // Случайная точка, отступив от краёв на радиус, чтобы мишень
        // целиком поместилась на экране.
        let x = rand::gen_range(TARGET_RADIUS, screen_width() - TARGET_RADIUS);
        let y = rand::gen_range(TARGET_RADIUS, screen_height() - TARGET_RADIUS);
        // Кладём мишень в HashMap под её ID.
        targets.insert(id, (x, y));
    }

    targets
}

#[macroquad::main("Option")]
async fn main() {
    // Зерно — текущее время, поэтому при каждом запуске поле новое.
    rand::srand(macroquad::miniquad::date::now() as u64);

    // Расставляем мишени при старте.
    let mut targets = make_targets();

    loop {
        clear_background(BLACK);

        // Турель стоит в центре экрана.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Ищем ближайшую цель. ID лучшей цели на данный момент лежит в
        // Option: пока цель не найдена, там None — «ничего».
        let mut best_id: Option<u32> = None;
        // Расстояние до неё. Сначала — самое большое число f32, чтобы
        // первая же мишень оказалась ближе.
        let mut best_distance = f32::MAX;

        // Смотрим на каждую мишень по очереди.
        for (&id, &(x, y)) in &targets {
            // Расстояние от турели до этой мишени.
            let d = distance(center_x, center_y, x, y);
            // Строго ближе, чем всё, что видели раньше, — запоминаем её.
            if d < best_distance {
                best_distance = d;
                // Нашли цель получше — кладём её ID внутрь Some.
                best_id = Some(id);
            }
        }

        // Точка, на которую смотрит турель. Если целей нет — точка
        // правее турели.
        let mut look_x = center_x + 100.0;
        let mut look_y = center_y;
        // Если в best_id лежит Some, называем то, что внутри, id.
        if let Some(id) = best_id {
            // Достаём координаты выбранной цели по её ID.
            let (target_x, target_y) = targets[&id];
            look_x = target_x;
            look_y = target_y;
        }

        // Направление от турели к этой точке.
        let (direction_x, direction_y) = direction(center_x, center_y, look_x, look_y);

        // Глазик — центр турели плюс направление, растянутое на 25 пикселей.
        let eye_x = center_x + direction_x * 25.0;
        let eye_y = center_y + direction_y * 25.0;

        // Кнопка мыши только что нажата.
        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(id) = best_id {
                // Убираем выбранную цель. Читать targets мы уже закончили.
                targets.remove(&id);
            } else {
                // В best_id None, целей не осталось — расставляем поле заново.
                targets = make_targets();
            }
        }

        // Рисуем мишени.
        for (&id, &(x, y)) in &targets {
            // Сначала красим мишень в красный.
            let mut color = RED;
            // Если выбранная цель — это именно эта, красим в оранжевый.
            if best_id == Some(id) {
                color = ORANGE;
            }
            draw_circle(x, y, TARGET_RADIUS, color);
        }

        // Тело турели.
        draw_circle(center_x, center_y, 40.0, BLUE);
        // Глазик — смотрит на выбранную цель.
        draw_circle(eye_x, eye_y, 10.0, WHITE);

        next_frame().await;
    }
}
