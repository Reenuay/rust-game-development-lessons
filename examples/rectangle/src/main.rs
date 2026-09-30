use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

static X: AtomicU32 = AtomicU32::new(0);
static Y: AtomicU32 = AtomicU32::new(0);
static WIDTH: AtomicU32 = AtomicU32::new(0);
static HEIGHT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn set_x(x: f32) {
    X.store(x.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_y(y: f32) {
    Y.store(y.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_width(width: f32) {
    WIDTH.store(width.to_bits(), Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn set_height(height: f32) {
    HEIGHT.store(height.to_bits(), Ordering::Relaxed);
}

#[macroquad::main("Прямоугольник")]
async fn main() {
    set_x(400.0);
    set_y(300.0);
    set_width(600.0);
    set_height(400.0);

    loop {
        clear_background(BLACK);

        let x = f32::from_bits(X.load(Ordering::Relaxed));
        let y = f32::from_bits(Y.load(Ordering::Relaxed));
        let width = f32::from_bits(WIDTH.load(Ordering::Relaxed));
        let height = f32::from_bits(HEIGHT.load(Ordering::Relaxed));

        // Прямоугольник левым верхним углом в (x, y), размером width на height.
        draw_rectangle(x, y, width, height, YELLOW);

        next_frame().await;
    }
}
