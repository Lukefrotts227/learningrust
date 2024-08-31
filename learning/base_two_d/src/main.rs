extern crate piston_window;

use piston_window::*;

fn main() {
    // Create a Piston window
    let mut window: PistonWindow = WindowSettings::new(
            "Piston: Draw a Square",
            [400, 400] // Window size: 400x400 pixels
        )
        .exit_on_esc(true) // Close the window when the escape key is pressed
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _| {
            // Clear the screen with a white background
            clear([1.0, 1.0, 1.0, 1.0], graphics);

            // Define the color of the square (red)
            let square_color = [1.0, 0.0, 0.0, 1.0]; // RGBA

            // Define the position and size of the square
            let square = [150.0, 150.0, 100.0, 100.0]; // x, y, width, height

            // Draw the square
            rectangle(square_color, square, context.transform, graphics);
        });
    }
}
    