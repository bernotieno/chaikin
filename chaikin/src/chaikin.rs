use nannou::prelude::*;
use crate::model::Model;

pub fn prepare_animation(model: &mut Model) {
    // Clear previous animation frames
    model.animation_frames.clear();
    
    // Add original points as first frame
    let original_points = model.points.iter().map(|p| p.position).collect::<Vec<Point2>>();
    model.animation_frames.push(original_points.clone());
    
    // Generate frames for each step of Chaikin's algorithm
    let mut current_points = original_points;
    for _ in 0..7 {
        current_points = chaikin_step(&current_points);
        model.animation_frames.push(current_points.clone());
    }
}