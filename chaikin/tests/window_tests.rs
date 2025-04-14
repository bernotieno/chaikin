use nannou::prelude::*;
use chaikin::model::Model;
use chaikin::window;

#[test]
fn test_model_creation() {
     
    struct MockApp {
        windows: Vec<String>,
    }
    
    impl MockApp {
        fn new() -> Self {
            MockApp { windows: Vec::new() }
        }
        
        fn new_window(&mut self) -> MockWindowBuilder {
            MockWindowBuilder { app: self }
        }
    }
    
    struct MockWindowBuilder<'a> {
        app: &'a mut MockApp,
    }
    
    impl<'a> MockWindowBuilder<'a> {
        fn size(self, _width: u32, _height: u32) -> Self { self }
        fn view(self, _view_fn: fn(&App, &Model, Frame)) -> Self { self }
        fn mouse_pressed(self, _mouse_pressed_fn: fn(&App, &mut Model, MouseButton)) -> Self { self }
        fn mouse_released(self, _mouse_released_fn: fn(&App, &mut Model, MouseButton)) -> Self { self }
        fn mouse_moved(self, _mouse_moved_fn: fn(&App, &mut Model, Point2)) -> Self { self }
        fn key_pressed(self, _key_pressed_fn: fn(&App, &mut Model, Key)) -> Self { self }
        
        fn build(self) -> Result<WindowId, String> {
            self.app.windows.push("test_window".to_string());
            Ok(WindowId::new(0))
        }
    }
    
    let mut mock_app = MockApp::new();
    
    
    // Simulate window creation
    let _ = mock_app.new_window()
        .size(800, 600)
        .view(|_, _, _| {})
        .mouse_pressed(|_, _, _| {})
        .mouse_released(|_, _, _| {})
        .mouse_moved(|_, _, _| {})
        .key_pressed(|_, _, _| {})
        .build();
    
    // Verify a window was created
    assert_eq!(mock_app.windows.len(), 1);
    
    // Test that Model::new() works
    let model = Model::new();
    assert!(model.message.is_empty());
}

#[test]
fn test_update_message_timeout() {
    struct MockApp {
        time: f32,
    }
    
    impl MockApp {
        fn new(time: f32) -> Self {
            MockApp { time }
        }
    }
    
    // Create a model with a message
    let mut model = Model::new();
    model.message = "Test message".to_string();
    model.message_time = 0.0;
    
    // Test that message persists when time < 2.0 seconds
    let app = MockApp::new(1.5);
    window::update(&app, &mut model, Update::new(0.0));
    assert_eq!(model.message, "Test message");
    
    // Test that message is cleared when time > 2.0 seconds
    let app = MockApp::new(2.5);
    window::update(&app, &mut model, Update::new(0.0));
    assert!(model.message.is_empty());
}

#[test]
fn test_animation_update() {
    struct MockApp {
        time: f32,
    }
    
    impl MockApp {
        fn new(time: f32) -> Self {
            MockApp { time }
        }
    }
    
    // Create a model with animation enabled
    let mut model = Model::new();
    model.animating = true;
    model.animation_frames = vec![vec![]];
    model.step = 0;
    model.last_animation_time = 0.0;
    
    // Test that step doesn't change when time < 0.8 seconds
    let app = MockApp::new(0.5);
    window::update(&app, &mut model, Update::new(0.0));
    assert_eq!(model.step, 0);
    
    // Test that step advances when time > 0.8 seconds
    let app = MockApp::new(1.0);
    window::update(&app, &mut model, Update::new(0.0));
    assert_eq!(model.step, 1);
    assert_eq!(model.last_animation_time, 1.0);
    
    // Test that step wraps around after reaching 7
    model.step = 7;
    window::update(&app, &mut model, Update::new(0.0));
    assert_eq!(model.step, 0); // Should wrap back to 0
}