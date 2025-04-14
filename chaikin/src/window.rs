use nannou::prelude::*;
use crate::model::Model;
use crate::ui;

pub fn model(app: &App) -> Model {
    // Create a window
    app.new_window()
        .size(800, 600)
        .view(ui::view)
        .mouse_pressed(ui::mouse_pressed)
        .mouse_released(ui::mouse_released)
        .mouse_moved(ui::mouse_moved)
        .key_pressed(ui::key_pressed)
        .build()
        .unwrap();

    Model::new()
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    // Update message display time
    if !model.message.is_empty() {
        if app.time - model.message_time > 2.0 {
            model.message.clear();
        }
    }

    // Update animation
    if model.animating && !model.animation_frames.is_empty() {
        // Advance animation step every 0.8 seconds
        if app.time - model.last_animation_time > 0.8 {
            model.step = (model.step + 1) % 8; // 0-7, then restart
            model.last_animation_time = app.time;
        }
    }
}