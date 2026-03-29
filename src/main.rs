

mod palette;
mod asset;
mod objects;

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
    
    let assets = asset::Assets::load().await.expect("Failed to load assets");

    let mut numbers: Vec<objects::Number> = (1..=10)
    .map(|index| {
        objects::Number::new(
            index as u8, 
            vec2(-200.0 + (index as f32 * 40.0), 0.0) // Spread them out horizontally
        )
    })
    .collect();

    let mut circle_slots: Vec<Option<objects::Number>> = (0..10).map(|index| None).collect();

    loop {
        clear_background( palette::QGRAY );
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let mouse_center_offset: Vec2 = Vec2::from(mouse_position()) - center;



        for i in 0..10 {
            let rotation = i as f32 * (2.0 * PI / 10.0);
            let size = vec2(500.0, 500.0); // Consistent size

            let color = if i%2 == 0 { palette::LWHITE } else { palette::MWHITE };
        
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

        update_interaction_system(&mut numbers, &mut circle_slots, mouse_center_offset);

        // Inside your render loop:
        for num in &numbers {
            num.draw(&assets.font, &center);
        }
        for slot in &circle_slots {
            if let Some(num) = slot {
                // These are the ones that were "disappearing"
                num.draw(&assets.font, &center);
            }
        }



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
                draw_circle_lines(gravity_pos.x, gravity_pos.y, gravity_threshold, 2.0, zone_color);
                
                // Draw a small crosshair at the exact visual target
                draw_line(visual_pos.x - 5.0, visual_pos.y, visual_pos.x + 5.0, visual_pos.y, 1.0, palette::XWHITE);
                draw_line(visual_pos.x, visual_pos.y - 5.0, visual_pos.x, visual_pos.y + 5.0, 1.0, palette::XWHITE);

                // Text Label for Slot Index
                draw_text(&format!("{}", i), visual_pos.x + 10.0, visual_pos.y + 10.0, 20.0, palette::LGRAY);
            }
            
        }



        next_frame().await



    }
}



pub fn update_interaction_system(
    numbers: &mut Vec<objects::Number>, 
    circle_slots: &mut Vec<Option<objects::Number>>, 
    mouse_center_offset: Vec2
) {
    let snap_radius = 140.0;
    let gravity_snap_radius = 100.0;
    let gravity_threshold = 80.0;

    // --- 1. PICKUP LOGIC ---
    if is_mouse_button_pressed(MouseButton::Left) {
        // A. Check if clicking a number ALREADY in a slot
        let mut slot_to_empty = None;
        for i in 0..10 {
            if let Some(num) = &circle_slots[i] {
                if (mouse_center_offset - num.center_offset).length() < 30.0 {
                    slot_to_empty = Some(i);
                    break;
                }
            }
        }

        if let Some(idx) = slot_to_empty {
            if let Some(mut extracted_num) = circle_slots[idx].take() {
                extracted_num.is_dragging = true;
                extracted_num.drag_offset = extracted_num.center_offset - mouse_center_offset;
                numbers.push(extracted_num);
                return; // Exit to avoid double-processing
            }
        }

        // B. Standard pickup from the hand/pool
        let mut dragged_index = None;
        for (i, num) in numbers.iter_mut().enumerate().rev() {
            if (mouse_center_offset - num.center_offset).length() < 30.0 {
                num.is_dragging = true;
                num.drag_offset = num.center_offset - mouse_center_offset;
                dragged_index = Some(i);
                break; 
            }
        }
        if let Some(i) = dragged_index {
            let picked_up = numbers.remove(i);
            numbers.push(picked_up);
        }
    }

    // --- 2. DRAG & SNAP LOGIC ---
    let mut snapped_to_slot: Option<(usize, usize)> = None;

    for (i, num) in numbers.iter_mut().enumerate() {
        if num.is_dragging {
            num.center_offset = mouse_center_offset + num.drag_offset;
            
            if is_mouse_button_released(MouseButton::Left) {
                num.is_dragging = false;
                let mut best_dist = gravity_threshold;
                let mut best_slot = None;
            
                for slot_idx in 0..10 {
                    let angle = slot_idx as f32 * (2.0 * PI / 10.0);
                    // Using gravity_snap_radius for detection
                    let slot_pos = vec2(angle.sin() * gravity_snap_radius, -angle.cos() * gravity_snap_radius);
                    let dist = (num.center_offset - slot_pos).length();
                    
                    if dist < best_dist && circle_slots[slot_idx].is_none() {
                        best_dist = dist;
                        best_slot = Some(slot_idx);
                    }
                }
            
                if let Some(slot_idx) = best_slot {
                    snapped_to_slot = Some((i, slot_idx));
                }
            }
        }
    }

    // --- 3. TRANSFER LOGIC ---
    if let Some((num_idx, slot_idx)) = snapped_to_slot {
        let mut num = numbers.remove(num_idx);
        let angle = slot_idx as f32 * (2.0 * std::f32::consts::PI / 10.0);
        // Snaps to the visual snap_radius
        num.center_offset = vec2(angle.sin() * snap_radius, -angle.cos() * snap_radius);
        circle_slots[slot_idx] = Some(num);
    }
}



fn check_adjacent_sum(slots: &Vec<Option<objects::Number>>, index: usize) -> u32 {
    let current = slots[index].as_ref().map(|n| n.value as u32).unwrap_or(0);
    let left = slots[(index + 9) % 10].as_ref().map(|n| n.value as u32).unwrap_or(0);
    let right = slots[(index + 1) % 10].as_ref().map(|n| n.value as u32).unwrap_or(0);
    
    current + left + right
}

fn get_slot_pos(index: usize, radius: f32) -> Vec2 {
    let angle = index as f32 * (2.0 * PI / 10.0);
    // Use sin/cos to get the point on the circle
    vec2(angle.sin() * radius, -angle.cos() * radius)
}