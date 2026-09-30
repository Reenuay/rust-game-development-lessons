use macroquad::prelude::*;

// Один кружок-кнопка: положение, радиус и обычный цвет.
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

#[macroquad::main("Круглые кнопки")]
async fn main() {
    loop {
        clear_background(BLACK);

        let w = screen_width();
        let h = screen_height();

        // Четыре кнопки в ряд — у каждой своя доля ширины экрана и свой радиус.
        let buttons = [
            Circle { x: w * 0.2, y: h * 0.5, radius: 140.0, color: YELLOW },
            Circle { x: w * 0.4, y: h * 0.5, radius: 200.0, color: RED },
            Circle { x: w * 0.6, y: h * 0.5, radius: 160.0, color: GREEN },
            Circle { x: w * 0.8, y: h * 0.5, radius: 220.0, color: PURPLE },
        ];

        let (mouse_x, mouse_y) = mouse_position();

        // Кнопка мыши зажата — есть смысл проверять попадание в кружки.
        if is_mouse_button_down(MouseButton::Left) {
            for button in &buttons {
                // Мышь попадает внутрь именно этого кружка.
                let inside = distance(mouse_x, mouse_y, button.x, button.y) < button.radius;

                if inside {
                    draw_circle(button.x, button.y, button.radius, BLUE);
                } else {
                    draw_circle(button.x, button.y, button.radius, button.color);
                }
            }
        } else {
            // Мышь отпущена — ни один кружок нажатым быть не может,
            // просто рисуем каждый его обычным цветом.
            for button in &buttons {
                draw_circle(button.x, button.y, button.radius, button.color);
            }
        }

        next_frame().await;
    }
}
