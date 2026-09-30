use macroquad::prelude::*;

// Фиксированный список цветов, из которого выбираем случайный.
const COLORS: [Color; 4] = [YELLOW, RED, GREEN, BLUE];

// Один кружок: положение, радиус и цвет вместе, под понятными именами.
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
}

// Своя функция: расстояние между двумя точками, как в уроке «Расстояние».
fn distance(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> f32 {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    (dx * dx + dy * dy).sqrt()
}

// Своя функция: случайный цвет из списка COLORS.
fn random_color() -> Color {
    COLORS[rand::gen_range(0, COLORS.len())]
}

#[macroquad::main("Свои структуры")]
async fn main() {
    // Список уже нарисованных кружков — сначала пустой.
    let mut circles: Vec<Circle> = Vec::new();

    // Точка, где начали тянуть радиус текущего кружка.
    let mut drag_x = 0.0;
    let mut drag_y = 0.0;

    loop {
        clear_background(BLACK);

        // Координаты мыши прямо сейчас.
        let (mouse_x, mouse_y) = mouse_position();

        // Кнопка мыши только что нажата — запоминаем точку начала.
        if is_mouse_button_pressed(MouseButton::Left) {
            drag_x = mouse_x;
            drag_y = mouse_y;
        }

        // Радиус не может быть меньше 5 — иначе кружок пропадёт из виду.
        let radius = distance(drag_x, drag_y, mouse_x, mouse_y).max(5.0);

        // Кнопка зажата — показываем, каким получится кружок.
        if is_mouse_button_down(MouseButton::Left) {
            draw_circle(drag_x, drag_y, radius, YELLOW);
        }

        // Кнопку только что отпустили — сохраняем готовый кружок.
        if is_mouse_button_released(MouseButton::Left) {
            circles.push(Circle { x: drag_x, y: drag_y, radius, color: random_color() });
        }

        // Рисуем все уже сохранённые кружки.
        for circle in &circles {
            draw_circle(circle.x, circle.y, circle.radius, circle.color);
        }

        next_frame().await;
    }
}
