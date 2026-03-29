

use macroquad::prelude::*;

mod palette;
mod asset;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sum17".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

use std::f32::consts::PI;

#[macroquad::main(window_conf)]
async fn main() {
    
    let assets = asset::Assets::load().await.expect("Failed to load assets");

    loop {
        clear_background( palette::MGRAY );
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);

        for i in 0..10 {
            let rotation = i as f32 * (2.0 * PI / 10.0);
            let size = vec2(500.0, 500.0); // Consistent size

            let color = if i%2 == 0 { RED } else { BLUE };
        
            draw_texture_ex(
                &assets.textures.slice,
                center.x - size.x / 2.0, // Center it on screen
                center.y - size.y / 2.0,
                color,
                DrawTextureParams {
                    rotation,
                    // Pivot on the center of the scaled texture
                    pivot: Some(vec2(center.x, center.y)), 
                    dest_size: Some(size),
                    ..Default::default()
                },
            );
        }

        next_frame().await
    }
}