// TODO(elsuizo:2019-09-20):
// - [ ] Sistema coordenadas(dos RectangleShape que se crucen para empezar)
// - [ ] Sistemo coordenado(con tiks periodicos)
// - [ ] Sistema coordenado(con tiks y numeros)
// - [ ] Sistema coordenado(con tiks y numeros y que grafique puntos)
//-------------------------------------------------------------------------
//                        crates imports
//-------------------------------------------------------------------------
extern crate sfml;

//-------------------------------------------------------------------------
//                        includes
//-------------------------------------------------------------------------
use sfml::graphics::{
    CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
    Transformable,
};
use sfml::system::{Clock, Time, Vector2f, Vector2u};
use sfml::window::{ContextSettings, Event, Key, Style, VideoMode};

// TODO(elsuizo:2019-09-20): por ahora estas son constantes globales, pero luego tendrian que ser
// un parametro del grafico
const WIDTH: u32 = 700; // anchura
const HEIGHT: u32 = 500; // altura

struct AxisTick<'a> {
    shape: RectangleShape<'a>,
    color: Color,
}

struct Axis<'a> {
    shape: RectangleShape<'a>,
    color: Color,
}

enum AxisTypes {
    XAxeType,
    YAxeType,
}

impl<'a> Axis<'a> {
    fn new(c: Color, axis_type: AxisTypes, ticks: Option<Vec<AxisTick>>) -> Self {
        let mut s = RectangleShape::new();
        match axis_type {
            AxisTypes::XAxeType => {
                let origin = Vector2f::new(0.0, (HEIGHT / 2) as f32); // fix position
                s.set_size(Vector2f::new(WIDTH as f32, 2.0));
                s.set_fill_color(&c);
                s.set_position(origin);
                if let Some(t) = ticks {
                    Self::draw_ticks(t);
                }
            }
            AxisTypes::YAxeType => {
                let origin = Vector2f::new((WIDTH / 2) as f32, 0.0); // fix position
                s.set_size(Vector2f::new(2.0, HEIGHT as f32));
                s.set_fill_color(&c);
                s.set_position(origin);
                if let Some(t) = ticks {
                    Self::draw_ticks(t);
                }
            }
        }
        // s.set_outline_thickness(3.0); // no se si va
        // s.set_outline_color(&Color::BLACK);
        Self { shape: s, color: c }
    }
}

impl<'a> Axis<'a> {
    fn draw_ticks(t: Vec<AxisTick>) {}
}

fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(
        VideoMode::new(WIDTH, HEIGHT, 32),
        "SFML plots",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let background_color = &Color::rgb(100, 100, 100);
    let x_axis_color = Color::BLACK;
    let y_axis_color = Color::BLACK;
    let mut x_axis = Axis::new(x_axis_color, AxisTypes::XAxeType, None);
    let mut y_axis = Axis::new(y_axis_color, AxisTypes::YAxeType, None);
    //-------------------------------------------------------------------------
    //                        loop
    //-------------------------------------------------------------------------
    loop {
        //-------------------------------------------------------------------------
        //                        event handling
        //-------------------------------------------------------------------------
        for event in window.wait_event() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => return,
                    _ => {}
                },
                _ => {}
            }
        }
        window.clear(background_color);
        window.draw(&x_axis.shape);
        window.draw(&y_axis.shape);
        window.display();
    }
}
