use macroquad::prelude::*;

// Радиус кружка, которым рисуем точку пути.
const POINT_RADIUS: f32 = 24.0;

// Радиус врага и его скорость в пикселях в секунду.
const ENEMY_RADIUS: f32 = 40.0;
const ENEMY_SPEED: f32 = 150.0;

// Точка на экране. Copy и Clone разрешают копировать точку на ходу.
#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // Расстояние от этой точки до другой.
    fn distance_to(self, other: Point) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    // Направление от этой точки к другой: вектор длиной 1.
    fn direction_to(self, other: Point) -> Point {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 0.0 {
            Point { x: dx / distance, y: dy / distance }
        } else {
            Point { x: 1.0, y: 0.0 }
        }
    }

    // Сумма двух точек: x складываем с x, y с y.
    fn plus(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    // Точка, умноженная на обычное число: и x, и y умножаются на него.
    fn times(self, factor: f32) -> Point {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

// Номер следующей точки пути. После последней снова идёт первая.
fn next_index(index: usize, count: usize) -> usize {
    if index + 1 < count {
        index + 1
    } else {
        0
    }
}

#[macroquad::main("Патрулирование")]
async fn main() {
    // Точки пути — в том порядке, в котором их добавили.
    let mut path: Vec<Point> = Vec::new();

    // Враг: где он стоит и куда смотрит.
    let mut enemy = Point { x: screen_width() / 2.0, y: screen_height() / 2.0 };
    let mut look = Point { x: 1.0, y: 0.0 };

    // Враг сейчас патрулирует? Сначала нет, он стоит.
    let mut patrolling = false;
    // Номер точки пути, к которой враг идёт сейчас.
    let mut target: usize = 0;

    loop {
        clear_background(BLACK);

        // dt (delta time) — сколько секунд прошло между прошлым кадром и этим.
        let dt = get_frame_time();

        // Пробел включает и выключает патрулирование.
        if is_key_pressed(KeyCode::Space) {
            patrolling = !patrolling;
        }

        // Путь можно править, только пока враг стоит. Клик по точке убирает
        // её, клик по пустому месту добавляет новую.
        if !patrolling && is_mouse_button_pressed(MouseButton::Left) {
            // Положение мыши.
            let (mouse_x, mouse_y) = mouse_position();
            let mouse = Point { x: mouse_x, y: mouse_y };

            // Номер точки под курсором. Если такой точки нет — None.
            let mut clicked: Option<usize> = None;
            for i in 0..path.len() {
                if path[i].distance_to(mouse) < POINT_RADIUS {
                    clicked = Some(i);
                }
            }

            if let Some(i) = clicked {
                // Убираем точку с номером i. Остальные сдвигаются на её место.
                path.remove(i);
            } else {
                // Добавляем новую точку в конец пути.
                path.push(mouse);
            }
        }

        // Патрулируем: нужна хотя бы одна точка.
        if patrolling && path.len() > 0 {
            // Если путь успели укоротить, начинаем с первой точки.
            if target >= path.len() {
                target = 0;
            }

            // Точка, к которой идём, и направление к ней.
            let goal = path[target];
            look = enemy.direction_to(goal);

            // Шаг за кадр — скорость в пикселях в секунду, умноженная на dt.
            let step = ENEMY_SPEED * dt;

            if enemy.distance_to(goal) <= step {
                // Осталось меньше шага — встаём ровно в точку и берём следующую.
                enemy = goal;
                target = next_index(target, path.len());
            } else {
                // Иначе просто делаем шаг к точке.
                enemy = enemy.plus(look.times(step));
            }
        }

        // Линии между соседними точками. Последняя соединяется с первой.
        for i in 0..path.len() {
            let from = path[i];
            let to = path[next_index(i, path.len())];
            draw_line(from.x, from.y, to.x, to.y, 5.0, GRAY);
        }

        // Кружки точек с номерами, чтобы было видно порядок.
        for i in 0..path.len() {
            let point = path[i];
            draw_circle(point.x, point.y, POINT_RADIUS, ORANGE);
            draw_text(&(i + 1).to_string(), point.x + 30.0, point.y - 30.0, 80.0, WHITE);
        }

        // Враг и его глазик.
        draw_circle(enemy.x, enemy.y, ENEMY_RADIUS, RED);
        let eye = enemy.plus(look.times(26.0));
        draw_circle(eye.x, eye.y, 11.0, WHITE);

        // Подсказка в углу: что сейчас делает враг и что нажать.
        let mode = if patrolling { "Patrol" } else { "Edit" };
        draw_text(mode, 30.0, 100.0, 100.0, WHITE);
        draw_text("Space - patrol / stop", 30.0, 170.0, 60.0, GRAY);

        next_frame().await;
    }
}
