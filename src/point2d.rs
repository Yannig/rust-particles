use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl ops::Add<Point2d> for Point2d {
    type Output = Point2d;
    fn add(self, rhs: Point2d) -> Point2d {
        Point2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<&Point2d> for Point2d {
    type Output = Point2d;
    fn add(self, rhs: &Point2d) -> Point2d {
        Point2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Point2d> for Point2d {
    type Output = Point2d;
    fn sub(self, rhs: Point2d) -> Point2d {
        Point2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::Sub<&Point2d> for Point2d {
    type Output = Point2d;
    fn sub(self, rhs: &Point2d) -> Point2d {
        Point2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::Sub<&Point2d> for &Point2d {
    type Output = Point2d;
    fn sub(self, rhs: &Point2d) -> Point2d {
        Point2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::AddAssign<&Point2d> for Point2d {
    fn add_assign(&mut self, rhs: &Point2d) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl ops::Div<f64> for Point2d {
    type Output = Point2d;
    fn div(self, rhs: f64) -> Point2d {
        Point2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Point2d {
    pub const fn new(x: f64, y: f64) -> Point2d {
        Point2d { x, y }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let _x = self.x as i32;
        let _y = self.y as i32;
        canvas.draw_point(Point::new(_x, _y))?;
        canvas.draw_line(Point::new(_x - 2, _y - 2), Point::new(_x + 2, _y + 2))?;
        canvas.draw_line(Point::new(_x + 2, _y - 2), Point::new(_x - 2, _y + 2))
    }
}
