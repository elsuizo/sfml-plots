// TODO(elsuizo:2019-09-20):
// - [X] Sistema coordenadas(dos RectangleShape que se crucen para empezar)
//  - [X] Faltario agregarle algunas opciones cuando se lo crea
// - [X] Sistemo coordenado(con tiks periodicos)
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

// TODO(elsuizo:2019-09-21): por ahora son constantes globales
const N: u32 = 10;
const DELTA_X: f32 = (WIDTH / N) as f32;
const DELTA_Y: f32 = (HEIGHT / N) as f32;
#[derive(Debug)]
struct AxisTick<'a> {
    shape: RectangleShape<'a>,
    color: Color,
    position: Vector2f,
}

struct Axis<'a> {
    shape: RectangleShape<'a>,
    color: Color,
    ticks: Option<TickType>,
    t: Vec<AxisTick<'a>>,
}

#[derive(Clone, Copy)]
enum TickType {
    TickWithNumber,
    TickOnly,
}

enum AxisTypes {
    XAxeType,
    YAxeType,
}

impl<'b> AxisTick<'b> {
    pub fn new(c: &Color, position: &Vector2f) -> Self {
        let mut s = RectangleShape::new();
        s.set_size(Vector2f::new(3.0, 3.0));
        s.set_fill_color(c);
        s.set_position(*position);

        Self {
            shape: s,
            color: *c,
            position: *position,
        }
    }
}

// NOTE(elsuizo:2019-09-21): es mejor llamar a el type con su alias Self, porque si le cambiamos el
// nombre no tenemos que cambiar nada
impl<'a> Axis<'a> {
    pub fn new(c: &Color, axis_type: AxisTypes, ticks: Option<TickType>) -> Self {
        let mut s = RectangleShape::new();
        let mut internal_tick: Vec<AxisTick> = Vec::new();
        match axis_type {
            AxisTypes::XAxeType => {
                let origin = Vector2f::new(0.0, (HEIGHT / 2) as f32); // fix position
                s.set_size(Vector2f::new(WIDTH as f32, 2.0));
                s.set_fill_color(c);
                s.set_position(origin);
                if let Some(t) = ticks {
                    internal_tick = Self::draw_ticks_x(t);
                }
            }
            AxisTypes::YAxeType => {
                let origin = Vector2f::new((WIDTH / 2) as f32, 0.0); // fix position
                s.set_size(Vector2f::new(2.0, HEIGHT as f32));
                s.set_fill_color(&c);
                s.set_position(origin);
                if let Some(t) = ticks {
                    internal_tick = Self::draw_ticks_y(t);
                }
            }
        }
        // s.set_outline_thickness(3.0); // no se si va
        // s.set_outline_color(&Color::BLACK);
        Self {
            shape: s,
            color: *c,
            ticks: ticks,
            t: internal_tick,
        }
    }
}

impl<'a> Axis<'a> {
    fn draw_ticks_x(t: TickType) -> Vec<AxisTick<'a>> {
        let mut vec_ticks: Vec<AxisTick> = Vec::new();
        match t {
            TickType::TickOnly => {
                let tick_color = Color::RED;
                for num_ticks in (0..WIDTH).step_by(DELTA_X as usize) {
                    let mut position = Vector2f::new(num_ticks as f32, (HEIGHT / 2) as f32);
                    let t = AxisTick::new(&tick_color, &position);
                    vec_ticks.push(t);
                }
            }
            TickType::TickWithNumber => {
                unimplemented!();
            }
        }
        vec_ticks
    }
    fn draw_ticks_y(t: TickType) -> Vec<AxisTick<'a>> {
        let mut vec_ticks: Vec<AxisTick> = Vec::new();
        match t {
            TickType::TickOnly => {
                let tick_color = Color::RED;
                for num_ticks in (0..HEIGHT).step_by(DELTA_Y as usize) {
                    let mut position = Vector2f::new((WIDTH / 2) as f32, num_ticks as f32);
                    let mut t = AxisTick::new(&tick_color, &position);
                    vec_ticks.push(t);
                }
            }
            TickType::TickWithNumber => {
                unimplemented!();
            }
        }
        vec_ticks
    }
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
    let background_color = &Color::rgb(120, 100, 100);
    // let background_color = &Color::WHITE;
    let x_axis_color = Color::BLACK;
    let y_axis_color = Color::BLACK;
    let x_axis = Axis::new(&x_axis_color, AxisTypes::XAxeType, Some(TickType::TickOnly));
    let y_axis = Axis::new(&y_axis_color, AxisTypes::YAxeType, Some(TickType::TickOnly));
    // println!("ticks: {:?}", x_axis.t);
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
        for tick in &x_axis.t {
            window.draw(&tick.shape);
        }
        for tick in &y_axis.t {
            window.draw(&tick.shape);
        }
        window.display();
    }
}
