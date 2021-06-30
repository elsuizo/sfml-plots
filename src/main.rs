// TODO(elsuizo:2021-06-30):
// - [ ] Primero hacer andar la animacion de `Axis`
// - [ ] Agregar los numberos en los ticks
// - [ ] Ver como hace el supuesto zoom con el mouse
// - [ ] Ver como hace que el eje y se ponga como limite a la izquierda cuando ya esta avanzada la
// animacion
use sfml::graphics::{
    CircleShape, Color, Font, PrimitiveType, RectangleShape, RenderTarget, RenderWindow, Shape,
    Text, Transformable, Vertex, VertexArray, View,
};

use sfml::system::{Clock, Vector2f};
use sfml::window::{Event, Key, Style};

mod graph;
mod utils;

use graph::AnimatedGraph;
const WINDOW_WIDTH: f32 = 500.0;
const WINDOW_HEIGHT: f32 = 500.0;

//-------------------------------------------------------------------------
//                        types
//-------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Axis<'a> {
    axis_shape: RectangleShape<'a>,
    marks: Vec<RectangleShape<'a>>,
    // numbers: Vec<Text<'a>>, TODO
    x_axis: bool,
    // number: Text<'a>, TODO
    tic: RectangleShape<'a>,
    tic_period: usize,
}

// TODO(elsuizo:2021-06-09): parece que podemos generar un RectangleShape desde una Texture...
impl<'a> Axis<'a> {
    // TODO(elsuizo:2021-06-09): aca lo construyo como lo hizo el chabon despues voy viendo si se
    // necesitan mas parametros(porque parece que no podes elegir los colores ni nada)
    pub fn new(view: &View, x_axis: bool) -> Self {
        let mut axis_shape = RectangleShape::new();
        axis_shape.set_fill_color(Color::rgb(128, 128, 128));
        axis_shape.set_size(Vector2f::new(view.size().x, 2.0));
        axis_shape.set_origin(Vector2f::new(view.size().x / 2.0, 1.0));
        if !x_axis {
            axis_shape.set_rotation(90.0);
        }
        let marks: Vec<RectangleShape> = Vec::new();
        // TODO(elsuizo:2021-06-09): esto no es muy importante por ahora
        // let numbers: Vec<Text>         = Vec::new();
        let mut tic = RectangleShape::new();
        tic.set_fill_color(Color::rgb(128, 128, 128));
        tic.set_size(Vector2f::new(10.0, 1.0));
        tic.set_origin(Vector2f::new(view.size().x / 2.0, 1.0));
        // TODO(elsuizo:2021-06-09): el chabon harcodeo el tic_period a 5 parece
        Self {
            axis_shape,
            marks,
            x_axis,
            tic,
            tic_period: 5,
        }
    }

