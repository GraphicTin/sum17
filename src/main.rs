

mod palette;
mod asset;
mod objects;
mod logic;

use macroquad::prelude::*;
use std::f32::consts::PI;



fn window_conf() -> Conf {
    Conf {
        window_title: "Sum17".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}



#[macroquad::main(window_conf)]
async fn main() {

    { const _LOAD_RESOURCES: () = (); } // for navigation
    // ------------------------------------------------------------------------------------------------------------
    let assets = asset::Assets::load().await.expect("Failed to load assets");

    let mut numbers: Vec<objects::Number> = (1..=10)
    .map(|i: u8| {
        objects::Number::new(
            i as u8, 
            vec2(-200.0 + (i as f32 * 40.0), 0.0) // Spread them out horizontally
        )
    })
    .collect();

    let mut circle_slots: Vec<Option<objects::Number>> = (0..10).map(|_| None).collect();
    // ------------------------------------------------------------------------------------------------------------


    
    loop {



        // --------------------------------------------------------------------------------------------------------
        clear_background( palette::QGRAY );

        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let mouse_center_offset: Vec2 = Vec2::from(mouse_position()) - center;
        // --------------------------------------------------------------------------------------------------------
        
        logic::update_interaction_system(&mut numbers, &mut circle_slots, mouse_center_offset);

        // --------------------------------------------------------------------------------------------------------
        // need to be optimized to not calculate this every frame, but it works for now
        { const _TASK_1: () = (); } // for navigation

        let mut slot_overloaded = [false; 10];
        for i in 0..10 {

            if circle_slots[i].is_none() { continue; }

            let sum = logic::check_adjacent_sum(&circle_slots, i);
            if sum > 17 {
                // If this triplet is > 17, all three slots that have numbers in the triplet are "Overloaded"
                slot_overloaded[i] = true;
                slot_overloaded[(i + 9) % 10] = circle_slots[(i + 9) % 10].is_some();
                slot_overloaded[(i + 1) % 10] = circle_slots[(i + 1) % 10].is_some();
            }

        }
        // --------------------------------------------------------------------------------------------------------



        
        { const _DRAW_SLICE: () = (); } // for navigation
        // --------------------------------------------------------------------------------------------------------
        for i in 0..10 {

            let rotation = i as f32 * (2.0 * PI / 10.0);
            let size = vec2(500.0, 500.0); // idk why but we use 500 now

            let mut slice_color = if i % 2 == 0 { palette::LWHITE } else { palette::XWHITE };
            if slot_overloaded[i] {
                slice_color = palette::OVERLOADED; 
            }
        
            draw_texture_ex(
                &assets.textures.slice,
                center.x - size.x / 2.0, // Center it on screen
                center.y - size.y / 2.0,
                slice_color,
                DrawTextureParams {
                    rotation,
                    // Pivot on the center of the scaled texture
                    pivot: Some(vec2(center.x, center.y)), 
                    dest_size: Some(size),
                    ..Default::default()
                },
            );

        }
        // --------------------------------------------------------------------------------------------------------

        // --------------------------------------------------------------------------------------------------------
        circle_slots.iter()
            .flatten()
            .chain(numbers.iter())
            .for_each(|num| num.draw(&assets.font, &center));
        // --------------------------------------------------------------------------------------------------------



        { const _DRAW_DEBUG: () = (); } // for navigation
        // --------------------------------------------------------------------------------------------------------
        if cfg!(debug_assertions) {

            let snap_radius = 140.0;
            let gravity_snap_radius = 100.0;
            let gravity_threshold = 80.0;

            for i in 0..10 {
                let angle = i as f32 * (2.0 * PI / 10.0);
                
                // 1. The VISUAL center (where the number will sit)
                let visual_pos = center + vec2(angle.sin() * snap_radius, -angle.cos() * snap_radius);
                
                // 2. The GRAVITY center (where the 'magnet' is located)
                let gravity_pos = center + vec2(angle.sin() * gravity_snap_radius, -angle.cos() * gravity_snap_radius);

                // DRAWING:
                
                // Draw a dotted line showing the "Pull" direction from gravity center to snap center
                draw_line(gravity_pos.x, gravity_pos.y, visual_pos.x, visual_pos.y, 1.0, palette::LGRAY);

                // Draw the "Snap Zone" (Red circle = if you let go inside here, it snaps)
                let zone_color = if circle_slots[i].is_some() { palette::SBLUE } else { Color::new(1.0, 0.0, 0.0, 0.3) };
                draw_circle_lines(gravity_pos.x, gravity_pos.y, gravity_threshold, 1.0, zone_color);
                
                // Draw a small crosshair at the exact visual target
                draw_line(visual_pos.x - 5.0, visual_pos.y, visual_pos.x + 5.0, visual_pos.y, 1.0, palette::XWHITE);
                draw_line(visual_pos.x, visual_pos.y - 5.0, visual_pos.x, visual_pos.y + 5.0, 1.0, palette::XWHITE);

                // Text Label for Slot Index
                draw_text(&format!("index{}", i), visual_pos.x + 10.0, visual_pos.y + 10.0, 20.0, palette::MBLUE);
            }

        }
        // --------------------------------------------------------------------------------------------------------



        next_frame().await



    }

}
