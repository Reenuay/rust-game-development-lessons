use macroquad::prelude::*;

#[macroquad::main("Турель стреляет")]
async fn main() {
    // Направление глазика по умолчанию — ещё до первого движения мыши.
    let mut direction_x = 1.0;
    let mut direction_y = 0.0;

    // Пока не было клика — пули на экране нет.
    let mut bullet_flying = false;
    let mut bullet_x = 0.0;
    let mut bullet_y = 0.0;
    let mut bullet_direction_x = 0.0;
    let mut bullet_direction_y = 0.0;

    loop {
        clear_background(BLACK);

        // Турель стоит в центре экрана.
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Координаты курсора.
        let (mouse_x, mouse_y) = mouse_position();

        // Направление от турели к курсору (как в «Направлении»).
        let dx = mouse_x - center_x;
        let dy = mouse_y - center_y;
        let distance = (dx * dx + dy * dy).sqrt();

        // Курсор не точно в центре — можно обновить направление.
        if distance > 0.0 {
            direction_x = dx / distance;
            direction_y = dy / distance;
        }

        // Глазик — точка плюс вектор (как в «Точке плюс векторе»):
        // центр турели плюс направление, растянутое на 25 пикселей.
        let eye_x = center_x + direction_x * 25.0;
        let eye_y = center_y + direction_y * 25.0;

        // Клик — новый выстрел: пуля стартует из глазика, а направление
        // полёта фиксируется прямо сейчас и больше не меняется.
        if is_mouse_button_pressed(MouseButton::Left) {
            bullet_flying = true;
            bullet_x = eye_x;
            bullet_y = eye_y;
            bullet_direction_x = direction_x;
            bullet_direction_y = direction_y;
        }

        // Пуля летит своим зафиксированным направлением, не подстраиваясь
        // под курсор — в отличие от «Погони за мышью».
        if bullet_flying {
            bullet_x += bullet_direction_x * 10.0;
            bullet_y += bullet_direction_y * 10.0;
        }

        // Тело турели.
        draw_circle(center_x, center_y, 40.0, BLUE);
        // Глазик — смотрит на курсор.
        draw_circle(eye_x, eye_y, 10.0, WHITE);
        // Пуля — рисуем, только если уже был хоть один клик.
        if bullet_flying {
            draw_circle(bullet_x, bullet_y, 8.0, YELLOW);
        }

        next_frame().await;
    }
}
