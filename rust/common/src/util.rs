use std::{hash::Hash, ops};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point(pub i64, pub i64);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Dir(pub i64, pub i64);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Interval(pub i64, pub i64);

impl_op!(+= |a: &mut Point, b: Dir| { a.0 += b.0; a.1 += b.1 });
impl_op!(+= |a: &mut Point, b: &Dir| { a.0 += b.0; a.1 += b.1 });
impl_op_ex!(+ |a: &Point, b: &Dir| -> Point { Point(a.0 + b.0, a.1 + b.1) });
impl_op_ex!(*|a: &Point, b: &i64| -> Point { Point(a.0 * b, a.1 * b) });
impl_op_ex!(*|a: &Dir, b: &i64| -> Dir { Dir(a.0 * b, a.1 * b) });
impl_op_ex!(-|a: &Point, b: &Point| -> Point { Point(a.0 - b.0, a.1 - b.1) });

impl Point {
    pub fn coords(&self) -> (i64, i64) {
        (self.0, self.1)
    }
    pub fn around(&self) -> Vec<Point> {
        vec![
            Point(self.0 + 1, self.1),
            Point(self.0 - 1, self.1),
            Point(self.0, self.1 + 1),
            Point(self.0, self.1 - 1),
            Point(self.0 + 1, self.1 + 1),
            Point(self.0 + 1, self.1 - 1),
            Point(self.0 - 1, self.1 + 1),
            Point(self.0 - 1, self.1 - 1),
        ]
    }
}
