use macroquad::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;

// Радиус у всех мишеней одинаковый — фиксированный, как в уроке «Константы».
const TARGET_RADIUS: f32 = 55.0;
// Радиус пули — свой, поменьше.
const BULLET_RADIUS: f32 = 8.0;
// Скорость пули — одна и та же на каждом кадре.
const SPEED: f32 = 8.0;
// Сколько мишеней расставляем при старте.
const TARGET_COUNT: usize = 15;

// Одна пуля: положение и зафиксированное направление полёта.
struct Bullet {
    x: f32,
    y: f32,
    direction_x: f32,
    direction_y: f32,
}

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

#[macroquad::main("Стрельба по мишеням")]
async fn main() {
    // Турель всегда стоит в центре экрана.
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    // Дистанция мишеней от турели — не ближе четверти высоты экрана,
    // не дальше половины, чтобы ни одна не оказалась за краем.
    let max_distance = screen_height() / 2.0;
    let min_distance = max_distance / 2.0;

    // Мишени — фиксированный набор, появляется один раз при старте.
    let mut targets: HashMap<u32, (f32, f32)> = HashMap::new();
    for i in 0..TARGET_COUNT {
        // Случайный угол и случайная дистанция вместе дают случайную
        // точку в кольце вокруг турели (как в уроке «Вращение»).
        let angle = rand::gen_range(0.0, 2.0 * PI);
        let target_distance = rand::gen_range(min_distance, max_distance);

        let target_x = center_x + angle.cos() * target_distance;
        let target_y = center_y + angle.sin() * target_distance;

        targets.insert(i as u32, (target_x, target_y));
    }

    // Пули появляются по одной, по клику — свой HashMap и свой счётчик
    // ID, никак не связанный со счётчиком мишеней выше.
    let mut bullets: HashMap<u32, Bullet> = HashMap::new();
    let mut next_id: u32 = 0;

    loop {
        clear_background(BLACK);

        // Координаты мыши.
        let (mouse_x, mouse_y) = mouse_position();

        // Турель мгновенно смотрит в сторону мыши.
        let (direction_x, direction_y) = direction(center_x, center_y, mouse_x, mouse_y);

        // Глазик — центр турели плюс направление, растянутое на 25 пикселей.
        let eye_x = center_x + direction_x * 25.0;
        let eye_y = center_y + direction_y * 25.0;

        // Кнопка мыши только что нажата — новая пуля из глазика.
        if is_mouse_button_pressed(MouseButton::Left) {
            bullets.insert(
                next_id,
                Bullet { x: eye_x, y: eye_y, direction_x, direction_y },
            );
            next_id += 1;
        }

        // Двигаем каждую пулю её собственным направлением.
        for bullet in bullets.values_mut() {
            bullet.x += bullet.direction_x * SPEED;
            bullet.y += bullet.direction_y * SPEED;
        }

        // Сначала только читаем bullets и targets — удалять во время
        // чтения нельзя, поэтому собираем ID на вылет в отдельные списки.
        let mut bullets_to_remove: Vec<u32> = Vec::new();
        let mut targets_to_remove: Vec<u32> = Vec::new();

        for (&bullet_id, bullet) in &bullets {
            // Пуля покинула экран — сразу на вылет, до мишеней дело не доходит.
            let outside = bullet.x < 0.0
                || bullet.x > screen_width()
                || bullet.y < 0.0
                || bullet.y > screen_height();

            if outside {
                bullets_to_remove.push(bullet_id);
                continue;
            }

            // Иначе проверяем пересечение с каждой мишенью — сумма
            // радиусов, как в уроке «Пересечение кружков».
            for (&target_id, &(target_x, target_y)) in &targets {
                if distance(bullet.x, bullet.y, target_x, target_y) < BULLET_RADIUS + TARGET_RADIUS {
                    bullets_to_remove.push(bullet_id);
                    targets_to_remove.push(target_id);
                }
            }
        }

        // Теперь, когда чтение закончилось, убираем всё, что попало в списки.
        for id in bullets_to_remove {
            bullets.remove(&id);
        }
        for id in targets_to_remove {
            targets.remove(&id);
        }

        // Мишени — все одного цвета, отличного и от турели, и от пуль.
        for &(target_x, target_y) in targets.values() {
            draw_circle(target_x, target_y, TARGET_RADIUS, RED);
        }

        // Тело турели.
        draw_circle(center_x, center_y, 40.0, BLUE);
        // Глазик — смотрит на мышь.
        draw_circle(eye_x, eye_y, 10.0, WHITE);

        // Пули.
        for bullet in bullets.values() {
            draw_circle(bullet.x, bullet.y, BULLET_RADIUS, YELLOW);
        }

        next_frame().await;
    }
}
