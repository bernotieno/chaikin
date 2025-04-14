pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    // Clear the background
    draw.background().color(WHITE);

    // Special cases
    if model.points.is_empty() {
        // Draw instructions
        draw.text("Click to place points, press Enter to animate")
            .color(GRAY)
            .font_size(15)
            .x(-300.0)
            .y(-250.0);
    } else if model.points.len() == 1 {
        // Draw single point
        draw_point(&draw, model.points[0].position, 5.0, RED);
    } else if model.points.len() == 2 && !model.animating {
        // Draw two points with a line between them
        draw_point(&draw, model.points[0].position, 5.0, RED);
        draw_point(&draw, model.points[1].position, 5.0, RED);
        draw.line()
            .start(model.points[0].position)
            .end(model.points[1].position)
            .color(GRAY)
            .stroke_weight(1.0);
    } else {
        // Draw original points
        for point in &model.points {
            draw_point(&draw, point.position, 5.0, if point.selected { BLUE } else { RED });
        }

        // Draw connecting lines between original points if not animating
        if !model.animating {
            for i in 0..model.points.len() - 1 {
                draw.line()
                    .start(model.points[i].position)
                    .end(model.points[i + 1].position)
                    .color(GRAY)
                    .stroke_weight(1.0);
            }
        }

        // Draw animation frames
        if model.animating && !model.animation_frames.is_empty() {
            let current_frame = &model.animation_frames[model.step.min(model.animation_frames.len() - 1)];
            
            // Draw lines connecting the points in the current frame
            if current_frame.len() > 1 {
                for i in 0..current_frame.len() - 1 {
                    draw.line()
                        .start(current_frame[i])
                        .end(current_frame[i + 1])
                        .color(BLUE)
                        .stroke_weight(2.0);
                }
            }
        }
    }

    // Draw message if any
    if !model.message.is_empty() {
        draw.text(&model.message)
            .color(RED)
            .font_size(20)
            .x(0.0)
            .y(-270.0);
    }

    // Draw step counter if animating
    if model.animating {
        draw.text(&format!("Step: {}", model.step))
            .color(BLACK)
            .font_size(15)
            .x(300.0)
            .y(-270.0);
    }

    // Draw instructions
    draw.text("Press Enter to animate | C to clear | Esc to quit")
        .color(GRAY)
        .font_size(15)
        .x(0.0)
        .y(270.0);

    draw.to_frame(app, &frame).unwrap();
}

pub fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if button != MouseButton::Left || model.animating {
        return;
    }

    let mouse_pos = app.mouse.position();
    
    // Check if clicking on an existing point (for dragging)
    for (i, point) in model.points.iter_mut().enumerate() {
        let distance = mouse_pos.distance(point.position);
        if distance < 10.0 {
            point.selected = true;
            model.drag_index = Some(i);
            return;
        }
    }
    
    // Add a new point if not dragging
    model.points.push(Point {
        position: mouse_pos,
        selected: false,
    });
}

pub fn mouse_released(_app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {
        if let Some(idx) = model.drag_index {
            if idx < model.points.len() {
                model.points[idx].selected = false;
            }
        }
        model.drag_index = None;
    }
}

pub fn mouse_moved(app: &App, model: &mut Model, pos: Vec2) {
    if let Some(idx) = model.drag_index {
        if idx < model.points.len() {
            model.points[idx].position = pos;
        }
    }
}
