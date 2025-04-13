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
- Visualization of control points as small circles
- Step-by-step animation of Chaikin's algorithm (7 steps)
- Automatic animation restart after completion
- Special case handling:
  - Single point: Just displays the point
  - Two points: Draws a straight line
  - Three or more points: Applies Chaikin's algorithm

## Controls

- **Left Mouse Button**: Place control points on the canvas
- **Enter**: Start the animation (if points are present)
- **Escape**: Quit the application

## Requirements

- [List of dependencies/libraries]
- [Minimum system requirements]

## Setup and Installation

1. Clone this repository:
   ```
   git clone [repository-url]
   ```

2. Install dependencies:
   ```
   [dependency installation commands]
   ```

3. Run the application:
   ```
   [command to run the application]
   ```

## Implementation Details

The application is structured into three main components:

1. **UI and Input Handling**: Manages the canvas, user input, and point visualization
2. **Chaikin's Algorithm**: Implements the core curve subdivision logic
3. **Animation System**: Controls the step-by-step visualization and timing

## Development

This project was developed as a 24-hour challenge by a team of three developers, each focusing on a different aspect of the application to enable parallel development.

## License

[License information]

## Acknowledgments

- [Any references or resources used]