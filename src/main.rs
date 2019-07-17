/// Intento de generar un sistema de coordenadas con sfml para graficar funciones

//-------------------------------------------------------------------------
//                        crates imports
//-------------------------------------------------------------------------
extern crate sfml;

//-------------------------------------------------------------------------
//                        includes
//-------------------------------------------------------------------------
use sfml::graphics::{CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape,
                     Text, Transformable};
use sfml::system::{Clock, Time, Vector2f, Vector2u};
use sfml::window::{VideoMode, ContextSettings, Event, Key, Style};


fn main() {
    // Define some constants
    let side: u32 = 700;
    let width: u32 = side;
    let height: u32 = side;
    // Create the window of the application
    let fg_color = &Color::rgb(255, 255, 255);
    let mut window = RenderWindow::new(VideoMode::new(width, height, 32),
                                       "Rosette",
                                       Style::CLOSE,
                                       &Default::default());
    window.set_vertical_sync_enabled(true);
    let k = side as f32 / 320E+9;
    let radius = 20.0;
    let dt = 1E-4;
    let mut counter:i32 = 0;
    let bg_color = &Color::rgb(0, 0, 0);
    loop {
        for event in window.wait_event() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => return,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        // rosette.shape.move2f(350.0 + pos.0, 350.0 + pos.1);
        window.clear(bg_color);

        // window.draw(&obj);
        window.display();
    }
}
