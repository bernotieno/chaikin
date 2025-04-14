mod model;
mod window;
mod ui;
mod chaikin;

fn main() {
    nannou::app(window::model)
        .update(window::update)
        .run();
}