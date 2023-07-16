// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::f64::consts::PI;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn magnitude(&self) -> f64 {
        // my solution
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
        // solution proposed (more readable)
        //f64::from(self.x.pow(2) + self.y.pow(2)).sqrt()
    }

    fn dist(self, p2: Point) -> f64 {
        // my solution
        (((self.x - p2.x).pow(2) + (self.y - p2.y).pow(2)) as f64).sqrt()
        // solution proposed (code reusing but need to implement Sub)
        //(self - p2).magnitude()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Self {
        Self { points: vec!() }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point)
    }

    fn left_most_point(&self) -> Option<Point> {
        // my dirty workaround to make it work
        Some(*self.points.iter().min_by_key(|p| p.x).unwrap())
        // proposed solution (nice and easy)
        //self.points.iter().min_by_key(|p| p.x).copied()
    }

    // my solution
    fn iter(&self) -> std::slice::Iter<'_, Point> {
        self.points.iter()
    }
    // proposed solution
    //pub fn iter(&self) -> impl Iterator<Item = &Point> {

    fn perimeter(&self) -> f64 {
        let mut perimeter = 0.00;
        let mut previous_point = self.points.first().unwrap();

        // calculate distance between subsequent points
        for point in &self.points[1..] {
            perimeter += previous_point.dist(*point);
            previous_point = point;
        }

        // add distance between first point and last point
        perimeter += previous_point.dist(*self.points.first().unwrap());

        perimeter
    }

}

pub struct Circle {
    point: Point,
    radius: i32
}

impl Circle {
    fn new(point: Point, radius: i32) -> Self {
        Self { point, radius}
    }

    fn circumference(&self) -> f64 {
        f64::from(self.radius) * 2.0 * PI
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Shape::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(poly) => poly.perimeter(),
            Shape::Circle(circle) => circle.circumference()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}