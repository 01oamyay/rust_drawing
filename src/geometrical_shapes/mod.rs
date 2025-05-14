use rand::Rng;
use raster::{Color, Image};
use std::cmp::max;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color() -> Color {
        let mut rng = rand::rng();
        let c: Color = loop {
            let r: u8 = rng.random_range(0..=255);
            let g: u8 = rng.random_range(0..=255);
            let b: u8 = rng.random_range(0..=255);

            if r != 0 || g != 0 || b != 0 {
                break Color { r, g, b, a: 255 }; // Only return if it's not black
            }
        };
        c
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Copy, Clone, Debug)]
pub struct Point(i32, i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();

        let x: i32 = rng.random_range(1..width);
        let y: i32 = rng.random_range(1..height);

        Self::new(x, y)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.0, self.1, Self::color());
    }
}

#[derive(Debug)]
pub struct Line(pub Point, pub Point, pub Color);

impl Line {
    pub fn new(p1: &Point, p2: &Point, color: Color) -> Self {
        Self(*p1, *p2, color)
    }

    pub fn random(width: i32, height: i32) -> Self {
        let p1: Point = Point::random(width, height);
        let p2: Point = Point::random(width, height);

        Self::new(&p1, &p2, Self::color())
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let color: Color = Color {
            r: self.2.r,
            g: self.2.g,
            b: self.2.b,
            a: self.2.a,
        };

        let Point(x_1, y_1) = self.0;
        let Point(x_2, y_2) = self.1;

        let x_distance = x_2 - x_1;
        let y_distance = y_2 - y_1;

        let steps = max(x_distance.abs(), y_distance.abs());

        let x_increment = x_distance as f32 / steps as f32;
        let y_increment = y_distance as f32 / steps as f32;
        println!("{self:?}, steps: {steps}, {x_increment}, {y_increment}");

        let mut x = x_1 as f32;
        let mut y = y_1 as f32;

        for _ in 0..=steps {
            image.display(
                x.round() as i32,
                y.round() as i32,
                Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                },
            );

            x += x_increment;
            y += y_increment;
        }
    }
}

pub struct Rectangle(pub Point, pub Point);

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(*p1, *p2)
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let Point(x_1, y_1) = self.0;
        let Point(x_2, y_2) = self.1;

        let corner_1: Point = Point::new(x_1, y_1);
        let corner_2: Point = Point::new(x_1, y_2);
        let corner_3: Point = Point::new(x_2, y_2);
        let corner_4: Point = Point::new(x_2, y_1);

        let color: Color = Self::color();

        println!("rectangle: {:?}", color);

        let line_hb: Line = Line::new(
            &corner_1,
            &corner_4,
            Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            },
        );
        let line_vr = Line::new(
            &corner_4,
            &corner_3,
            Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            },
        );
        let line_ht = Line::new(
            &corner_3,
            &corner_2,
            Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            },
        );
        let line_vl = Line::new(
            &corner_2,
            &corner_1,
            Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            },
        );

        line_hb.draw(image);
        line_vr.draw(image);
        line_ht.draw(image);
        line_vl.draw(image);
    }
}

pub struct Triangle(pub Point, pub Point, pub Point);

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self(*p1, *p2, *p3)
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let point_1 = self.0;
        let point_2 = self.1;
        let point_3 = self.2;

        let color = Self::color();

        println!("triangle: {:?}", color);

        let line_1: Line = Line::new(
            &point_1,
            &point_2,
            Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            },
        );
        let line_2: Line = Line::new(
            &point_2,
            &point_3,
            Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            },
        );
        let line_3: Line = Line::new(
            &point_3,
            &point_1,
            Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            },
        );

        line_1.draw(image);
        line_2.draw(image);
        line_3.draw(image);
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self {
            center: *center,
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let center: Point = Point::random(width, height);
        let mut rng = rand::rng();
        let radius: i32 = rng.random_range(1..=height / 2);
        Self::new(&center, radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let Point(cx, cy) = self.center;

        let color = Self::color();

        let mut x = 0;
        let mut y = -self.radius;
        while x < -y {
            if (x * x) as f32 + (y as f32 + 0.5) * (y as f32 + 0.5)
                > (self.radius * &self.radius) as f32
            {
                y += 1
            }
            image.display(
                cx + x,
                cy + y,
                Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                },
            );

            image.display(
                cx - x,
                cy + y,
                Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                },
            );

            image.display(
                cx + x,
                cy - y,
                Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                },
            );

            image.display(
                cx - x,
                cy - y,
                Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                },
            );

            image.display(
                cx + y,
                cy + x,
                Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                },
            );

            image.display(
                cx - y,
                cy + x,
                Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                },
            );

            image.display(
                cx + y,
                cy - x,
                Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                },
            );

            image.display(
                cx - y,
                cy - x,
                Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                },
            );

            x += 1
        }
    }
}
