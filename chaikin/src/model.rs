// model.rs
use nannou::prelude::*;

pub struct Point {
    pub position: Point2,
    pub selected: bool,
}

pub struct Model {
    pub points: Vec<Point>,
    pub animation_frames: Vec<Vec<Point2>>,
    pub animating: bool,
    pub step: usize,
    pub last_animation_time: f32,
    pub drag_index: Option<usize>,
    pub message: String,
    pub message_time: f32,
}

impl Model {
    pub fn new() -> Self {
        Model {
            points: Vec::new(),
            animation_frames: Vec::new(),
            animating: false,
            step: 0,
            last_animation_time: 0.0,
            drag_index: None,
            message: String::new(),
            message_time: 0.0,
        }
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.animation_frames.clear();
        self.animating = false;
        self.step = 0;
    }

    pub fn set_message(&mut self, msg: &str, current_time: f32) {
        self.message = String::from(msg);
        self.message_time = current_time;
    }
}