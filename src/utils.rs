//-------------------------------------------------------------------------
// @file utils.rs
//
// @date 06/14/21 21:44:52
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
use sfml::system::Vector2f;
// TODO(elsuizo:2021-06-14): capaz que vamos a tener que hacer que los argumentos de las funciones
// sean borrowed...

fn distance_squared(vec1: Vector2f, vec2: Vector2f) -> f32 {
    (vec2.x - vec1.x) * (vec2.x - vec1.x) + (vec2.y - vec1.y) * (vec2.y - vec1.y)
}

fn distance(v1: Vector2f, v2: Vector2f) -> f32 {
    f32::sqrt(distance_squared(v1, v2))
}
