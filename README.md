# Chaikin's Algorithm Animation

This application demonstrates Chaikin's curve subdivision algorithm through an interactive, step-by-step animation. Users can place control points on a canvas and watch as the algorithm transforms these points into a smooth curve over 7 iterations.

## What is Chaikin's Algorithm?

Chaikin's algorithm is a simple but effective curve subdivision technique that creates a smooth curve from a set of control points. It works by:

1. Taking a polyline (a series of connected line segments)
2. For each line segment, removing the endpoints and replacing them with points that are 1/4 and 3/4 of the way along the segment
3. Connecting these new points to form a new polyline
4. Repeating the process to achieve greater smoothness

With each iteration, the curve becomes smoother and approaches a quadratic B-spline.

## Features

- Interactive canvas for placing control points
- Visualization of control points as small circles (red for normal, blue for selected)
- Step-by-step animation of Chaikin's algorithm (7 steps)
- Point dragging functionality for adjusting control points
- Special case handling:
  - Single point: Displays a red dot
  - Two points: Draws a straight line between points
  - Three or more points: Applies Chaikin's algorithm

## Controls

- **Left Mouse Button**: Place or drag control points
- **Enter**: Start the animation (requires 3 or more points)
- **C**: Clear all points
- **Escape**: Quit the application

## Requirements

- Rust 2021 Edition or later
- nannou graphics library
- Linux with OpenGL support

## Setup and Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/chaikin.git
   cd chaikin
   ```

2. Build and run the application:
   ```bash
   cargo run
   ```

## Implementation Details

The application is structured into four main components:

1. **UI ([`ui.rs`](src/ui.rs))**: Handles rendering, user input, and point visualization
2. **Model ([`model.rs`](src/model.rs))**: Manages application state and data structures
3. **Chaikin's Algorithm ([`chaikin.rs`](src/chaikin.rs))**: Implements the curve subdivision logic
4. **Window Management ([`window.rs`](src/window.rs))**: Controls the application window and update loop

## Development

This project was developed as a 24-hour challenge by a team of three developers:
- Person 1: UI and Input Handling
- Person 2: Chaikin's Algorithm Implementation
- Person 3: Animation and Integration

## License

MIT License

## Acknowledgments

- [Chaikin's Algorithm Paper](https://www.cs.unc.edu/~dm/UNC/COMP258/LECTURES/Chaikins-Algorithm.pdf)
- [nannou Creative Coding Framework](https://nannou.cc/)