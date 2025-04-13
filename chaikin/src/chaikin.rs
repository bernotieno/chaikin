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

pub fn chaikin_step(points: &[Point2]) -> Vec<Point2> {
    if points.len() <= 2 {
        return points.to_vec();
    }
    
    let mut result = Vec::new();
    
    for i in 0..points.len() - 1 {
        let p0 = points[i];
        let p1 = points[i + 1];
        
        // Calculate points at 1/4 and 3/4 of the way (Chaikin's algorithm)
        let q = pt2(
            p0.x * 0.75 + p1.x * 0.25,
            p0.y * 0.75 + p1.y * 0.25
        );
        
        let r = pt2(
            p0.x * 0.25 + p1.x * 0.75,
            p0.y * 0.25 + p1.y * 0.75
        );
        
        // For first point, keep the first point
        if i == 0 {
            result.push(p0);
        }
        
        result.push(q);
        result.push(r);
        
        // For last segment, keep the last point
        if i == points.len() - 2 {
            result.push(p1);
        }
    }
    
    result
}