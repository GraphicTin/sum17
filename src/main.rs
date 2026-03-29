

use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    loop {
        clear_background(RED);
        draw_text("HELLO FROM WASM", 20.0, 20.0, 30.0, DARKGRAY);
        next_frame().await
    }
}