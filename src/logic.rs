

#![allow(unused)]

use crate::objects;

use macroquad::prelude::*;
use std::f32::consts::PI;


/// Calculates the sum of a slot and its two immediate neighbors in a circular ring.
/// 
/// ## Arguments
/// * `circle_slots` - A reference to the vector containing 10 optional numbers.
/// * `index` - The position (0-9) of the target slot to center the sum on.
///
/// ## Returns
/// The combined `u8` value of the current slot and its two neighbors.
pub fn check_adjacent_sum(circle_slots: &Vec<Option<objects::Number>>, index: usize) -> u8 {

    let current = circle_slots[index].as_ref().map(|n| n.value).unwrap_or(0);

    let left = circle_slots[(index + 9) % 10].as_ref().map_or(0, |n| n.value);

    let right = circle_slots[(index + 1) % 10].as_ref().map_or(0, |n| n.value);
    
    current + left + right as u8
}

/// calculates the position of that slot on a circle.
/// ## Arguments
/// * `index` - The slot index (0-9).
/// * `radius` - The radius of the circle on which the slots are positioned.
pub fn get_slot_center_offset(index: usize, radius: f32) -> Vec2 {
    let angle = index as f32 * (2.0 * PI / 10.0);
    // Use sin/cos to get the point on the circle
    vec2(angle.sin() * radius, -angle.cos() * radius)
}

/// Handles all interaction logic: picking up numbers, dragging them, snapping to slots, and swapping back to hand.
/// 
/// also not finished
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