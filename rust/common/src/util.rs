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
impl_op_ex!(-|a: &Point, b: &Point| -> Dir { Dir(a.0 - b.0, a.1 - b.1) });
impl_op_ex!(-|a: &Dir| -> Dir { Dir(-a.0, -a.1) });

impl Dir {
    pub fn norm(&self) -> Dir {
        Dir(self.0.signum(), self.1.signum())
    }
}

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
    pub fn saround(&self) -> Vec<Point> {
        vec![
            Point(self.0 + 1, self.1),
            Point(self.0 - 1, self.1),
            Point(self.0, self.1 + 1),
            Point(self.0, self.1 - 1),
        ]
    }
    pub fn dir(&self) -> Dir {
        Dir(self.0, self.1)
    }
}

impl From<(i64, i64)> for Point {
    fn from(p: (i64, i64)) -> Self {
        Point(p.0, p.1)
    }
}

impl From<Point> for Dir {
    fn from(p: Point) -> Self {
        Dir(p.0, p.1)
    }
}