    // TODO(elsuizo:2021-06-10): esta funcion usa un parametro que no tenemos parece que es el ppu
    pub fn update(&mut self, window: &RenderWindow) {
        let view = window.view();
        let center = view.center();
        // TODO(elsuizo:2021-06-10): este numero lo harcodeo porque no se que es y no esta
        // disponible como lo estaba en la implementacion de Julia
        let ppu: usize = 10;

        if self.x_axis {
            self.axis_shape.set_position(Vector2f::new(center.x, 0.0));
        } else {
            self.axis_shape.set_position(Vector2f::new(0.0, center.y));
        }
        // TODO(elsuizo:2021-06-10): aca faltan los numbers
        self.marks.clear();
        // let tic = self.tic_period;
        if self.x_axis {
            // NOTE(elsuizo:2021-06-10): original
            // right = Int(round((center.x + get_size(view).x/2)/ppu))
            // left = Int(round((center.x - get_size(view).x/2)/ppu))
            let right = (center.x + view.size().x / 2.0) as usize;
            let mut left = (center.x - view.size().x / 2.0) as usize;
            // TODO(elsuizo:2021-06-10): que hara esto???
            while left % self.tic_period != 0 {
                left += 1;
            }
            for i in (left..right).step_by(self.tic_period) {
                // NOTE(elsuizo:2021-06-10): aca es cuando ocurren los ticks
                if i % self.tic_period == 0 {
                    // un clone no pasa nada
                    let mut tic = self.tic.clone();
                    // TODO(elsuizo:2021-06-10): no se si esto es en radianes o grados ojo eh
                    tic.set_rotation(90.0);
                    // NOTE(elsuizo:2021-06-10): aca van los numeros
                    if center.y < 0.0 {
                        // ypos = min(center.y - get_size(view).y/2, 0)
                        let ypos = f32::min(center.y - view.size().y / 2.0, 0.0);
                        tic.set_position(Vector2f::new((i * ppu) as f32, ypos))
                    } else {
                        let ypos = f32::max(center.y + view.size().y / 2.0, 0.0);
                        tic.set_position(Vector2f::new((i * ppu) as f32, ypos))
                    }
                    self.marks.push(tic);
                }
            }
        } else {
            // NOTE(elsuizo:2021-06-10): notar que aca no se si se esta haciendo bien el round
            // up = Int(round((center.y - get_size(view).y/2)/ppu))
            // down = Int(round((center.y + get_size(view).y/2)/ppu))
            let up = (center.y + view.size().y / 2.0) as usize;
            let mut down = (center.y - view.size().y / 2.0) as usize;
            // TODO(elsuizo:2021-06-10): aca el asume que los dos ejes tienen el mismo
            // tic_period(cosa que puede que no sea asi...)
            // NOTE(elsuizo:2021-06-10): otra cosa esto me parece medio raro habria que pensar
            // todo de nuevo como se hace esto porque no me gusta...
            while down % self.tic_period != 0 {
                down += 1;
            }
            for i in (down..up).step_by(self.tic_period) {
                let mut tic = self.tic.clone();
                if i % self.tic_period == 0 {
                    if center.x < 0.0 {
                        let xpos = f32::min(center.x + view.size().x / 2.0, 0.0);
                        tic.set_position(Vector2f::new(xpos, (i * ppu) as f32))
                    } else {
                        let xpos = f32::max(center.x - view.size().x / 2.0, 0.0);
                        tic.set_position(Vector2f::new((i * ppu) as f32, xpos))
                    }
                }
                self.marks.push(tic);
            }
        }
    }

    fn plot(&self, window: &mut RenderWindow) {
        window.draw(&self.axis_shape);
        for mark in &self.marks {
            window.draw(mark)
        }
    }
}

#[derive(Debug)]
struct PlotWindow<'a, F> {
    render_window: RenderWindow,
    graphs: Vec<AnimatedGraph<F>>,
    view: View,
    event: Option<Event>,
    last_mouse_pos: Vector2f,
    x_axis: Axis<'a>,
    y_axis: Axis<'a>,
    ppu: usize,
    // TODO(elsuizo:2021-06-12): aca el chabon pone un parametro que se llama task(me parece que es
    // para algo que tiene que ver con async ...)
}

// TODO(elsuizo:2021-06-12): me faltaria implementar los types AnimatedGraph y ver que hacer con
// Graph(creo que lo mejor es hacer un trait porque Graph es una struct abstract en Julia)
impl<'a, F: Fn(f32) -> f32> PlotWindow<'a, F> {
    fn new(render_window: RenderWindow) -> Self {
        let width = render_window.size().x as f32;
        let height = render_window.size().y as f32;
        let graphs: Vec<AnimatedGraph<F>> = Vec::new();
        let view = View::new(Vector2f::new(0.0, 0.0), Vector2f::new(width, -height));
        let x_axis = Axis::new(&view, true);
        let y_axis = Axis::new(&view, false);
        let last_mouse_pos = Vector2f::new(0.0, 0.0);

        Self {
            render_window,
            graphs,
            view: *view,
            event: None,
            last_mouse_pos,
            x_axis,
            y_axis,
            ppu: 20,
        }
    }
}

fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(
        (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
        "sfml-plots",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let background_color = Color::BLACK;

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::ESCAPE, ..
                } => return,
                Event::KeyPressed {
                    code: Key::SPACE, ..
                } => {}
                _ => {}
            }
        }
        window.display();
        window.clear(background_color);
    }
}
