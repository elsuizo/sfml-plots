//-------------------------------------------------------------------------
// @file line.rs
//
// @date 06/14/21 21:30:55
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
use crate::utils::*;
use sfml::graphics::RectangleShape;

pub struct Line<'a> {
    line_shape: RectangleShape,
}

impl<'a> Line<'a> {
    fn new(p1: Vector2f, p2: Vector2f) -> Self {
        let thickness = 2.0;
        let mut line_shape: RectangleShape = Default::default();
        line_shape.set_position(p1);
        line_shape.set_size(Vector2f::new(distance(p1, p2)), thickness);
        // TODO(elsuizo:2021-06-16): no se porque lo convierte a degrees en la implementacion
        // original
        line_shape.set_rotation(f32::atan2(p2.y - p1.y, p2.x - p1.x));
        line_shape.set_origin(Vector2f::new(0.0, thickness / 2.0));
        Self { line_shape }
    }
}
