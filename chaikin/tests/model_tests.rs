use nannou::prelude::*;
use chaikin::model::{Model, Point};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let position = pt2(10.0, 20.0);
        let point = Point {
            position,
            selected: false,
        };
        
        assert_eq!(point.position.x, 10.0);
        assert_eq!(point.position.y, 20.0);
        assert_eq!(point.selected, false);
    }

    #[test]
    fn test_model_new() {
        let model = Model::new();
        
        assert!(model.points.is_empty());
        assert!(model.animation_frames.is_empty());
        assert_eq!(model.animating, false);
        assert_eq!(model.step, 0);
        assert_eq!(model.last_animation_time, 0.0);
        assert_eq!(model.drag_index, None);
        assert!(model.message.is_empty());
        assert_eq!(model.message_time, 0.0);
    }

    #[test]
    fn test_model_clear() {
        let mut model = Model::new();
        
        // Add some data to the model
        model.points.push(Point {
            position: pt2(10.0, 20.0),
            selected: false,
        });
        
        model.animation_frames.push(vec![pt2(10.0, 20.0)]);
        model.animating = true;
        model.step = 5;
        
        // Clear the model
        model.clear();
        
        // Verify everything is cleared
        assert!(model.points.is_empty());
        assert!(model.animation_frames.is_empty());
        assert_eq!(model.animating, false);
        assert_eq!(model.step, 0);
    }

    #[test]
    fn test_set_message() {
        let mut model = Model::new();
        let current_time = 123.45;
        
        model.set_message("Test message", current_time);
        
        assert_eq!(model.message, "Test message");
        assert_eq!(model.message_time, current_time);
    }

    #[test]
    fn test_drag_index() {
        let mut model = Model::new();
        
        // Initially None
        assert_eq!(model.drag_index, None);
        
        // Set to Some value
        model.drag_index = Some(3);
        assert_eq!(model.drag_index, Some(3));
        
        // Reset to None
        model.drag_index = None;
        assert_eq!(model.drag_index, None);
    }

    #[test]
    fn test_animation_state() {
        let mut model = Model::new();
        
        // Test initial state
        assert_eq!(model.animating, false);
        assert_eq!(model.step, 0);
        
        // Change animation state
        model.animating = true;
        model.step = 2;
        model.last_animation_time = 10.5;
        
        assert_eq!(model.animating, true);
        assert_eq!(model.step, 2);
        assert_eq!(model.last_animation_time, 10.5);
    }
}