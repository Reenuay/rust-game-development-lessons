use macroquad::prelude::*;

// Один прямоугольник-кнопка: положение, размер и обычный цвет.
struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}

#[macroquad::main("Прямоугольные кнопки")]
async fn main() {
    loop {
        clear_background(BLACK);

        let w = screen_width();
        let h = screen_height();

        // Четыре кнопки в ряд — у каждой своя доля ширины экрана и своя ширина.
        let buttons = [
            Rectangle { x: w * 0.05, y: h * 0.35, width: w * 0.15, height: h * 0.3, color: YELLOW },
            Rectangle { x: w * 0.28, y: h * 0.35, width: w * 0.22, height: h * 0.3, color: RED },
            Rectangle { x: w * 0.58, y: h * 0.35, width: w * 0.15, height: h * 0.3, color: GREEN },
            Rectangle { x: w * 0.80, y: h * 0.35, width: w * 0.18, height: h * 0.3, color: PURPLE },
        ];

        let (mouse_x, mouse_y) = mouse_position();
        // Кнопка мыши зажата прямо сейчас.
        let mouse_down = is_mouse_button_down(MouseButton::Left);

        for button in &buttons {
            // Мышь внутри прямоугольника по обеим осям сразу.
            let inside = mouse_x >= button.x
                && mouse_x <= button.x + button.width
                && mouse_y >= button.y
                && mouse_y <= button.y + button.height;
            // Кнопка нажата, только если мышь зажата и попадает внутрь.
            let pressed = mouse_down && inside;

            if pressed {
                draw_rectangle(button.x, button.y, button.width, button.height, BLUE);
            } else {
                draw_rectangle(button.x, button.y, button.width, button.height, button.color);
            }
        }

        next_frame().await;
    }
}
