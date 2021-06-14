//-------------------------------------------------------------------------
// @file graph.rs
//
// @date 06/13/21 14:15:55
// @author Martin Noblia
// @email mnoblia@disroot.org
//
// @brief
//
// @detail
//
//  Licence:
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
//--------------------------------------------------------------------------
// NOTE(elsuizo:2021-06-13): el hace un type abstracto Graph con dos implementaciones que son
// AnimatedGraph y StaticGraph que son para los que son animaciones y para los que son staticos
// creo que voy a hacer solo primero la que me interesa que es la que tiene animaciones y que
// aparte es la mas dificil. Por ahora lo hago como un simple struct pero despues podria hacer un
// trait general que sea Graph y que tengan las implementaciones para los dos casos
pub struct AnimatedGraph<F> {
    fun: F,
    points: HashMap<i32, Vector2f>,
    accuracy: i32,
    xval: f32,
    start_x: f32,
    speed: f32,
    clock: Clock,
    r_time: f32,
    color: Color,
    thickness: f32,
}

// TODO(elsuizo:2021-06-13): implementar Default y hacer el truco con ..default()
// ya que el chabon pone todos los parametros como default
impl<F: Fn(f32) -> f32> AnimatedGraph<F> {
    fn new(fun: F) -> Self {
        let thickness = 2.0;
        let color = Color::RED;
        let speed = 2.0;
        let start_x = 0.0;
        let points: HashMap<i32, Vector2f> = HashMap::new();
        let clock = Clock::start();
        let xval = 0.0;
        let accuracy = 1.0;
        let r_time = 0.0;
        Self {
            fun,
            points,
            accuracy,
            xval,
            start_x,
            speed,
            clock,
            r_time,
            color,
            thickness,
        }
    }

    fn advance(&mut self, ppu: f32) {
        // TODO(elsuizo:2021-06-13): aca estoy comparando dos floats asi nomas deberia usar alguna
        // de las funciones de static-math
        if self.xval == self.start_x {
            self.start_x *= ppu;
            self.xval    *= ppu;
            self.clock.restart();
            self.advance_x();
        }
        let mut elapsed_time = self.clock.elapsed_time().as_seconds() + self.r_time;
        // TODO(elsuizo:2021-06-13): ojo que esto puede estar mal y capaz que haya que poner algun
        // parentesis
        let interval = 1 / self.speed * self.accuracy / ppu;
        while elapsed_time >= interval {
            self.clock.restart();
            let result = self.fun(self.xval / ppu);
            // NOTE(elsuizo:2021-06-13): aca prueba si el valor de retorno de la funcion es
            // `nothing` o no (que en Julia parece que ese valor es el que simboliza a un `void` en
            // C) osea que podriamos hacer lo que hace el cuando no obtenemos un valor de la
            // funcion pero con un Option
            // NOTE(elsuizo:2021-06-13): aca tambien mira cual es el type de result para hacer dos
            // cosas diferentes con los valores
            let pos = Vector2f::new(self.xval, ppu * result); // esto es el punto que nos da la funcion basicamente
            // agregamos los puntos en el HashMap
            self.points.insert(xval, pos);
            self.advance_x();
            elapsed_time -= interval;
        }
        self.r_time = elapsed_time;
    }

    fn advance_x(&mut self) {
        self.xval += self.accuracy;
    }

    // NOTE(elsuizo:2021-06-13): esta funcion es lo mismo que hace arriba cuando espera una
    // exception
    fn add_point(&mut self, index: i32, ppu: f32) {
        let result = self.fun(index / ppu);
        let pos = Vector2f::new(index, ppu * result);
        self.points.insert(index, pos);
    }

    fn add_point_hash(&mut self, index: i32, pos: Vector2f) {
        self.points.insert(index, pos);
    }

    // TODO(elsuizo:2021-06-13): aca esta la magia de todo, lo que nos faltaria implementar es un
    // type Line que haga lo que hace el del chabon(que basicamente es un RectangleShape que le
    // calcula la pendiente y no se que mas para que sea una linea) otra cosa que hace es ordenar
    // los puntos del HashMap lo que podriamos hacer es probar con otra estructura de datos en la
    // que cuando se agreguen los puntos se hagan en orden
    fn draw(&self, window: RenderWindow) {
        let last_point = 0.0;
        let point
    }
}
